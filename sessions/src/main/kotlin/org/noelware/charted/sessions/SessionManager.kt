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

package org.noelware.charted.sessions

import java.io.Closeable
import java.util.UUID

/**
 * Represents the manager for handling sessions.
 */
interface SessionManager: Closeable {
    /**
     * Retrieves a session from Redis.
     * @param token The JWT token that the session was created from.
     */
    suspend fun getSession(token: String): Session?

    /**
     * Retrieves a session by its UUID.
     * @param id The session's UUID.
     */
    suspend fun getSessionById(id: UUID): Session?

    /**
     * Creates a session.
     * @param userID The user's ID
     */
    suspend fun createSession(userID: Long): Session

    /**
     * Revokes all the user's sessions.
     * @param userID The user's ID
     */
    suspend fun revokeAllSessions(userID: Long)

    /**
     * Revokes a session object.
     */
    suspend fun revokeSession(session: Session)

    /**
     * Refreshes the old session with a new session object.
     */
    suspend fun refreshSession(session: Session): Session

    /**
     * Checks if the [token] is expired or not.
     */
    fun isExpired(token: String): Boolean
}
