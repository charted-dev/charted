/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.modules.sessions.ldap

import com.unboundid.ldap.sdk.LDAPConnectionPool
import dev.floofy.utils.slf4j.logging
import kotlinx.serialization.json.Json
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.modules.sessions.SessionManager

class LDAPSessionManager(redis: RedisClient, json: Json, config: Config) : SessionManager(redis, json, "ldap", config) {
    private val connectionPool: LDAPConnectionPool? = null
    private val log by logging<LDAPSessionManager>()

    init {
        log.info("Creating LDAP connection...")
    }

    override suspend fun doAuthenticate(user: UserEntity, password: String): Session {
        TODO("Not yet implemented")
    }

    override suspend fun isPasswordValid(user: UserEntity, password: String): Boolean {
        TODO("Not yet implemented")
    }
}
