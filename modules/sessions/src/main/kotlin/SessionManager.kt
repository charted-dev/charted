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

package org.noelware.charted.modules.sessions

/**
 * Represents a manager for managing sessions from Redis. This also takes care of authentication
 * when you log in since different session managers need to do extra work if needed.
 */
interface SessionManager {
    /**
     * Does the actual authentication process with the given [username or email][userOrEmail] and the
     * [password] itself.
     *
     * @param userOrEmail The username or email to authenticate
     * @param password    The password to do the authentication
     */
    suspend fun doAuthenticate(userOrEmail: String, password: String): Session

    /**
     * Fetch a [Session] from Redis with the given [token]. If the session was found
     * and hasn't expired, a [Session] object will return, otherwise null.
     *
     * @param token The access or refresh token to validate
     */
    suspend fun fetch(token: String): Session?

    /**
     * Refresh a [session] object and returns a new session
     * @param session The session to refresh
     */
    suspend fun refresh(session: Session): Session

    /**
     * Revokes a session from Redis and returns a [Boolean] for a successful
     * revoke, or it failed somehow.
     *
     * @param session The session to revoke
     */
    suspend fun revoke(session: Session): Boolean
}
