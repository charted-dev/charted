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

package org.noelware.charted.sessions.local

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.auth0.jwt.exceptions.TokenExpiredException
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
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
import org.noelware.charted.sessions.Session
import org.noelware.charted.sessions.SessionManager
import java.time.Instant
import java.util.*
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours
import kotlin.time.DurationUnit
import kotlin.time.toDuration

/**
 * Represents a [SessionManager] for local user management.
 */
class LocalSessionManager(
    config: Config,
    private val redis: IRedisClient,
    private val json: Json
): SessionManager {
    private val algorithm = Algorithm.HMAC512(config.jwtSecretKey.toByteArray())
    private val expirationJobs = mutableMapOf<UUID, Job>()
    private val log by logging<LocalSessionManager>()

    init {
        log.info("Collecting sessions from Redis!")
        val sessions = log.measureSuspendTime("Took %T to collect sessions~") {
            redis.commands.hgetall("charted:sessions").await()
        }

        val sw = StopWatch.createStarted()
        for ((key, value) in sessions) {
            if (sw.isStopped) {
                sw.reset()
                sw.start()
            }

            log.debug("Collecting TTL for session [$key]")
            val ttl = runBlocking { redis.commands.ttl("sessions:$key").await() }
            if (ttl == -2L) continue
            if (ttl == -1L) {
                log.debug("Session $key expires! Deleting session...")
                runBlocking { revokeSession(json.decodeFromString(value)) }
            } else {
                log.debug("Session $key expires in $ttl seconds")
                expirationJobs[UUID.fromString(key)] = ChartedScope.launch {
                    delay((ttl.toDuration(DurationUnit.SECONDS)).inWholeMilliseconds)
                    revokeSession(json.decodeFromString(value))
                }
            }
        }
    }

    override fun close() {
        log.warn("Closing off ${expirationJobs.size} session expiration jobs...")
        for (job in expirationJobs.values) {
            job.cancel()
        }

        log.warn("Done! Closed off ${expirationJobs.size} sessions~")
    }

    override fun isExpired(token: String): Boolean = try {
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

    /**
     * Retrieves a session from Redis.
     * @param token The JWT token that the session was created from.
     */
    override suspend fun getSession(token: String): Session? {
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

        val sessionID = headers["session_id"]?.jsonPrimitive?.content
            ?: throw IllegalArgumentException("Malformed JWT token.")

        return redis.commands.hget("charted:sessions", sessionID).await().ifNotNull { json.decodeFromString(this) }
    }

    override suspend fun getSessionById(id: UUID): Session? = redis
        .commands
        .hget("charted:sessions", id.toString())
        .await()
        .ifNotNull {
            json.decodeFromString(this)
        }

    /**
     * Creates a session.
     * @param userID The user's ID
     */
    override suspend fun createSession(userID: Long): Session {
        val session = UUID.randomUUID()

        // Create a short-lived token (~12 hours) which will be the access token.
        // You can refresh the session via the refresh token on the POST /users/@me/refresh_session
        // endpoint.
        val accessToken = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(Date.from(Instant.now().plusMillis(12.hours.inWholeMilliseconds)))
            .withHeader(
                mapOf(
                    "session_id" to session.toString(),
                    "user_id" to userID
                )
            ).sign(algorithm)

        // Create a "long-lived" token that can be refreshed after the 12-hour
        // period is up. If you're just doing API requests and not session management,
        // it is best recommended to go with an API key which allow a permissions system
        // to not mess around with any data.
        val refreshToken = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(Date.from(Instant.now().plusMillis(7.days.inWholeMilliseconds)))
            .withHeader(
                mapOf(
                    "session_id" to session.toString(),
                    "user_id" to userID
                )
            ).sign(algorithm)

        val s = Session(refreshToken, accessToken, session, userID)
        val encoded = json.encodeToString(s)

        redis.commands.set(session.toString(), "<just here for expirations>").await()
        redis.commands.expire(session.toString(), 7.days.inWholeSeconds).await()
        redis.commands.hmset("charted:sessions", mapOf("$session" to encoded)).await()

        // Create the expiration job
        expirationJobs[session] = ChartedScope.launch {
            delay(7.days.inWholeMilliseconds)
            revokeSession(s)
        }

        return s
    }

    /**
     * Revokes all the user's sessions.
     * @param userID The user's ID
     */
    override suspend fun revokeAllSessions(userID: Long) {
        val sessions = redis.commands.hgetall("charted:sessions").await()
            .mapValues { json.decodeFromString<Session>(it.value) }
            .filter { it.value.userID == userID }

        for (session in sessions.values) revokeSession(session)
    }

    /**
     * Revokes a session object.
     */
    override suspend fun revokeSession(session: Session) {
        redis.commands.del(session.sessionID.toString()).await()
        redis.commands.hdel("charted:sessions", session.sessionID.toString()).await()

        if (expirationJobs.containsKey(session.sessionID)) {
            val job = expirationJobs[session.sessionID]!!
            job.cancel()
            expirationJobs.remove(session.sessionID)
        }
    }

    /**
     * Refreshes the old session with a new session object.
     */
    override suspend fun refreshSession(session: Session): Session {
        revokeSession(session)
        return createSession(session.userID)
    }
}
