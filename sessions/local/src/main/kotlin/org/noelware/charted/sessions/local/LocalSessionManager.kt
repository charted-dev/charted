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

import dev.floofy.utils.slf4j.logging
import kotlinx.serialization.json.Json
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.sessions.Session
import org.noelware.charted.sessions.SessionManager
import java.util.UUID

/**
 * Represents a [SessionManager] for local user management.
 */
class LocalSessionManager(private val redis: IRedisClient, private val json: Json): SessionManager {
    private val expirationJobs = mutableMapOf<UUID, Session>()
    private val log by logging<LocalSessionManager>()

    init {
        log.info("")
    }

    /**
     * Retrieves a session from Redis.
     * @param token The JWT token that the session was created from.
     */
    override suspend fun getSession(token: String): Session? {
        TODO("Not yet implemented")
    }

    /**
     * Creates a session.
     * @param userID The user's ID
     */
    override suspend fun createSession(userID: Long): Session {
        TODO("Not yet implemented")
    }

    /**
     * Revokes all the user's sessions.
     * @param userID The user's ID
     */
    override suspend fun revokeAllSessions(userID: Long): Session {
        TODO("Not yet implemented")
    }

    /**
     * Revokes a session object.
     */
    override suspend fun revokeSession(session: Session) {
        TODO("Not yet implemented")
    }
}
