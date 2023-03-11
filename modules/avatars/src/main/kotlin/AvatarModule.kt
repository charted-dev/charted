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

package org.noelware.charted.modules.avatars

import io.ktor.http.*
import io.ktor.http.content.*
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.models.users.User

/**
 * Represents the module for handling avatars from different sources.
 */
interface AvatarModule {
    suspend fun gravatar(email: String): ByteArray
    suspend fun identicons(id: Long): ByteArray
    suspend fun retrieveRepoIcon(repository: Repository, hash: String? = null): Pair<ContentType, ByteArray>?
    suspend fun updateRepoIcon(repository: Repository, part: PartData.FileItem)
    suspend fun retrieveOrgAvatar(org: Organization, hash: String? = null): Pair<ContentType, ByteArray>?
    suspend fun updateOrgAvatar(org: Organization, part: PartData.FileItem)
    suspend fun retrieveUserAvatar(user: User, hash: String? = null): Pair<ContentType, ByteArray>?
    suspend fun updateUserAvatar(user: User, part: PartData.FileItem)
}
