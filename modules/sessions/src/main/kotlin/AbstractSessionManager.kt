/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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
import org.noelware.charted.launch
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
import org.noelware.charted.ChartedScope
import org.noelware.charted.common.extensions.sentry.ifSentryEnabled
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.redis.RedisClient
import java.io.Closeable
import java.time.Instant
import java.util.*
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours
import kotlin.time.DurationUnit
import kotlin.time.toDuration
import kotlin.time.toJavaDuration

abstract class AbstractSessionManager(
    type: String,
    config: Config,
    private val json: Json,
    private val redis: RedisClient
): Closeable {
    private val expirationJobs: MutableMap<UUID, Job> = mutableMapOf()
    private val redisTableKey: String = "charted:sessions:$type"
    private val jwtAlgorithm: Algorithm = Algorithm.HMAC512(config.jwtSecretKey.toByteArray())
    private val log by logging<AbstractSessionManager>()

    init {
        log.info("Collecting session information...")
        val sessions = runBlocking { redis.commands.hgetall(redisTableKey).await() } ?: mapOf()

        log.info("Collected and found ${sessions.size} sessions available")
        for ((key, ses) in sessions) {
            val uuidKey = UUID.fromString(key)
            val session = json.decodeFromString<Session>(ses)

            log.trace("...found session [$key]")
            val ttl = runBlocking { redis.commands.ttl("$redisTableKey:$key").await() }
            if (ttl == -1L) {
                log.warn("Session with key [$key] has expired")
                runBlocking { redis.commands.hdel(redisTableKey, key).await() }
            } else {
                log.trace("...session [$key] expires in $ttl seconds")
                expirationJobs[uuidKey] = ChartedScope.launch {
                    delay((ttl.toDuration(DurationUnit.SECONDS)).inWholeMilliseconds)
                    revoke(session)
                }
            }
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
     * Returns a [boolean][Boolean] if the [token] given has expired or not.
     * @param token The token itself
     */
    fun isTokenExpired(token: String? = null): Boolean {
        if (token.isNullOrBlank()) return true
        try {
            val verifier = JWT.require(jwtAlgorithm)
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
     * Returns all the sessions that the [user with this ID][userID] has created.
     * @param userID The user ID that was already created
     * @return [List] of all the sessions
     */
    suspend fun all(userID: Long): List<Session> = redis.commands.hgetall(redisTableKey)
        .await()
        .mapValues { json.decodeFromString(Session.serializer(), it.value) }
        .filterValues { it.userID == userID }
        .toList()
        .map { it.second }

    /**
     * Fetch a [Session] from Redis with the given [token]. If the session was found
     * and hasn't expired, a [Session] object will return, otherwise null.
     *
     * @param token The access or refresh token to validate
     */
    suspend fun fetch(token: String): Session? = if (token.isBlank()) {
        null
    } else {
        val verifier = JWT.require(jwtAlgorithm)
            .withIssuer("Noelware/charted")
            .build()

        val jwt = verifier.verify(token)
        val payload: JsonObject = json.decodeFromString(String(Base64.getDecoder().decode(jwt.payload.toByteArray())))
        val headers: JsonObject = json.decodeFromString(String(Base64.getDecoder().decode(jwt.header.toByteArray())))

        val issuer = payload["iss"]?.jsonPrimitive?.content
            ?: throw IllegalStateException("Malformed JWT token: missing 'iss' key")

        if (issuer != "Noelware/charted") {
            throw IllegalStateException("Malformed JWT token: 'issuer' was not 'Noelware/charted'")
        }

        val sessionID = headers["session"]?.jsonPrimitive?.content
            ?: throw IllegalStateException("Malformed JWT token: missing 'session' header")

        redis.commands.hget(redisTableKey, sessionID).await()
            .ifNotNull { json.decodeFromString(this) }
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
            ).sign(jwtAlgorithm)

        // This will create the refresh token which will refresh the access token
        // that has lasted for 12 hours. If you're just doing API requests to the server,
        // it is best recommended to use the API Key authentication to have custom expiration dates
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
            ).sign(jwtAlgorithm)

        val session = Session(refreshToken, accessToken, sessionID, userID)
        redis.commands.hmset(redisTableKey, mapOf("$sessionID" to json.encodeToString(session))).await()
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
        redis.commands.hdel(redisTableKey, session.sessionID.toString()).await()
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
        return redis.commands.hgetall(redisTableKey)
            .await()
            // TODO(@auguwu): refractor this later if this becomes a problem
            .filterValues {
                val serialized: Session = json.decodeFromString(it)
                serialized.userID == userID
            }.forEach {
                runBlocking { redis.commands.hdel(redisTableKey, it.key).await() }
            }
    }

    override fun close() {
        log.warn("Closing all sessions!")
        for (job in expirationJobs.values) job.cancel()
    }
}
