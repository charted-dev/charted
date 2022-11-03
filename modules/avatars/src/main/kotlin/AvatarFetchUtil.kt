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

package org.noelware.charted.modules.avatars

import dev.floofy.utils.koin.inject

/**
 * Auxiliary utility for handling avatar storage.
 */
object AvatarFetchUtil {
    private val module: AvatarModule by inject()

    fun retrieve(id: String): String? = null
    fun update(id: String, data: ByteArray) = Unit
}

/*
    /**
 * Returns a byte-array of the image of the specified user and the content type. Defaults to
 * identicons if the user doesn't have an avatar OR they didn't specify
 * a Gravatar email.
 *
 * This method returns null if the user couldn't be found.
 */
    suspend fun get(user: User, hash: String? = null): Pair<ContentType, ByteArray> {
        // If we can't get the user's avatar, then we should specify
        // if we should use Gravatar or Identicon
        if (user.avatarHash == null) {
            return if (user.gravatarEmail != null) {
                ContentType.Image.PNG to gravatar.fetch(user.gravatarEmail!!)
            } else {
                ContentType.Image.SVG to identicon.fetch(user.id)
            }
        }

        if (hash != null) {
            val stream = storage.trailer.open("./avatars/${user.id}/$hash") ?: return if (user.gravatarEmail != null) {
                ContentType.Image.PNG to gravatar.fetch(user.gravatarEmail!!)
            } else {
                ContentType.Image.SVG to identicon.fetch(user.id)
            }

            val bytes = stream.readBytes()

            // We already read through the stream, so let's just close it
            // and not catch the IO exceptions.
            stream.closeQuietly()
            return when (val contentType = storage.trailer.figureContentType(bytes)) {
                ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> ContentType.parse(contentType) to bytes
                else -> {
                    asyncTransaction(ChartedScope) {
                        UserTable.update({ UserTable.id eq user.id.toLong() }) {
                            it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                            it[avatarHash] = null
                        }
                    }

                    storage.trailer.delete("./avatars/${user.id}/$hash")
                    if (user.gravatarEmail != null) {
                        ContentType.Image.PNG to gravatar.fetch(user.gravatarEmail!!)
                    } else {
                        ContentType.Image.SVG to identicon.fetch(user.id)
                    }
                }
            }
        }

        val stream = storage.trailer.open("./avatars/${user.id}/${user.avatarHash}") ?: return if (user.gravatarEmail != null) {
            ContentType.Image.PNG to gravatar.fetch(user.gravatarEmail!!)
        } else {
            ContentType.Image.SVG to identicon.fetch(user.id)
        }

        val bytes = stream.readBytes()

        // We already read through the stream, so let's just close it
        // and not catch the IO exceptions.
        stream.closeQuietly()
        return when (val contentType = storage.trailer.figureContentType(bytes)) {
            ContentType.Image.PNG.toString(), ContentType.Image.GIF.toString(), ContentType.Image.JPEG.toString() -> ContentType.parse(contentType) to bytes

            // If for some reason that the data was not an image from the content type,
            // let's just delete it *for now*.
            else -> {
                asyncTransaction(ChartedScope) {
                    UserTable.update({ UserTable.id eq user.id.toLong() }) {
                        it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                        it[avatarHash] = null
                    }
                }

                storage.trailer.delete("./avatars/${user.id}/${user.avatarHash}")
                if (user.gravatarEmail != null) {
                    ContentType.Image.PNG to gravatar.fetch(user.gravatarEmail!!)
                } else {
                    ContentType.Image.SVG to identicon.fetch(user.id)
                }
            }
        }
    }

    suspend fun update(user: User, part: PartData.FileItem) {
        val bytes = part.streamProvider().use { it.readBytes() }
        val contentType = storage.trailer.figureContentType(bytes)
        if (!ACCEPTED_CONTENT_TYPES.contains(contentType)) {
            throw IllegalArgumentException("File provided was not any of [${ACCEPTED_CONTENT_TYPES.joinToString(", ")}], received [$contentType]")
        }

        val hash = RandomGenerator.generate(8)
        val ext = when {
            contentType.startsWith("image/jpg") || contentType.startsWith("image/jpeg") -> ".jpg"
            contentType.startsWith("image/png") -> ".png"
            contentType.startsWith("image/gif") -> ".gif"
            else -> throw AssertionError("ext != png/jpg/gif when passed through.")
        }

        storage.trailer.upload("./avatars/${user.id}/$hash$ext", ByteArrayInputStream(bytes), contentType)
        part.dispose()

        asyncTransaction(ChartedScope) {
            UserTable.update({ UserTable.id eq user.id.toLong() }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                it[avatarHash] = "$hash$ext"
            }
        }
    }

    companion object {
        private val ACCEPTED_CONTENT_TYPES: List<String> = listOf("png", "jpeg", "jpg", "gif").map { "image/$it" }
    }
 */
