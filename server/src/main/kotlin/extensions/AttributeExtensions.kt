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

package org.noelware.charted.server.extensions

import io.ktor.server.application.*
import io.ktor.util.*
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.common.lazy.Lazy
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.ktor.ApiKeyAttributeKey
import org.noelware.charted.modules.postgresql.ktor.UserEntityAttributeKey
import org.noelware.charted.modules.sessions.Session
import kotlin.contracts.ExperimentalContracts
import kotlin.contracts.InvocationKind
import kotlin.contracts.contract

internal val sessionKey: AttributeKey<Session> = AttributeKey("Session")

val ApplicationCall.session: Session?
    get() = attributes.getOrNull(sessionKey)

@OptIn(ExperimentalContracts::class)
suspend fun <K: Any, T> Attributes.putAndRemove(attr: AttributeKey<K>, value: K, block: suspend () -> T): T {
    contract { callsInPlace(block, InvocationKind.EXACTLY_ONCE) }

    // if it doesn't contain the attribute, then add it.
    if (!contains(attr)) {
        put(attr, value)
    }

    return block().also { remove(attr) }
}

/**
 * Same as [currentUserEntity] but returns a safe-serializable [User] entity.
 */
val ApplicationCall.currentUser: User?
    // Since it can get expensive on the session side, we do it lazily and fetch it whenever we need it.
    get() = currentUserEntity?.let { entity -> User.fromEntity(entity) }

val ApplicationCall.currentUserEntity: UserEntity?
    get() = Lazy.create {
        // basic user
        if (attributes.contains(UserEntityAttributeKey)) {
            return@create attributes[UserEntityAttributeKey]
        }

        if (attributes.contains(sessionKey)) {
            val attr: Session = attributes[sessionKey]
            return@create transaction { UserEntity.findById(attr.userID) }
        }

        if (attributes.contains(ApiKeyAttributeKey)) {
            val attr = attributes[ApiKeyAttributeKey]
            return@create attr.owner
        }

        null
    }.get()
