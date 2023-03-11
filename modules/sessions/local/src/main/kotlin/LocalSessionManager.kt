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

package org.noelware.charted.modules.sessions.local

import io.ktor.http.*
import kotlinx.serialization.json.Json
import org.noelware.charted.KtorHttpRespondException
import org.noelware.charted.common.types.responses.ApiError
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.modules.sessions.Session
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder

class LocalSessionManager(
    json: Json,
    config: Config,
    redis: RedisClient,
    private val argon2: Argon2PasswordEncoder
): AbstractSessionManager("local", config, json, redis) {
    override suspend fun doAuthenticate(user: UserEntity, password: String): Session {
        check(user.password != null) {
            "`password` field is null, which I'd to assume -- this account was from a non-local session manager. Please run the `charted sessions migrate --from=ldap --to=local` command."
        }

        if (!isPasswordValid(user, password)) {
            throw KtorHttpRespondException(HttpStatusCode.Unauthorized, listOf(ApiError("INVALID_PASSWORD", "Password was invalid")))
        }

        return create(user.id.value)
    }

    override suspend fun isPasswordValid(user: UserEntity, password: String): Boolean {
        check(user.password != null) {
            "`password` field is null, which I'd to assume -- this account was from a non-local session manager. Please run the `charted sessions migrate --from=ldap --to=local` command."
        }

        return argon2.matches(password, user.password)
    }
}
