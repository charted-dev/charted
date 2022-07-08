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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core

import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.runBlocking
import org.noelware.charted.common.data.StorageConfig
import org.noelware.remi.core.StorageTrailer
import org.noelware.remi.filesystem.FilesystemStorageTrailer
import org.noelware.remi.minio.MinIOStorageTrailer
import org.noelware.remi.s3.S3StorageTrailer
import java.io.File
import java.io.InputStream

class StorageWrapper(config: StorageConfig) {
    private val log by logging<StorageWrapper>()
    val trailer: StorageTrailer<*>

    init {
        trailer = when {
            config.filesystem != null -> FilesystemStorageTrailer(config.filesystem!!)
            config.minio != null -> MinIOStorageTrailer(config.minio!!)
            config.fs != null -> FilesystemStorageTrailer(config.fs!!)
            config.s3 != null -> S3StorageTrailer(config.s3!!)
            else -> error("Missing `filesystem`, `minio`, `fs`, or `s3` configuration")
        }

        log.info("Using storage provider [${trailer.name}]")
        runBlocking {
            try {
                log.info("Setting up trailer...")
                trailer.init()
            } catch (e: Exception) {
                if (e !is IllegalStateException && e.message?.contains("doesn't support StorageTrailer#init/0") == false) {
                    throw e
                }
            }

            if (trailer is FilesystemStorageTrailer) {
                for (folder in listOf("./avatars", "./tarballs", "./metadata")) {
                    log.warn("Directory doesn't exist: [${trailer.normalizePath(folder)}]")
                    File(trailer.normalizePath(folder)).mkdirs()
                }
            }
        }
    }

    /**
     * Opens a file under the [path] and returns the [InputStream] of the file.
     */
    suspend fun open(path: String): InputStream? = trailer.open(path)

    /**
     * Deletes the file under the [path] and returns a [Boolean] if the
     * operation was a success or not.
     */
    suspend fun delete(path: String): Boolean = trailer.delete(path)

    /**
     * Checks if the file exists under this storage trailer.
     * @param path The path to find the file.
     */
    suspend fun exists(path: String): Boolean = trailer.exists(path)

    /**
     * Uploads file to this storage trailer and returns a [Boolean] result
     * if the operation was a success or not.
     *
     * @param path The path to upload the file to
     * @param stream The [InputStream] that represents the raw data.
     * @param contentType The content type of the file (useful for S3 and GCS support)!
     */
    suspend fun upload(
        path: String,
        stream: InputStream,
        contentType: String = "application/octet-stream"
    ): Boolean = trailer.upload(path, stream, contentType)
}
