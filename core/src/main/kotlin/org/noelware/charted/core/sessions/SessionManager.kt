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
import kotlinx.serialization.json.*
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.data.Config
import java.io.Closeable
import java.time.Instant
import java.util.*
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours

class SessionManager(config: Config, private val json: Json, private val redis: IRedisClient): Closeable {
    private val algorithm = Algorithm.HMAC512(config.jwtSecretKey.toByteArray())
    private val jobs = mutableMapOf<UUID, Job>()
    private val log by logging<SessionManager>()

    init {
        val sessions = runBlocking { redis.commands.hgetall("charted:sessions").await() }
        for (key in sessions.keys) {
            val ttl = runBlocking { redis.commands.ttl(key).await() }

            if (ttl == -2L) continue
            if (ttl == -1L) {
                runBlocking { redis.commands.hdel("charted:sessions", key).await() }
            } else {
                jobs[UUID.fromString(key)] = ChartedScope.launch {
                    delay(ttl / 1000)
                    redis.commands.hdel("charted:sessions", key).await()
                }
            }
        }
    }

    override fun close() {
        log.warn("Closing off ${jobs.size} session expiration jobs...")
        for (job in jobs.values) job.cancel()

        log.warn("Done!")
    }

    fun isExpired(token: String): Boolean = try {
        val verifier = JWT.require(algorithm)
            .withIssuer("Noelware/charted-server")
            .build()

        val jwt = verifier.verify(token)
        val payload = json.decodeFromString<JsonObject>(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))

        payload["exp"]?.jsonPrimitive?.intOrNull?.ifNotNull { it >= Clock.System.now().epochSeconds } ?: true
    } catch (e: TokenExpiredException) {
        true
    } catch (e: Exception) {
        throw e
    }

    suspend fun retrieve(token: String): Session? {
        if (token.isEmpty()) return null

        val verifier = JWT.require(algorithm)
            .withIssuer("Noelware/charted-server")
            .build()

        val jwt = verifier.verify(token)
        val payload = json.decodeFromString<JsonObject>(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))
        val header = json.decodeFromString<JsonObject>(String(Base64.getDecoder().decode(jwt.header.toByteArray())))

        val iss = payload["iss"]?.jsonPrimitive?.content
        if (iss == null || iss != "Noelware/charted-server") {
            error("Issuer was not a valid issuer.")
        }

        val sessionID = header["session_id"]?.jsonPrimitive?.contentOrNull
            ?: error("Missing `session_id` in header payload")

        return redis.commands.hget("charted:sessions", sessionID).await().ifNotNull {
            json.decodeFromString(it)
        }
    }

    suspend fun create(id: Long): Session {
        val sessionID = UUID.randomUUID()

        // short-lived token (~12 hours)
        val accessToken = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(Date.from(Instant.now().plusMillis(12.hours.inWholeMilliseconds)))
            .withHeader(
                mapOf(
                    "session_id" to sessionID.toString(),
                    "user_id" to id
                )
            ).sign(algorithm)

        // long-lived token (~7 days) for just refreshing the access token to keep
        // this session alive. if it keeps refreshing, then it'll invalidate the
        // access token AND refresh token
        val refreshToken = JWT.create()
            .withIssuer("Noelware/charted-server")
            .withExpiresAt(Date.from(Instant.now().plusMillis(7.days.inWholeMilliseconds)))
            .withHeader(
                mapOf(
                    "session_id" to sessionID.toString(),
                    "user_id" to id
                )
            ).sign(algorithm)

        val session = Session(
            id,
            sessionID,
            refreshToken,
            accessToken
        )

        redis.commands.expire(sessionID.toString(), 7.days.inWholeSeconds).await()
        redis.commands.hmset(
            "charted:sessions",
            mapOf(
                "$sessionID" to json.encodeToString(session)
            )
        ).await()

        jobs[sessionID] = ChartedScope.launch {
            delay(7.days.inWholeMilliseconds)
            redis.commands.hdel("charted:sessions", sessionID.toString()).await()
        }

        return session
    }

    suspend fun revoke(session: Session) {
        redis.commands.hdel("charted:sessions", session.sessionID.toString()).await()
        val job = jobs[session.sessionID]

        job?.cancel()
        jobs.remove(session.sessionID)
    }

    suspend fun refreshSession(session: Session): Session {
        revoke(session)
        return create(session.userID)
    }

    suspend fun revokeAllSessions(userID: Long) {
        val sessions = redis.commands.hgetall("charted:sessions").await()
            .mapValues { json.decodeFromString<Session>(it.value) }
            .filter { it.value.userID == userID }

        for (session in sessions.values) {
            revoke(session)
        }
    }
}
