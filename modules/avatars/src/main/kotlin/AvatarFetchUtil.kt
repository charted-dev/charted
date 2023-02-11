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

package org.noelware.charted.modules.avatars

import dev.floofy.utils.koin.inject
import io.ktor.http.*
import io.ktor.http.content.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.update
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.ValidationException
import org.noelware.charted.databases.postgres.asyncTransaction
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.remi.support.filesystem.FilesystemStorageService
import java.io.ByteArrayInputStream
import java.io.File

private val ACCEPTABLE_CONTENT_TYPES: List<String> = listOf("png", "jpeg", "gif").map { "image/$it" }

/**
 * Auxiliary utility for handling avatar storage.
 */
@Deprecated("AvatarFetchUtil's methods have been switched to DefaultAvatarModule", level = DeprecationLevel.WARNING)
object AvatarFetchUtil {
    private val storage: StorageHandler by inject()
    private val module: AvatarModule by inject()

    @Deprecated("Use AvatarModule#retrieveRepoIcon instead", level = DeprecationLevel.WARNING)
    suspend fun retrieveRepositoryIcon(repository: Repository, hash: String? = null): Pair<ContentType, ByteArray>? {
        if (repository.iconHash == null) return null

        if (hash != null) {
            val stream = storage.open("./repositories/${repository.ownerID}/${repository.id}/icons/$hash") ?: return null
            val bytes = stream.use { it.readBytes() }

            return when (val contentType = storage.service.getContentTypeOf(bytes)) {
                ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() ->
                    ContentType.parse(contentType) to bytes

                else -> {
                    if (hash == repository.iconHash) {
                        asyncTransaction {
                            RepositoryTable.update({ RepositoryTable.id eq repository.id }) {
                                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                                it[iconHash] = null
                            }
                        }
                    }

                    storage.delete("./repositories/${repository.ownerID}/${repository.id}/icons/$hash")
                    null
                }
            }
        }

        val stream = storage.open("./repositories/${repository.ownerID}/${repository.id}/icons/${repository.iconHash}")
            ?: return null

        val bytes = stream.use { it.readBytes() }
        return when (val contentType = storage.service.getContentTypeOf(bytes)) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() ->
                ContentType.parse(contentType) to bytes

            else -> {
                asyncTransaction {
                    RepositoryTable.update({ RepositoryTable.id eq repository.id }) {
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                        it[iconHash] = null
                    }
                }

                storage.delete("./repositories/${repository.ownerID}/${repository.id}/icons/${repository.iconHash}")
                null
            }
        }
    }

    @Deprecated("Use AvatarModule#updateRepoIcon instead", level = DeprecationLevel.WARNING)
    suspend fun updateRepositoryIcon(repository: Repository, part: PartData.FileItem) {
        val bytes = part.streamProvider().use { it.readBytes() }
        val contentType = storage.service.getContentTypeOf(bytes) ?: "application/octet-stream"
        if (!ACCEPTABLE_CONTENT_TYPES.contains(contentType)) {
            throw ValidationException("body", "File was not any of [${ACCEPTABLE_CONTENT_TYPES.joinToString(", ")}], received $contentType")
        }

        val hash = RandomStringGenerator.generate(8)
        val ext = when {
            contentType.startsWith("image/jpg") || contentType.startsWith("image/jpeg") -> ".jpg"
            contentType.startsWith("image/png") -> ".png"
            contentType.startsWith("image/gif") -> ".gif"
            else -> throw AssertionError("ext != png/jpg/gif when passed through.")
        }

        if (storage.service is FilesystemStorageService) {
            val service = storage.service as FilesystemStorageService
            val file = File(service.normalizePath("./repositories/${repository.ownerID}/${repository.id}/avatars"))
            if (!file.exists()) file.mkdirs()
        }

        storage.upload("./repositories/${repository.ownerID}/${repository.id}/avatars/$hash$ext", ByteArrayInputStream(bytes), contentType)
        part.dispose()

        asyncTransaction {
            RepositoryTable.update({ RepositoryTable.id eq repository.id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                it[iconHash] = "$hash$ext"
            }
        }
    }

    /**
     * Returns a byte-array of the image of the specified user and the content type. Defaults to
     * identicons if the user doesn't have an avatar OR they didn't specify
     * a Gravatar email.
     *
     * This method returns null if the user couldn't be found.
     */
    @Deprecated("Use AvatarModule#retrieveUserAvatar instead", level = DeprecationLevel.WARNING)
    suspend fun retrieve(user: User, hash: String? = null): Pair<ContentType, ByteArray>? {
        if (user.avatarHash == null) {
            return if (user.gravatarEmail != null) {
                ContentType.Image.PNG to module.gravatar(user.gravatarEmail!!)
            } else {
                ContentType.Image.SVG to module.identicons(user.id)
            }
        }

        if (hash != null) {
            val stream = storage.open("./users/${user.id}/avatars/$hash")
                ?: return null

            val bytes = stream.use { it.readBytes() }
            return when (val contentType = storage.service.getContentTypeOf(bytes)) {
                ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> ContentType.parse(contentType) to bytes
                else -> {
                    if (hash == user.avatarHash) {
                        asyncTransaction {
                            UserTable.update({ UserTable.id eq user.id }) {
                                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                                it[avatarHash] = null
                            }
                        }
                    }

                    storage.delete("./avatars/${user.id}/$hash")
                    if (user.gravatarEmail != null) {
                        ContentType.Image.PNG to module.gravatar(user.gravatarEmail!!)
                    } else {
                        ContentType.Image.SVG to module.identicons(user.id)
                    }
                }
            }
        }

        val stream = storage.open("./users/${user.id}/avatars/${user.avatarHash}")
            ?: return null

        val bytes = stream.use { it.readBytes() }
        return when (val contentType = storage.service.getContentTypeOf(bytes)) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> ContentType.parse(contentType) to bytes
            else -> {
                asyncTransaction {
                    UserTable.update({ UserTable.id eq user.id }) {
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                        it[avatarHash] = null
                    }
                }

                storage.delete("./users/${user.id}/avatars/${user.avatarHash}")
                if (user.gravatarEmail != null) {
                    ContentType.Image.PNG to module.gravatar(user.gravatarEmail!!)
                } else {
                    ContentType.Image.SVG to module.identicons(user.id)
                }
            }
        }
    }

    @Deprecated("Use AvatarModule#updateUserAvatar instead", level = DeprecationLevel.WARNING)
    suspend fun update(id: Long, part: PartData.FileItem) {
        val bytes = part.streamProvider().use { it.readBytes() }
        val contentType = storage.service.getContentTypeOf(bytes) ?: "application/octet-stream"
        if (!ACCEPTABLE_CONTENT_TYPES.contains(contentType)) {
            throw ValidationException("body", "File was not any of [${ACCEPTABLE_CONTENT_TYPES.joinToString(", ")}], received $contentType")
        }

        val hash = RandomStringGenerator.generate(8)
        val ext = when {
            contentType.startsWith("image/jpg") || contentType.startsWith("image/jpeg") -> ".jpg"
            contentType.startsWith("image/png") -> ".png"
            contentType.startsWith("image/gif") -> ".gif"
            else -> throw AssertionError("ext != png/jpg/gif when passed through.")
        }

        if (storage.service is FilesystemStorageService) {
            val file = File((storage.service as FilesystemStorageService).normalizePath("./users/$id/avatars"))
            if (!file.exists()) file.mkdirs()
        }

        storage.upload("./users/$id/avatars/$hash$ext", ByteArrayInputStream(bytes), contentType)
        part.dispose()

        asyncTransaction {
            UserTable.update({ UserTable.id eq id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                it[avatarHash] = "$hash$ext"
            }
        }
    }
}
