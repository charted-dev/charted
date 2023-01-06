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

package org.noelware.charted.modules.sessions.local

import kotlinx.serialization.json.Json
import org.noelware.charted.ValidationException
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.modules.sessions.SessionManager
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder

class LocalSessionManager(
    private val argon2: Argon2PasswordEncoder,
    redis: RedisClient,
    json: Json,
    config: Config
) : SessionManager(redis, json, "local", config) {
    /**
     * Does the actual authentication process with the given [user] and the
     * [password] itself.
     *
     * @param user        The user that was found to authenticate with
     * @param password    The password to do the authentication
     */
    override suspend fun doAuthenticate(user: UserEntity, password: String): Session {
        // Check if the user's password is null, since it can happen (if user was in LDAP
        // but switched to local sessions, which we should do migrations if that ever
        // happens.)
        check(user.password != null) { "`password` field is null, which I am assuming that this account is from a non-local session manager, please do migrations~!" }

        // Now, let's check if the user's password is correct or not
        if (!isPasswordValid(user, password)) {
            throw ValidationException("body.password", "Password was invalid.")
        }

        // Now, let's just create the session.
        return create(user.id.value)
    }

    /**
     * Checks if the given [password] is valid or not. This is mainly used for Basic
     * authentication
     *
     * @param user [UserEntity] object
     * @param password The password to check for
     */
    override suspend fun isPasswordValid(user: UserEntity, password: String): Boolean = argon2.matches(password, user.password)
}
