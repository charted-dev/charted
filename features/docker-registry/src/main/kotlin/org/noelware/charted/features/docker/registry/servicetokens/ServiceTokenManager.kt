/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.features.docker.registry.servicetokens

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.auth0.jwt.exceptions.TokenExpiredException
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.lettuce.core.SetArgs
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.Clock
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.intOrNull
import kotlinx.serialization.json.jsonPrimitive
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.extensions.measureSuspendTime
import org.noelware.charted.common.extensions.measureTime
import java.io.Closeable
import java.time.Instant
import java.util.*
import kotlin.time.Duration.Companion.days
import kotlin.time.DurationUnit
import kotlin.time.toDuration

class ServiceTokenManager(config: Config, private val redis: IRedisClient, private val json: Json): Closeable {
    private val algorithm = Algorithm.HMAC512(config.jwtSecretKey.toByteArray())
    private val expirationJobs = mutableMapOf<Long, Job>()
    private val log by logging<ServiceTokenManager>()

    init {
        log.info("Collecting registry service tokens from Redis!")
        val tokens = log.measureSuspendTime("Took %T to collect all service tokens") {
            redis.commands.hgetall(REDIS_KEY).await()
        }

        val sw = StopWatch.createStarted()
        for ((key, value) in tokens) {
            if (sw.isStopped) {
                sw.reset()
                sw.start()
            }

            // key => user id
            log.debug("Collecting TTL for service token [$key]")
            val ttl = runBlocking { redis.commands.ttl(key).await() }
            if (ttl == -2L) continue
            if (ttl == -1L) {
                log.debug("Service token [$key] expires! Deleting from Redis...")
                log.measureSuspendTime("Took %T to delete service token") {
                    revoke(json.decodeFromString(value))
                }
            } else {
                log.debug("Service token [$key] expires in $ttl seconds.")
                expirationJobs[key.toLong()] = ChartedScope.launch {
                    delay(ttl.toDuration(DurationUnit.SECONDS).inWholeMilliseconds)
                    revoke(json.decodeFromString(value))
                }
            }
        }
    }

    override fun close() {
        log.warn("Closing off ${expirationJobs.size} registry service tokens...")
        log.measureTime("Took %T to close off ${expirationJobs.size} registry service tokens!") {
            for (job in expirationJobs.values) job.cancel()
        }
    }

    fun isExpired(token: String): Boolean = try {
        val verifier = JWT.require(algorithm)
            .withIssuer("Noelware/charted-server")
            .build()

        val jwt = verifier.verify(token)
        val payload = json.decodeFromString<JsonObject>(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))

        payload["exp"]?.jsonPrimitive?.intOrNull?.ifNotNull { this >= Clock.System.now().epochSeconds } ?: false
    } catch (e: TokenExpiredException) {
        true
    } catch (e: Exception) {
        throw e
    }

    suspend fun getServiceToken(token: String): DockerRegistryServiceToken? {
        if (token.isEmpty()) return null

        val verifier = JWT.require(algorithm)
            .withIssuer("Noelware/charted-server")
            .build()

        val jwt = verifier.verify(token)
        val payload = json.decodeFromString<JsonObject>(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))
        val headers = json.decodeFromString<JsonObject>(String(Base64.getDecoder().decode(jwt.header.toByteArray())))

        val issuer = payload["iss"]?.jsonPrimitive?.content
        if (issuer == null || issuer != "Noelware/charted-server") {
            throw IllegalArgumentException("Malformed JWT token.")
        }

        val userID = headers["user_id"]?.jsonPrimitive?.content
            ?: throw IllegalArgumentException("Malformed JWT token.")

        return redis.commands.hget(REDIS_KEY, userID).await().ifNotNull {
            json.decodeFromString(this)
        }
    }

    suspend fun createServiceToken(userID: Long): DockerRegistryServiceToken {
        val existingToken = redis.commands.hget(REDIS_KEY, userID.toString()).await().ifNotNull { json.decodeFromString<DockerRegistryServiceToken>(this) }
        if (existingToken != null) return existingToken

        // Create a token that should last around ~2 days or so, the /v2/token endpoint
        // will recreate the service token if it expires.
        val token = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(Instant.now().plusMillis(2.days.inWholeMilliseconds))
            .withHeader(
                mapOf(
                    "user_id" to userID.toString()
                )
            )
            .sign(algorithm)

        val serviceToken = DockerRegistryServiceToken(userID, token)
        val encoded = json.encodeToString(serviceToken)

        redis.commands.set(userID.toString(), "<just here for expiration>", SetArgs().ex(2.days.inWholeSeconds)).await()
        redis.commands.hmset(
            REDIS_KEY,
            mapOf(
                userID.toString() to encoded
            )
        ).await()

        expirationJobs[userID] = ChartedScope.launch {
            delay(2.days.inWholeMilliseconds)
            revoke(serviceToken)
        }

        return serviceToken
    }

    suspend fun revokeAll(userID: Long) {
        val tokens = redis.commands.hgetall(REDIS_KEY).await()
            .mapValues { json.decodeFromString<DockerRegistryServiceToken>(it.value) }
            .filter { it.value.userID == userID }

        for (token in tokens.values) revoke(token)
    }

    suspend fun revoke(token: DockerRegistryServiceToken) {
        redis.commands.del(token.userID.toString()).await()
        redis.commands.hdel(REDIS_KEY, token.userID.toString()).await()

        if (expirationJobs.containsKey(token.userID)) {
            val job = expirationJobs[token.userID]!!
            job.cancel()

            expirationJobs.remove(token.userID)
        }
    }

    suspend fun refresh(token: DockerRegistryServiceToken): DockerRegistryServiceToken {
        revoke(token)
        return createServiceToken(token.userID)
    }

    companion object {
        private const val REDIS_KEY = "charted:registry:service_tokens"
    }
}
