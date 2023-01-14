/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.modules.docker.registry.authorization

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.lettuce.core.SetArgs
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.ChartedScope
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.SessionManager
import java.time.Instant
import java.util.Base64
import kotlin.time.Duration.Companion.days
import kotlin.time.DurationUnit
import kotlin.time.toDuration
import kotlin.time.toJavaDuration

class DefaultRegistryAuthorizationPolicyManager(
    private val sessionManager: SessionManager,
    private val redis: RedisClient,
    private val json: Json,
    config: Config
) : RegistryAuthorizationPolicyManager {
    private val algorithm: Algorithm = Algorithm.HMAC512(config.jwtSecretKey)
    private val expirationJobs = mutableListOf<Job>()
    private val redisHashKey = "charted:docker_registry:tokens"
    private val log by logging<DefaultRegistryAuthorizationPolicyManager>()

    init {
        log.info("Collecting all authorization tokens from Redis...")
        val sw = StopWatch.createStarted()
        val tokens = runBlocking { redis.commands.hgetall(redisHashKey).await() }

        sw.suspend()
        log.info("Took ${sw.doFormatTime()} to collect ${tokens.size} tokens")
        sw.resume()

        for (k in tokens.keys) {
            if (sw.isSuspended) sw.resume()

            log.debug("Collecting TTL for authorization token [$k]")
            val ttl = runBlocking { redis.commands.ttl(k).await() }
            sw.suspend()

            if (ttl == -1L) {
                log.warn("Authorization token [$k] has expired! [took ${sw.doFormatTime()}]")
                runBlocking { redis.commands.hdel(redisHashKey, k).await() }
            } else {
                log.debug("Authorization token $k expires in $ttl seconds! [took ${sw.doFormatTime()}]")
                expirationJobs.add(
                    ChartedScope.launch {
                        delay(ttl.toDuration(DurationUnit.MILLISECONDS))
                        redis.commands.hdel(redisHashKey, k).await()
                    },
                )
            }
        }
    }

    override fun isTokenExpired(token: String?): Boolean = sessionManager.isTokenExpired(token)
    override fun isTokenValid(token: String?): Boolean = if (token.isNullOrBlank() || isTokenExpired(token)) {
        false
    } else {
        val verifier = JWT.require(algorithm)
            .withIssuer("Noelware/charted")
            .build()

        val jwt = verifier.verify(token)
        val payload: JsonObject = json.decodeFromString(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))

        val issuer = payload["iss"]?.jsonPrimitive?.content
        if (issuer == null || issuer != "Noelware/charted") {
            throw IllegalStateException("Malformed JWT token: 'issuer' was not 'Noelware/charted'")
        }

        true
    }

    override suspend fun doAuthorize(header: String?): RegistryAuthorizationToken? {
        if (header == null) return null

        // Check if it is "Basic <...>"
        val basicAuth = header.split(' ', limit = 2)
        if (basicAuth.size != 2) {
            throw IllegalStateException("Expected header to be 'Basic <b64-encoded>'")
        }

        val basicAuthContents = basicAuth.last()
        val decoded = String(Base64.getDecoder().decode(basicAuthContents.toByteArray()))
        val userPass = decoded.split(':', limit = 2)
        if (userPass.size != 2) {
            throw IllegalStateException("Expected Basic header contents to be 'username:password'")
        }

        val (username, password) = userPass

        // Check if a token already exists, if so, return `null`
        val token = redis.commands.hget(redisHashKey, "users:$username").await().ifNotNull {
            json.decodeFromString(RegistryAuthorizationToken.serializer(), this)
        }

        if (token != null) return token

        val user = asyncTransaction(ChartedScope) {
            UserEntity.find { UserTable.username eq username }.firstOrNull()
        } ?: throw IllegalStateException("User with username [$username] was not found")

        return if (sessionManager.isPasswordValid(user, password)) {
            create(user)
        } else {
            null
        }
    }

    override suspend fun create(user: UserEntity): RegistryAuthorizationToken {
        val expires = 2.days
        val authorizationToken = JWT.create()
            .withIssuer("Noelware/charted")
            .withExpiresAt(Instant.now().plusMillis(expires.inWholeMilliseconds))
            .withHeader(
                mapOf(
                    "user" to user.id.value,
                ),
            )
            .sign(algorithm)

        val token = RegistryAuthorizationToken(user.id.value, RegistryAuthorizationScopes().addAll().bits(), authorizationToken)
        redis.commands.hmset(
            redisHashKey,
            mapOf(
                "users:${user.username}" to json.encodeToString(authorizationToken),
            ),
        )

        redis.commands.set("$redisHashKey:users:${user.username}", "<nothing to see here!>", SetArgs().ex(expires.toJavaDuration())).await()
        expirationJobs.add(
            ChartedScope.launch {
                delay(expires)
                redis.commands.hdel(redisHashKey, "users:${user.username}").await()
            },
        )

        return token
    }

    override fun close() {
        TODO("Not yet implemented")
    }
}
