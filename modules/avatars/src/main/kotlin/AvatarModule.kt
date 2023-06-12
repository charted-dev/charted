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
    /**
     * Returns a [ByteArray] of a generated Gravatar image from an
     * email address.
     *
     * @param email The email to use to fetch the avatar
     */
    suspend fun gravatar(email: String): ByteArray

    /**
     * Returns a [ByteArray] of a generated [Dicebear Identicons](https://www.dicebear.com/styles/identicon)
     * image from an entity's ID.
     *
     * @param id The ID of the entity to be used as the hash.
     */
    suspend fun identicons(id: Long): ByteArray

    /**
     * Returns a [Pair] that contains the content type and the actual bytes that
     * should be sent to the recipient.
     *
     * This can return null
     *  * If the [hash] was provided and the storage service couldn't find it
     *  * If there was no previous icon for the repository.
     *
     * @param repository The repository to fetch the icon for
     * @param hash Optional hash identifier to retrieve a specific icon.
     * @return [Pair] that contains the [ContentType] and the actual byte array, or `null` if the
     * resource was not found.
     */
    suspend fun retrieveRepoIcon(repository: Repository, hash: String? = null): Pair<ContentType, ByteArray>?

    /**
     * Updates a repository icon on the server from a `multipart/form-data` request.
     * @param repository Repository entity to update for
     * @param part The [PartData] to use as the actual icon to use.
     */
    suspend fun updateRepoIcon(repository: Repository, part: PartData.FileItem)

    /**
     * Updates a repository icon on the server from a `text/plain` request.
     * @param repository Repository entity to update for
     * @param text The body to use, must be in the form of 'data:image/{format};base64,...'
     */
    suspend fun updateRepoIcon(repository: Repository, text: String) {
        TODO("As of v0.4.0-unstable.4, this is not supported.")
    }

    /**
     * Returns a [Pair] that contains the content type and the actual bytes that
     * should be sent to the recipient.
     *
     * This can return null
     *  * If the [hash] was provided and the storage service couldn't find it
     *  * If there was no previous avatar for the organization.
     *
     * @param org The organization to fetch the icon for
     * @param hash Optional hash identifier to retrieve a specific icon.
     * @return [Pair] that contains the [ContentType] and the actual byte array, or `null` if the
     * resource was not found.
     */
    suspend fun retrieveOrgAvatar(org: Organization, hash: String? = null): Pair<ContentType, ByteArray>?

    /**
     * Updates an organization avatar on the server from a `multipart/form-data` request.
     * @param org Repository entity to update for
     * @param part The [PartData] to use as the actual icon to use.
     */
    suspend fun updateOrgAvatar(org: Organization, part: PartData.FileItem)

    /**
     * Updates an organization avatar on the server from a `text/plain` request.
     * @param org Organization entity to update for
     * @param text The body to use, must be in the form of 'data:image/{format};base64,...'
     */
    suspend fun updateOrgIcon(org: Organization, text: String) {
        TODO("As of v0.4.0-unstable.4, this is not supported.")
    }

    /**
     * Returns a [Pair] that contains the content type and the actual bytes that
     * should be sent to the recipient.
     *
     * This can return null
     *  * If the [hash] was provided and the storage service couldn't find it
     *  * If there was no previous avatar for the user.
     *
     * @param user The user to fetch the icon for
     * @param hash Optional hash identifier to retrieve a specific icon.
     * @return [Pair] that contains the [ContentType] and the actual byte array, or `null` if the
     * resource was not found.
     */
    suspend fun retrieveUserAvatar(user: User, hash: String? = null): Pair<ContentType, ByteArray>?

    /**
     * Updates a user avatar on the server from a `multipart/form-data` request.
     * @param user User entity to update for
     * @param part The [PartData] to use as the actual icon to use.
     */
    suspend fun updateUserAvatar(user: User, part: PartData.FileItem)

    /**
     * Updates a user avatar on the server from a `text/plain` request.
     * @param user Repository entity to update for
     * @param text The body to use, must be in the form of 'data:image/{format};base64,...'
     */
    suspend fun updateUserIcon(user: User, text: String) {
        TODO("As of v0.4.0-unstable.4, this is not supported.")
    }
}
