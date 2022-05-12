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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core.sessions

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.auth0.jwt.exceptions.TokenExpiredException
import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.core.ChartedScope
import org.noelware.charted.core.config.Config
import org.noelware.charted.core.launch
import org.noelware.charted.core.redis.IRedisClient
import java.io.Closeable
import java.time.Instant
import java.util.*
import java.util.concurrent.TimeUnit
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours

class SessionManager(private val redis: IRedisClient, private val json: Json, config: Config): Closeable {
    private val algorithm = Algorithm.HMAC512(config.jwtSecretKey.toByteArray())
    private val jobs = mutableListOf<Job>()
    private val log by logging<SessionManager>()

    init {
        log.info("Initializing the sessions manager...")

        val sw = StopWatch.createStarted()
        val sessions = runBlocking {
            redis.commands.hgetall("charted:sessions").await()
        }

        sw.stop()
        log.info("Received ${sessions.size} session entries in ${sw.getTime(TimeUnit.MILLISECONDS)}ms, clearing expired entries...")

        sw.reset()
        sw.start()
        for (key in sessions.keys) {
            log.debug("Checking expiration of charted:sessions:$key...")
            val ttl = runBlocking {
                redis.commands.ttl("charted:sessions:$key").await()
            }

            if (ttl == -1L) {
                log.debug("Session charted:sessions:$key has expired! Deleting entry...")
                runBlocking {
                    redis.commands.hdel("charted:sessions", key).await()
                }
            } else {
                log.debug("Session for $key expires in $ttl seconds.")
                jobs.add(
                    ChartedScope.launch {
                        delay(ttl / 1000)
                        log.debug("Session charted:sessions:$key has expired! Deleting entry...")

                        redis.commands.hdel("charted:sessions", key).await()
                    }
                )
            }
        }

        sw.stop()
        log.info("Took ${sw.getTime(TimeUnit.MILLISECONDS)} to delete expired sessions!")
    }

    override fun close() {
        log.warn("Closing off ${jobs.size} expiration jobs!")

        // TODO: this might be expensive at scale.
        for (job in jobs)
            job.cancel()

        log.info("Done!")
    }

    fun isExpired(token: String): Boolean = try {
        JWT.require(algorithm)
            .withIssuer("Noelware/charted-server")
            .build()

        false
    } catch (e: TokenExpiredException) {
        true
    } catch (e: Exception) {
        throw e
    }

    suspend fun getSession(token: String): Session? {
        if (token.isEmpty()) return null

        val verifier = JWT.require(algorithm)
            .withIssuer("Noelware/charted-server")
            .build()

        val jwt = verifier.verify(token)
        val payload = String(Base64.getDecoder().decode(jwt.payload.toByteArray()))
        val payloadAsJson = json.decodeFromString(JsonObject.serializer(), payload)
        println(payloadAsJson)

        return null
    }

    suspend fun createSession(userId: String): Session {
        val sessionId = UUID.randomUUID()

        // Create access token, which is short-lived.
        val accessExpiresAt = Date.from(Instant.now().plusMillis(12.hours.inWholeMilliseconds))
        val accessToken = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(accessExpiresAt)
            .withHeader(
                mapOf(
                    "session_id" to sessionId.toString(),
                    "user_id" to userId,
                    "expires_in" to accessExpiresAt.toInstant().toString()
                )
            )
            .sign(algorithm)

        // Create refresh token, which is long-lived for 7 days. If you keep refreshing
        // the token with this, it'll invalidate the previous one that was stored.
        val refreshExpiresAt = Date.from(Instant.now().plusMillis(7.days.inWholeMilliseconds))
        val refreshToken = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(refreshExpiresAt)
            .withHeader(
                mapOf(
                    "session_id" to sessionId,
                    "user_id" to userId,
                    "expires_in" to refreshExpiresAt.toInstant().toString()
                )
            )
            .sign(algorithm)

        val session = Session(
            userId.toLong(),
            sessionId,
            refreshToken,
            accessToken
        )

        redis.commands.set("charted:sessions:$userId-$sessionId", "owo")
        redis.commands.expire("charted:sessions:$userId-$sessionId", 7.days.inWholeMilliseconds)
        redis.commands.hmset("charted:sessions", mapOf("$userId-$sessionId" to json.encodeToString(Session.serializer(), session))).await()
        return session
    }

    suspend fun revokeSession(session: Session) {
        redis.commands.hdel("charted:sessions", "${session.userId}-${session.sessionId}").await()
    }

    suspend fun refreshSession(session: Session): Session {
        revokeSession(session)

        // Create access token, which is short-lived.
        val accessExpiresAt = Date.from(Instant.now().plusMillis(12.hours.inWholeMilliseconds))
        val accessToken = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(accessExpiresAt)
            .withHeader(
                mapOf(
                    "session_id" to session.sessionId.toString(),
                    "user_id" to session.userId.toString(),
                    "expires_in" to accessExpiresAt.toInstant().toString()
                )
            )
            .sign(algorithm)

        // Create refresh token, which is long-lived for 7 days. If you keep refreshing
        // the token with this, it'll invalidate the previous one that was stored.
        val refreshExpiresAt = Date.from(Instant.now().plusMillis(7.days.inWholeMilliseconds))
        val refreshToken = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(refreshExpiresAt)
            .withHeader(
                mapOf(
                    "session_id" to session.sessionId.toString(),
                    "user_id" to session.userId.toString(),
                    "expires_in" to refreshExpiresAt.toInstant().toString()
                )
            )
            .sign(algorithm)

        val s = Session(
            session.userId,
            session.sessionId,
            refreshToken,
            accessToken
        )

        val userId = s.userId
        val sessionId = s.sessionId

        redis.commands.set("charted:sessions:$userId-$sessionId", "owo")
        redis.commands.expire("charted:sessions:$userId-$sessionId", 7.days.inWholeMilliseconds)
        redis.commands.hmset("charted:sessions", mapOf("$userId-$sessionId" to json.encodeToString(Session.serializer(), s))).await()
        return s
    }
}
