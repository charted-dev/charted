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

package org.noelware.charted.modules.sessions

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
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.charted.modules.redis.RedisClient
import java.io.Closeable
import java.time.Instant
import java.util.*
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours
import kotlin.time.DurationUnit
import kotlin.time.toDuration
import kotlin.time.toJavaDuration

abstract class SessionManager(
    private val redis: RedisClient,
    private val json: Json,
    key: String,
    config: Config
): Closeable {
    private val expirationJobs: MutableMap<UUID, Job> = mutableMapOf()
    private val algorithm: Algorithm = Algorithm.HMAC512(config.jwtSecretKey)
    private val log by logging<SessionManager>()

    private val sessionKey: String = "charted:sessions:$key"

    init {
        log.info("Collecting all sessions from Redis...")
        val sw = StopWatch.createStarted()
        val sessions = runBlocking { redis.commands.hgetall(sessionKey).await() }

        sw.suspend()
        log.info("Took ${sw.doFormatTime()} to collect ${sessions.size} sessions from Redis!")
        for ((k, value) in sessions) {
            sw.resume()

            log.debug("Collecting TTL for session [$k]")
            val ttl = runBlocking { redis.commands.ttl(k).await() }
            sw.suspend()

            if (ttl == -1L) {
                log.warn("Session $ttl has expired! [${sw.doFormatTime()}]")
                runBlocking { redis.commands.hdel(sessionKey, k).await() }
            } else {
                log.debug("Session $k expires in $ttl seconds! [${sw.doFormatTime()}]")
                expirationJobs[UUID.fromString(k)] = ChartedScope.launch {
                    delay((ttl.toDuration(DurationUnit.SECONDS)).inWholeMilliseconds)
                    revoke(json.decodeFromString(value))
                }
            }
        }
    }

    /**
     * Returns a [boolean][Boolean] if the [token] given has expired or not.
     * @param token The token itself
     */
    fun isTokenExpired(token: String?): Boolean {
        if (token.isNullOrBlank()) return true
        try {
            val verifier = JWT.require(algorithm)
                .withIssuer("Noelware/charted")
                .build()

            val jwt = verifier.verify(token)
            val payload: JsonObject = json.decodeFromString(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))
            return payload["exp"]?.jsonPrimitive?.intOrNull?.ifNotNull {
                this >= Clock.System.now().epochSeconds
            } ?: true
        } catch (_: TokenExpiredException) {
            return true
        } catch (e: Exception) {
            ifSentryEnabled { Sentry.captureException(e) }
            throw e
        }
    }

    /**
     * Fetch a [Session] from Redis with the given [token]. If the session was found
     * and hasn't expired, a [Session] object will return, otherwise null.
     *
     * @param token The access or refresh token to validate
     */
    suspend fun fetch(token: String): Session? = if (token.isBlank()) {
        null
    } else {
        val verifier = JWT.require(algorithm)
            .withIssuer("Noelware/charted")
            .build()

        val jwt = verifier.verify(token)
        val payload: JsonObject = json.decodeFromString(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))
        val headers: JsonObject = json.decodeFromString(String(Base64.getDecoder().decode(jwt.header.toByteArray())))

        val issuer = payload["iss"]?.jsonPrimitive?.content
        if (issuer == null || issuer != "Noelware/charted") {
            throw IllegalStateException("Malformed JWT token: 'issuer' was not 'Noelware/charted'")
        }

        val sessionID = headers["session"]?.jsonPrimitive?.content
            ?: throw IllegalStateException("Malformed JWT token: missing 'session' header")

        redis.commands.hget(sessionKey, sessionID).await()
            .ifNotNull { json.decodeFromString(this) }
    }

    /**
     * Lists all the sessions that a user by their ID has created.
     * @param id The ID of the user
     * @return list of sessions
     */
    suspend fun all(id: Long): List<Session> = redis.commands.hgetall(sessionKey).await()
        // TODO: this isn't probably performant, so we might need to
        //       refactor this
        .filterValues {
            val serialized: Session = json.decodeFromString(it)
            serialized.userID == id
        }.map {
            json.decodeFromString(it.value)
        }

    /**
     * Creates a new [Session] object by the user's ID.
     * @param userID The user's ID
     * @return created session
     */
    suspend fun create(userID: Long): Session {
        val sessionID = UUID.randomUUID()

        // This will create the access token, which is usually short-lived. You will need
        // to use the refresh token to refresh the session (POST /users/@me/sessions/refresh) with
        // the given refresh token.
        val accessToken = JWT.create()
            .withIssuer("Noelware/charted")
            .withExpiresAt(Instant.now().plusMillis(12.hours.inWholeMilliseconds))
            .withHeader(
                mapOf(
                    "session" to sessionID.toString(),
                    "user" to "$userID",
                ),
            ).sign(algorithm)

        // This will create the refresh token which will refresh the access token
        // that has lasted for 12 hours. If you're just doing API requests to the server,
        // it is best recommended to use the API Keys routes to have custom expiration dates
        // and an easier way to use the API.
        val week = 7.days
        val refreshToken = JWT.create()
            .withIssuer("Noelware/charted")
            .withExpiresAt(Instant.now().plusMillis(week.inWholeMilliseconds))
            .withHeader(
                mapOf(
                    "session" to sessionID.toString(),
                    "user" to "$userID",
                ),
            ).sign(algorithm)

        val session = Session(refreshToken, accessToken, sessionID, userID)
        redis.commands.hmset(sessionKey, mapOf("$sessionID" to json.encodeToString(session))).await()
        redis.commands.set(sessionID.toString(), "<nothing to see here!>", SetArgs().ex(week.toJavaDuration())).await()

        // Now, we create the expiration job here
        expirationJobs[sessionID] = ChartedScope.launch {
            delay(week.inWholeMilliseconds)
            revoke(session)
        }

        return session
    }

    /**
     * Refresh a [session] object and returns a new session
     * @param session The session to refresh
     */
    suspend fun refresh(session: Session): Session {
        revoke(session)
        return create(session.userID)
    }

    /**
     * Revokes a session from Redis and returns a [Boolean] for a successful
     * revoke, or it failed somehow.
     *
     * @param session The session to revoke
     */
    suspend fun revoke(session: Session) {
        redis.commands.hdel(sessionKey, session.sessionID.toString()).await()
        redis.commands.del(session.sessionID.toString()).await()

        if (expirationJobs.contains(session.sessionID)) {
            val job = expirationJobs[session.sessionID]!!
            job.cancel()

            expirationJobs.remove(session.sessionID)
        }
    }

    /**
     * Revokes all the sessions given by a [userID].
     * @param userID The user ID to delete all sessions from.
     */
    suspend fun revokeAll(userID: Long) {
        log.warn("Deleting all sessions from user [$userID]")
        return redis.commands.hgetall(sessionKey)
            .await()
            // TODO: this is isn't probably performant, so we might need to
            //       refactor this
            .filterValues {
                val serialized: Session = json.decodeFromString(it)
                serialized.userID == userID
            }.forEach {
                runBlocking { redis.commands.hdel(sessionKey, it.key).await() }
            }
    }

    /**
     * Does the actual authentication process with the given [user] and the
     * [password] itself.
     *
     * @param user        The user that was found to authenticate with
     * @param password    The password to do the authentication
     */
    abstract suspend fun doAuthenticate(user: UserEntity, password: String): Session

    /**
     * Checks if the given [password] is valid or not. This is mainly used for Basic
     * authentication
     *
     * @param user [UserEntity] object
     * @param password The password to check for
     */
    abstract suspend fun isPasswordValid(user: UserEntity, password: String): Boolean

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
        log.warn("Closing all sessions!")
        for (job in expirationJobs.values) job.cancel()
    }
}
