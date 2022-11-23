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

package org.noelware.charted.modules.helm.charts

import co.elastic.apm.api.Traced
import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.decodeFromStream
import dev.floofy.utils.slf4j.logging
import io.ktor.http.content.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.encodeToString
import org.apache.commons.compress.archivers.tar.TarArchiveInputStream
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.types.helm.ChartIndexYaml
import org.noelware.remi.core.figureContentType
import org.noelware.remi.filesystem.FilesystemStorageTrailer
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.File
import java.io.InputStream

private val ACCEPTABLE_TAR_CONTENT_TYPES: List<String> = listOf("gzip", "tar+gzip", "tar").map { "application/$it" }
private val ALLOWED_FILES_REGEX = "\\/?(Chart.lock|Chart.ya?ml|index.ya?ml|\\.helmignore|\\/?templates\\/.*\\.(txt|tpl|ya?ml)|\\/?charts\\/.*\\.(tgz|tar\\.gz))".toRegex()
private val EXEMPTED_FILES = listOf("values.schema.json")

class DefaultHelmChartModule(
    private val storage: StorageHandler,
    private val yaml: Yaml
): HelmChartModule {
    private val log by logging<DefaultHelmChartModule>()

    /**
     * Returns the `index.yaml` contents serialized as [ChartIndexYaml] with the given user.
     * @param owner repository owner
     * @return [ChartIndexYaml] object if it exists, `null` if not.
     */
    @Traced
    override suspend fun getIndexYaml(owner: Long): ChartIndexYaml? = storage.open("./metadata/$owner/index.yaml")?.use {
        yaml.decodeFromStream(it)
    }

    /**
     * Creates an `index.yaml` for the repository
     * @param owner repository owner
     */
    @Traced
    override suspend fun createIndexYaml(owner: Long) {
        if (storage.trailer is FilesystemStorageTrailer) {
            val folder = File((storage.trailer as FilesystemStorageTrailer).normalizePath("./metadata/$owner"))
            if (!folder.exists()) folder.mkdirs()
        }

        val index = ChartIndexYaml()
        val serialized = yaml.encodeToString(index).toByteArray()
        storage.upload("./metadata/$owner/index.yaml", ByteArrayInputStream(serialized), "application/yaml")
    }

    /**
     * Deletes the `index.yaml` file.
     */
    override suspend fun destroyIndexYaml(owner: Long) {
        storage.delete("./metadata/$owner/index.yaml")
    }

    /**
     * Returns a release tarball from the specified [owner] and [repository][repo] IDs and the
     * specified version.
     *
     * @param owner   owner ID
     * @param repo    repository ID
     * @param version release version to fetch from
     * @return [InputStream] of the tarball itself, returns `null` if it was not found
     */
    override suspend fun getReleaseTarball(owner: Long, repo: Long, version: String): InputStream? = storage.open("./tarballs/$owner/$repo/$version.tar.gz")

    /**
     * Upload a release tarball that can be downloaded from the following locations:
     *
     *    - (if `config.storage.alias_host` is set):  $STORAGE_ALIAS_HOST/{users|organizations}/{id}/releases/{version}.tar.gz
     *    - (if `config.cdn.enabled` is set to true): $SERVER_URL/{cdn_prefix}/{users|organizations}/{id}/releases/{version}.tar.gz
     *    - (mapped from storage handler):            $ROOT/{users|organizations}/{id}/releases/{version}.tar.gz
     *    - (repositories api):                       $SERVER_URL/repositories/{id}/releases/{version}/{version}.tar.gz
     *
     * @param owner     owner ID
     * @param repo      repository ID
     * @param version   release version
     * @param multipart multipart/form-data packet to store
     */
    override suspend fun uploadReleaseTarball(owner: Long, repo: Long, version: String, multipart: PartData.FileItem) {
        log.info("Uploading release tarball $version.tar.gz to repository [$owner/$repo]")

        // First, we need to get the data itself. This will determine if the tarball
        // sent to us was actually a tarball or not.
        val inputStream = multipart.streamProvider()
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        val contentType = storage.trailer.figureContentType(data)
        if (!ACCEPTABLE_TAR_CONTENT_TYPES.contains(contentType)) {
            throw IllegalArgumentException("File provided was not any of [${ACCEPTABLE_TAR_CONTENT_TYPES.joinToString(", ")}], received $contentType")
        }

        // Now, we need to actually see if it's in the Helm Chart structure. It should be something
        // like:
        //
        // $ <CHART>/Chart.yaml
        // $ <CHART>/index.yaml
        // $ <CHART>/templates/*.{txt,tpl,yaml,yml}
        // $ <CHART>/charts/*.tgz
        // $ <CHART>/.helmignore
        //
        // We won't validate all the dependencies (since it will take a while, and we do
        // not want to add a lot of overhead).
        val tarInputStream = TarArchiveInputStream(ByteArrayInputStream(data))
        tarInputStream.use {
            while (true) {
                // Get the archive entry, if it is null, then we'll assume
                // it's the end of the entry list.
                val nextEntry = it.nextEntry ?: break

                // Check if it doesn't follow the regular expression
                if (!(nextEntry.name matches ALLOWED_FILES_REGEX)) {
                    // If it contains any exempted files (that are allowed),
                    // then allow it
                    if (EXEMPTED_FILES.contains(nextEntry.name)) continue

                    // Otherwise, just heck off
                    throw IllegalStateException("Entry ${nextEntry.name} (~${nextEntry.size} bytes) is not allowed")
                }
            }
        }

        if (storage.trailer is FilesystemStorageTrailer) {
            val tarballPath = (storage.trailer as FilesystemStorageTrailer).normalizePath("./tarballs/$owner/$repo")
            val tarballFile = File(tarballPath)

            if (!tarballFile.exists()) {
                tarballFile.mkdirs()
            }
        }

        // Now, we should be allowed to upload it
        storage.upload(
            "./tarballs/$owner/$repo/$version.tar.gz",
            ByteArrayInputStream(data),
            "application/tar+gzip"
        )
    }
}
