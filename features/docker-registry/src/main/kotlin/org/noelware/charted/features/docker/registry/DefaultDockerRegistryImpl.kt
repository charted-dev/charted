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

package org.noelware.charted.features.docker.registry

import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import org.noelware.charted.core.StorageWrapper
import org.noelware.remi.filesystem.FilesystemStorageTrailer
import java.nio.file.Files
import java.nio.file.Path

class DefaultDockerRegistryImpl(private val storage: StorageWrapper): DockerRegistry {
    private val log by logging<DefaultDockerRegistryImpl>()

    init {
        log.info("Deleting chart-specific directories...")
        log.warn("THIS WILL DELETE ANY PREVIOUS CHART INSTALLATIONS, PLEASE KEEP CAUTION OF THAT!!!!")

        // Delete the `metadata/` and `tarballs/` data due to the
        // server not using the Charts engine.
        runBlocking {
            storage.delete("./metadata")
            storage.delete("./tarballs")
        }

        log.info("Creating layers/ directory")
        runBlocking {
            if (storage.trailer is FilesystemStorageTrailer) {
                val path = (storage.trailer as FilesystemStorageTrailer).normalizePath("./layers")
                withContext(Dispatchers.IO) {
                    Files.createDirectories(Path.of(path))
                }
            }
        }
    }

    override fun close() {
        // TODO: this
    }
}
