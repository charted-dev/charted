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

package org.noelware.charted.modules.avatars

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.kotlin.ifNotNull
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.http.content.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.transactions.transaction
import org.jetbrains.exposed.sql.update
import org.noelware.charted.ChartedScope
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.ValidationException
import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.databases.postgres.models.Organization
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.databases.postgres.tables.OrganizationTable
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.remi.support.filesystem.FilesystemStorageService
import java.io.ByteArrayInputStream
import java.io.File

private val acceptableContentTypes: List<String> = listOf("png", "jpeg", "gif").map { "image/$it" }

class DefaultAvatarModule(
    private val storage: StorageHandler,
    private val httpClient: HttpClient
) : AvatarModule {
    init {
        if (storage.service is FilesystemStorageService) {
            val service = storage.service as FilesystemStorageService
            val normalizedPath = File(service.normalizePath("./avatars"))
            if (normalizedPath.exists()) {
                if (normalizedPath.isFile) {
                    normalizedPath.delete()
                    normalizedPath.mkdir()
                }
            } else {
                normalizedPath.mkdirs()
            }
        }
    }

    override suspend fun identicons(id: Long): ByteArray = httpClient.get("https://avatars.dicebear.com/api/identicon/$id.svg")
        .body()

    override suspend fun gravatar(email: String): ByteArray {
        val hash = CryptographyUtils.md5Hex(email)
        return httpClient.get("https://secure.gravatar.com/avatar/$hash.png").body()
    }

    override suspend fun retrieveRepoIcon(repository: Repository, hash: String?): Pair<ContentType, ByteArray>? {
        if (repository.iconHash == null) return ContentType.Image.SVG to identicons(repository.id)
        return search("./repositories/${repository.ownerID}/${repository.id}/icons${hash.ifNotNull { "/$this" } }") {
            if (hash != null && hash == repository.iconHash) {
                transaction {
                    RepositoryTable.update({ RepositoryTable.id eq repository.id }) {
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                        it[iconHash] = null
                    }
                }

                storage.delete("./repositories/${repository.ownerID}/${repository.id}/icons/$hash")
            }
        }
    }

    override suspend fun updateRepoIcon(repository: Repository, part: PartData.FileItem) {
        val (hash, ext) = update("./repositories/${repository.ownerID}/${repository.id}/icons", part)
        asyncTransaction(ChartedScope) {
            RepositoryTable.update({ RepositoryTable.id eq repository.id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                it[iconHash] = "$hash$ext"
            }
        }
    }

    override suspend fun retrieveOrgAvatar(org: Organization, hash: String?): Pair<ContentType, ByteArray>? {
        if (org.iconHash == null) {
            return if (org.gravatarEmail != null) {
                ContentType.Image.PNG to gravatar(org.gravatarEmail!!)
            } else {
                ContentType.Image.SVG to identicons(org.id)
            }
        }

        return search("./avatars/${org.id}${hash.ifNotNull { "/$this" }}") {
            if (hash != null && hash == org.iconHash) {
                transaction {
                    OrganizationTable.update({ OrganizationTable.id eq org.id }) {
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                        it[iconHash] = null
                    }
                }

                storage.delete("./avatars/${org.id}/$hash")
            }
        }
    }

    override suspend fun updateOrgAvatar(org: Organization, part: PartData.FileItem) {
        val (hash, ext) = update("./avatars/${org.id}", part)
        asyncTransaction(ChartedScope) {
            OrganizationTable.update({ OrganizationTable.id eq org.id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                it[iconHash] = "$hash$ext"
            }
        }
    }

    override suspend fun retrieveUserAvatar(user: User, hash: String?): Pair<ContentType, ByteArray>? {
        if (user.avatarHash == null) {
            return if (user.gravatarEmail != null) {
                ContentType.Image.PNG to gravatar(user.gravatarEmail!!)
            } else {
                ContentType.Image.SVG to identicons(user.id)
            }
        }

        return search("./avatars/${user.id}${hash.ifNotNull { "/$this" }}") {
            if (hash != null && hash == user.avatarHash) {
                transaction {
                    UserTable.update({ UserTable.id eq user.id }) {
                        it[avatarHash] = null
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    }
                }

                storage.delete("./avatars/${user.id}/$hash")
            }
        }
    }

    override suspend fun updateUserAvatar(user: User, part: PartData.FileItem) {
        val (hash, ext) = update("./avatars/${user.id}", part)
        asyncTransaction(ChartedScope) {
            UserTable.update({ UserTable.id eq user.id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                it[avatarHash] = "$hash$ext"
            }
        }
    }

    private fun search(path: String, onFailed: () -> Unit): Pair<ContentType, ByteArray>? = storage.open(path)?.ifNotNull {
        val bytes = use { it.readBytes() }
        when (val contentType = storage.service.getContentTypeOf(bytes)) {
            "image/png", "image/jpg", "image/jpeg", "image/gif" ->
                ContentType.parse(contentType) to bytes

            else -> {
                onFailed()
                null
            }
        }
    }

    private fun update(path: String, part: PartData.FileItem): Pair<String, String> {
        val bytes = part.streamProvider().use { it.readBytes() }
        val contentType = storage.service.getContentTypeOf(bytes) ?: "application/octet-stream"
        if (!acceptableContentTypes.contains(contentType)) {
            throw ValidationException("multipart.body.${part.originalFileName ?: "<unknown file>"}", "File was not the correct content type [${acceptableContentTypes.joinToString(", ")}], received [$contentType]")
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
            val file = File(service.normalizePath(path))
            if (file.exists() && file.isFile) {
                file.deleteRecursively()
                file.mkdirs()
            } else {
                file.mkdirs()
            }
        }

        storage.upload("$path/$hash$ext", ByteArrayInputStream(bytes), contentType)
        part.dispose()

        return hash to ext
    }
}
