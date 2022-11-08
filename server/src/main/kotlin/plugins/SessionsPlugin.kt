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

package org.noelware.charted.server.plugins

import dev.floofy.utils.koin.inject
import dev.floofy.utils.kotlin.ifNotNull
import io.ktor.server.application.*
import io.ktor.util.*
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.models.ApiKeys
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.modules.sessions.SessionManager

private val SESSIONS_KEY: AttributeKey<Session> = AttributeKey("Session")
private val API_KEY_KEY: AttributeKey<ApiKeys> = AttributeKey("ApiKey")

/**
 * Returns the current user that this endpoint is running as.
 */
val ApplicationCall.currentUser: User?
    get() = attributes.getOrNull(SESSIONS_KEY).ifNotNull {
        transaction {
            UserEntity.findById(userID)?.let { User.fromEntity(it) }
        }
    } ?: attributes.getOrNull(API_KEY_KEY).ifNotNull { owner }

val SessionsPlugin = createRouteScopedPlugin("Sessions") {
    val sessions: SessionManager by inject()

    onCall { call ->
    }
}
