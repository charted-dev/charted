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

package org.noelware.charted.modules.docker.registry.tokens

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.auth0.jwt.exceptions.TokenExpiredException
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.lettuce.core.SetArgs
import io.sentry.Sentry
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
import org.noelware.charted.ChartedScope
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.charted.modules.redis.RedisClient
import java.io.Closeable
import java.time.Instant
import java.util.Base64
import kotlin.time.Duration.Companion.days
import kotlin.time.DurationUnit
import kotlin.time.toDuration

private const val REDIS_KEY = "charted:registry:service_tokens"

/**
 * Represents a manager for handling registry service token's expiration dates. This requires Redis
 * to be configured.
 */
class RegistryServiceTokenManager(
    private val redis: RedisClient,
    private val json: Json,
    config: Config
): Closeable {
    private val expirationJobs: MutableMap<Long, Job> = mutableMapOf()
    private val algorithm: Algorithm = Algorithm.HMAC512(config.jwtSecretKey)
    private val log by logging<RegistryServiceTokenManager>()

    init {
        val sw = StopWatch.createStarted()
        log.info("Collecting Docker registry service tokens from Redis...")

        val tokens = runBlocking { redis.commands.hgetall(REDIS_KEY).await() }
        sw.stop()

        log.info("Took ${sw.doFormatTime()} to collect ${tokens.size} service tokens!")
        for ((key, value) in tokens) {
            if (sw.isStopped) {
                sw.reset()
                sw.start()
            }

            // key => user id
            log.debug("Collecting TTL for registry service token [$key]")
            val ttl = runBlocking { redis.commands.ttl(key).await() }
            if (ttl == -2L) continue
            if (ttl == -1L) {
                log.warn("Registry service token [$key] has expired! Deleting from Redis...")
                runBlocking { revoke(json.decodeFromString(value)) }
            } else {
                log.debug("Registry service token [$key] expires in $ttl seconds!")
                expirationJobs[key.toLong()] = ChartedScope.launch {
                    delay(ttl.toDuration(DurationUnit.SECONDS).inWholeMilliseconds)
                    revoke(json.decodeFromString(value))
                }
            }
        }
    }

    /**
     * Returns a [Boolean] if the [token] specified is expired or not.
     * @param token The token to check if it's expired or not.
     */
    fun isTokenExpired(token: String): Boolean = try {
        val verifier = JWT.require(algorithm)
            .withIssuer("Noelware/charted")
            .build()

        val jwt = verifier.verify(token)
        val payload = json.decodeFromString<JsonObject>(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))
        payload["exp"]?.jsonPrimitive?.intOrNull?.ifNotNull { this >= Clock.System.now().epochSeconds } ?: true
    } catch (_: TokenExpiredException) {
        true
    } catch (e: Exception) {
        ifSentryEnabled { Sentry.captureException(e) }
        throw e
    }

    /**
     * Returns a [RegistryServiceToken] on a specified [token], or null if it was
     * expired or not found in Redis.
     *
     * @param token The token itself
     * @return The [registry service token][RegistryServiceToken] or null if the token was
     * expired or not found in Redis.
     */
    suspend fun getToken(token: String): RegistryServiceToken? {
        if (token.isEmpty()) return null
        if (isTokenExpired(token)) return null

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

    /**
     * Creates a registry service token for a user, or returns the [registry service token][RegistryServiceToken] if
     * there is one in Redis.
     *
     * @param userID The user ID that this token belongs to
     */
    suspend fun createServiceToken(userID: Long): RegistryServiceToken {
        val existingToken: RegistryServiceToken? = redis.commands.hget(REDIS_KEY, userID.toString()).await().ifNotNull { json.decodeFromString(this) }
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

        val serviceToken = RegistryServiceToken(userID, token)
        val encoded = json.encodeToString(serviceToken)

        redis.commands.set(userID.toString(), "<just here for expiration, nothing fancy here!>", SetArgs().ex(2.days.inWholeSeconds)).await()
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

    /**
     * Revokes all the tokens made by the [user][userID].
     * @param userID The user's ID
     */
    suspend fun revokeAll(userID: Long) {
        val tokens = redis.commands.hgetall(REDIS_KEY).await()
            .mapValues { json.decodeFromString<RegistryServiceToken>(it.value) }
            .filter { it.value.userID == userID }

        for (token in tokens.values) revoke(token)
    }

    /**
     * Revokes a [registry service token][RegistryServiceToken] from Redis and the expiration
     * jobs table, if it exists.
     *
     * @param token The registry service token to revoke
     */
    suspend fun revoke(token: RegistryServiceToken) {
        redis.commands.del(token.userID.toString()).await()
        redis.commands.hdel(REDIS_KEY, token.userID.toString()).await()

        if (expirationJobs.containsKey(token.userID)) {
            val job = expirationJobs[token.userID]!!
            job.cancel()

            expirationJobs.remove(token.userID)
        }
    }

    /**
     * Refreshes a [registry service token][RegistryServiceToken] for whatever reason.
     */
    suspend fun refresh(token: RegistryServiceToken): RegistryServiceToken {
        revoke(token)
        return createServiceToken(token.userID)
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * As noted in [AutoCloseable.close], cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * *mark* the `Closeable` as closed, prior to throwing
     * the `IOException`.
     *
     * @throws java.io.IOException if an I/O error occurs
     */
    override fun close() {
        for (job in expirationJobs.values) {
            job.cancel()
        }
    }
}
