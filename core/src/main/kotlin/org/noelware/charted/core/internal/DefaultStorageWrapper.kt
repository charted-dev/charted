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

package org.noelware.charted.core.internal

import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.runBlocking
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.configuration.dsl.StorageConfig
import org.noelware.charted.core.StorageWrapper
import org.noelware.remi.core.StorageTrailer
import org.noelware.remi.filesystem.FilesystemStorageTrailer
import org.noelware.remi.minio.MinIOStorageTrailer
import org.noelware.remi.s3.S3StorageTrailer
import java.io.File

class DefaultStorageWrapper(config: StorageConfig): StorageWrapper {
    private val _trailer: SetOnceGetValue<StorageTrailer<*>> = SetOnceGetValue()
    private val log by logging<DefaultStorageWrapper>()

    override val trailer: StorageTrailer<*>
        get() = _trailer.value

    init {
        _trailer.value = when {
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
                    val file = File((trailer as FilesystemStorageTrailer).normalizePath(folder))
                    if (!file.exists()) {
                        log.warn("Directory [${(trailer as FilesystemStorageTrailer).normalizePath(folder)}] doesn't exist!")
                        file.mkdirs()
                    }

                    if (!file.isDirectory) {
                        log.warn("Directory [${(trailer as FilesystemStorageTrailer).normalizePath(folder)}] is not a valid directory.")
                        file.deleteRecursively()
                        file.mkdirs()
                    }
                }
            }
        }
    }
}
