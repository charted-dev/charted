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

package org.noelware.charted.modules.postgresql.ktor

import io.ktor.server.application.*
import io.ktor.util.*
import org.noelware.charted.modules.postgresql.entities.ApiKeyEntity
import org.noelware.charted.modules.postgresql.entities.RepositoryEntity
import org.noelware.charted.modules.postgresql.entities.UserEntity

val UserEntityAttributeKey: AttributeKey<UserEntity> = AttributeKey("User Entity")
val ApiKeyAttributeKey: AttributeKey<ApiKeyEntity> = AttributeKey("Api Key")

// this is only for the repository controller
val OwnerIdAttributeKey: AttributeKey<Long> = AttributeKey("Owner ID")

// this is only for the repository members and releases controllers
val RepositoryAttributeKey: AttributeKey<RepositoryEntity> = AttributeKey("Repository ID")

// internal because it only belongs here, not outside
internal val ApplicationCall.ownerId: Long?
    get() = attributes.getOrNull(OwnerIdAttributeKey)

// internal because it only belongs here, not outside
internal val ApplicationCall.repository: RepositoryEntity?
    get() = attributes.getOrNull(RepositoryAttributeKey)
