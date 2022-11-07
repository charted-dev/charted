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

package org.noelware.charted.modules.sessions.local

import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.Job
import kotlinx.serialization.json.Json
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.modules.sessions.SessionManager

class LocalSessionManager(
    private val redis: RedisClient,
    private val config: Config,
    private val json: Json
): SessionManager {
    private val expirationJobs: MutableMap<Long, Job> = mutableMapOf()
    private val log by logging<LocalSessionManager>()

    /**
     * Does the actual authentication process with the given [username or email][userOrEmail] and the
     * [password] itself.
     *
     * @param userOrEmail The username or email to authenticate
     * @param password    The password to do the authentication
     */
    override suspend fun doAuthenticate(userOrEmail: String, password: String): Session {
        TODO("Not yet implemented")
    }

    /**
     * Fetch a [Session] from Redis with the given [token]. If the session was found
     * and hasn't expired, a [Session] object will return, otherwise null.
     *
     * @param token The access or refresh token to validate
     */
    override suspend fun fetch(token: String): Session? {
        TODO("Not yet implemented")
    }

    /**
     * Refresh a [session] object and returns a new session
     * @param session The session to refresh
     */
    override suspend fun refresh(session: Session): Session {
        TODO("Not yet implemented")
    }

    /**
     * Revokes a session from Redis and returns a [Boolean] for a successful
     * revoke, or it failed somehow.
     *
     * @param session The session to revoke
     */
    override suspend fun revoke(session: Session): Boolean {
        TODO("Not yet implemented")
    }
}
