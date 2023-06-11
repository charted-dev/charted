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

package org.noelware.charted.modules.helm.charts

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.decodeFromStream
import com.charleskorn.kaml.encodeToStream
import dev.floofy.utils.slf4j.logging
import io.github.z4kn4fein.semver.VersionFormatException
import io.github.z4kn4fein.semver.toVersion
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.utils.io.core.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.datetime.Clock
import kotlinx.serialization.encodeToString
import org.apache.commons.compress.archivers.tar.TarArchiveEntry
import org.apache.commons.compress.archivers.tar.TarArchiveInputStream
import org.apache.commons.compress.utils.IOUtils
import org.noelware.charted.KtorHttpRespondException
import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.common.types.helm.ChartIndexSpec
import org.noelware.charted.common.types.helm.ChartIndexYaml
import org.noelware.charted.common.types.helm.ChartSpec
import org.noelware.charted.common.types.responses.ApiError
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.toApiBaseUrl
import org.noelware.charted.configuration.kotlin.dsl.toCdnBaseUrl
import org.noelware.charted.modules.helm.charts.buildables.UploadReleaseTarball
import org.noelware.charted.modules.storage.StorageModule
import org.noelware.remi.core.Blob
import org.noelware.remi.core.ListBlobsRequest
import org.noelware.remi.support.filesystem.FilesystemStorageService
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.File
import java.io.InputStream
import java.util.zip.GZIPInputStream
import kotlin.io.use
import kotlin.text.toByteArray

val acceptableContentTypes: List<String> = listOf("gzip", "tar+gzip", "tar").map { "application/$it" }
private val allowedFilesRegex = """(Chart.lock|Chart.ya?ml|values.ya?ml|[.]helmignore|templates/\w+.*[.](txt|tpl|ya?ml)|charts/\w+.*.(tgz|tar.gz))""".toRegex()
private val exemptedFiles = listOf("values.schema.json")

class DefaultHelmChartModule(
    private val storage: StorageModule,
    private val config: Config,
    private val yaml: Yaml
): HelmChartModule {
    private val log by logging<DefaultHelmChartModule>()

    init {
        if (storage.service is FilesystemStorageService) {
            log.info("Creating missing directories if any...")
            for (dir in listOf("metadata", "repositories")) {
                val directoryName = (storage.service as FilesystemStorageService).normalizePath("./$dir")
                val dirFile = File(directoryName)
                if (dirFile.exists()) continue

                if (dirFile.isFile) {
                    log.warn("File [$dirFile] was a file, deleting and marking it as a directory")
                    dirFile.delete()
                    dirFile.mkdirs()

                    continue
                }

                dirFile.mkdir()
            }
        }
    }

    /**
     * Returns the latest SemVer version that is available to look-up. If [allowPrereleases] is set to
     * `true`, then it will attempt to look up pre-releases (i.e, `0.1.2-alpha.3`).
     *
     * @param owner repository owner
     * @param repo repository ID
     * @param allowPrereleases If pre-releases should be allowed in this lookup or not
     * @return latest, valid SemVer version available, or `null` if not available.
     */
    override suspend fun getLatestVersion(owner: Long, repo: Long, allowPrereleases: Boolean): String? {
        if (!storage.exists("./repositories/$owner/$repo")) return null

        val files: List<Blob> = storage.service.blobs(
            ListBlobsRequest.Builder().apply {
                withPrefix("./repositories/$owner/$repo/tarballs")
            }.build(),
        )

        return files
            .asSequence()
            .mapNotNull {
                try {
                    it.name().toVersion(false)
                } catch (e: VersionFormatException) {
                    null
                }
            }
            .sortedWith(Comparator(io.github.z4kn4fein.semver.Version::compareTo))
            .filter {
                if (allowPrereleases) {
                    true
                } else {
                    it.isStable
                }
            }
            .map { it.toString() }
            .firstOrNull()
    }

    /**
     * Returns the `index.yaml` contents serialized as [ChartIndexYaml] with the given user.
     * @param owner repository owner
     * @return [ChartIndexYaml] object if it exists, `null` if not.
     */
    override suspend fun getIndexYaml(owner: Long): ChartIndexYaml? = storage.open("./metadata/$owner/index.yaml")?.use {
        yaml.decodeFromStream(it)
    }

    /**
     * Creates an `index.yaml` for the repository
     * @param owner repository owner
     */
    override suspend fun createIndexYaml(owner: Long) {
        log.info("Creating index.yaml for user [$owner]")
        if (storage.service is FilesystemStorageService) {
            val folder = File((storage.service as FilesystemStorageService).normalizePath("./metadata/$owner"))
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
    override suspend fun getReleaseTarball(owner: Long, repo: Long, version: String): InputStream? = storage.open("./repositories/$owner/$repo/tarballs/$version.tar.gz")

    /**
     * Upload a release tarball that can be downloaded from the following locations:
     *
     *    - (if `config.storage.alias_host` is set):  $STORAGE_ALIAS_HOST/{users|organizations}/{id}/releases/{version}.tar.gz
     *    - (if `config.cdn.enabled` is set to true): $SERVER_URL/{cdn_prefix}/{users|organizations}/{id}/releases/{version}.tar.gz
     *    - (mapped from storage handler):            $ROOT/{users|organizations}/{id}/releases/{version}.tar.gz
     *    - (repositories api):                       $SERVER_URL/repositories/{id}/releases/{version}/{version}.tar.gz
     *
     * @param uploadDataDsl The [UploadReleaseTarball] builder DSL to use to
     * identify the data that is being uploaded.
     */
    override suspend fun uploadReleaseTarball(uploadDataDsl: UploadReleaseTarball.Builder.() -> Unit) {
        val uploadData = UploadReleaseTarball.Builder().apply(uploadDataDsl).build()
        val (_, tarballFile, version, owner, repo) = uploadData

        log.info("Uploading release tarball $version.tar.gz to repository [$owner/${repo.name}]")

        // Disallow overwriting the tarball -- just delete the release and then re-do the upload. The Helm plugin
        // will do it for you if you provide the `--overwrite` flag when you use `helm charted push`.
        if (storage.exists("./repositories/$owner/${repo.id}/tarballs/$version.tar.gz")) {
            throw KtorHttpRespondException(
                HttpStatusCode.Conflict,
                ApiError("TARBALL_ALREADY_EXISTS", "Tarball for version '$version' already exists"),
            )
        }

        // First, we need to get the data itself. This will determine if the tarball
        // sent to us was actually a tarball or not.
        val inputStream = tarballFile.streamProvider()
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        val contentType = storage.service.getContentTypeOf(data)
        if (!acceptableContentTypes.contains(contentType)) {
            throw IllegalArgumentException("File provided was not any of [${acceptableContentTypes.joinToString(", ")}], received $contentType")
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
        val tarInputStream = TarArchiveInputStream(
            withContext(Dispatchers.IO) {
                GZIPInputStream(ByteArrayInputStream(data))
            },
        )

        var chartSpec: ChartSpec? = null
        var nextEntry: TarArchiveEntry?
        while (true) {
            nextEntry = tarInputStream.nextTarEntry
            if (nextEntry == null) break

            val firstDash = nextEntry.name.indexOfFirst { c -> c == '/' }
            val entryName = if (firstDash != -1) {
                nextEntry.name.substring(firstDash + 1)
            } else {
                nextEntry.name
            }

            // Check if it doesn't follow the regular expression
            if (!(entryName matches allowedFilesRegex)) {
                // If it contains any exempted files (that are allowed),
                // then allow it
                if (exemptedFiles.contains(entryName)) continue

                // Otherwise, just heck off
                throw IllegalStateException("Entry ${nextEntry.name} (~${nextEntry.size} bytes) is not allowed")
            }

            if (entryName == "Chart.yaml") {
                chartSpec = ByteArrayInputStream(IOUtils.toByteArray(tarInputStream)).use { stream -> yaml.decodeFromStream(stream) }
            }
        }

        if (chartSpec == null) {
            throw IllegalStateException("Corrupt tar file: missing `Chart.yaml` file")
        }

        if (storage.service is FilesystemStorageService) {
            val tarballPath = (storage.service as FilesystemStorageService).normalizePath("./repositories/$owner/${repo.id}/tarballs")
            val file = File(tarballPath)

            if (!file.exists()) {
                file.mkdirs()
            }
        }

        // Now, we should be allowed to upload it
        storage.upload(
            "./repositories/$owner/${repo.id}/tarballs/$version.tar.gz",
            ByteArrayInputStream(data),
            "application/tar+gzip",
        )

        val indexYaml = getIndexYaml(owner)!!
        val entries = indexYaml.entries.toMutableMap()
        val host = config.toApiBaseUrl("/repositories/${repo.id}/releases/$version.tar.gz")
        val cdnBase = config.toCdnBaseUrl("/repositories/${repo.id}/releases/$version.tar.gz")

        val digest = CryptographyUtils.sha256(ByteArrayInputStream(IOUtils.toByteArray(tarInputStream)))
        if (!entries.containsKey(repo.name)) {
            entries[repo.name] = listOf(
                ChartIndexSpec.fromSpec(
                    listOf(host, cdnBase),
                    Clock.System.now(),
                    false,
                    digest,
                    chartSpec,
                ),
            )
        } else {
            entries[repo.name] = entries[repo.name]!! + listOf(
                ChartIndexSpec.fromSpec(
                    listOf(host, cdnBase),
                    Clock.System.now(),
                    false,
                    digest,
                    chartSpec,
                ),
            )
        }

        // Upload updated index.yaml file
        val stream = ByteArrayOutputStream()
        yaml.encodeToStream(indexYaml.copy(entries = entries, generated = Clock.System.now()), stream)

        storage.upload(
            "./metadata/$owner/index.yaml",
            ByteArrayInputStream(stream.toByteArray()),
            "application/yaml",
        )
    }

    /**
     * Retrieves a template from the given [repository][repo] and returns an [InputStream] that is
     * used to send the data to the end user.
     * @param owner     owner ID
     * @param repo      repository object
     * @param version   release id
     * @param template  template file to render
     */
    override suspend fun getTemplate(owner: Long, repo: Long, version: String, template: String): InputStream? {
        log.info("Finding template [$template] for repository $owner/$repo v$version")

        val inputStream = storage.open("./repositories/$owner/$repo/tarballs/$version.tar.gz") ?: return null
        val tarInputStream = TarArchiveInputStream(
            withContext(Dispatchers.IO) {
                GZIPInputStream(inputStream)
            },
        )

        var data: InputStream? = null
        tarInputStream.use { stream ->
            var nextEntry: TarArchiveEntry?
            while (true) {
                nextEntry = stream.nextTarEntry
                if (nextEntry == null) break

                val firstDash = nextEntry.name.indexOfFirst { c -> c == '/' }
                val entryName = if (firstDash != -1) {
                    nextEntry.name.substring(firstDash + 1)
                } else {
                    nextEntry.name
                }

                if (entryName == "templates/$template") {
                    data = ByteArrayInputStream(IOUtils.toByteArray(stream))
                    break
                }
            }
        }

        return data
    }

    /**
     * Returns all the templates from a given repository's release tarball. The list contains
     * endpoint URLs that are used to access the template's data itself.
     *
     * @param owner     owner ID
     * @param repo      repository object
     * @param version   release id
     */
    override suspend fun getAllTemplates(owner: Long, repo: Long, version: String): List<String> {
        val templates = mutableListOf<String>()

        log.info("Finding all templates in repository [$owner/$repo v$version]")
        val inputStream = storage.open("./repositories/$owner/$repo/tarballs/$version.tar.gz") ?: return emptyList()
        val tarInputStream = TarArchiveInputStream(
            withContext(Dispatchers.IO) {
                GZIPInputStream(inputStream)
            },
        )

        val host = config.storage.hostAlias ?: config.baseUrl ?: "http${if (config.server.ssl != null) "s" else ""}://${config.server.host}:${config.server.port}"
        return tarInputStream.use { stream ->
            var nextEntry: TarArchiveEntry?
            while (true) {
                nextEntry = stream.nextTarEntry
                if (nextEntry == null) break

                val firstDash = nextEntry.name.indexOfFirst { c -> c == '/' }
                val entryName = if (firstDash != -1) {
                    nextEntry.name.substring(firstDash + 1)
                } else {
                    nextEntry.name
                }

                templates.add("$host/repositories/$repo/templates/$entryName")
            }

            templates
        }
    }

    /**
     * Retrieves the `Chart.yaml` file from the given [repository][repo] and returns an [ByteArray] that is
     * used to send the data to the end user.
     * @param owner     owner ID
     * @param repo      repository object
     * @param version   release id
     */
    override suspend fun getChartYaml(owner: Long, repo: Long, version: String): InputStream? {
        log.info("Finding Chart.yaml for repository $owner/$repo v$version")

        val inputStream = storage.open("./repositories/$owner/$repo/tarballs/$version.tar.gz") ?: return null
        val tarInputStream = TarArchiveInputStream(
            withContext(Dispatchers.IO) {
                GZIPInputStream(inputStream)
            },
        )

        var data: InputStream? = null
        tarInputStream.use { stream ->
            var nextEntry: TarArchiveEntry?
            while (true) {
                nextEntry = stream.nextTarEntry
                if (nextEntry == null) break

                val firstDash = nextEntry.name.indexOfFirst { c -> c == '/' }
                val entryName = if (firstDash != -1) {
                    nextEntry.name.substring(firstDash + 1)
                } else {
                    nextEntry.name
                }

                if (entryName == "Chart.yaml") {
                    data = ByteArrayInputStream(IOUtils.toByteArray(stream))
                    break
                }
            }
        }

        return data
    }

    /**
     * Retrieves the `values.yaml` file from the given [repository][repo] and returns an [ByteArray] that is
     * used to send the data to the end user.
     * @param owner     owner ID
     * @param repo      repository object
     * @param version   release id
     */
    override suspend fun getValuesYaml(owner: Long, repo: Long, version: String): InputStream? {
        log.info("Finding values.yaml for repository $owner/$repo v$version")

        val inputStream = storage.open("./repositories/$owner/$repo/tarballs/$version.tar.gz") ?: return null
        val tarInputStream = TarArchiveInputStream(
            withContext(Dispatchers.IO) {
                GZIPInputStream(inputStream)
            },
        )

        var data: InputStream? = null
        tarInputStream.use { stream ->
            var nextEntry: TarArchiveEntry?
            while (true) {
                nextEntry = stream.nextTarEntry
                if (nextEntry == null) break

                val firstDash = nextEntry.name.indexOfFirst { c -> c == '/' }
                val entryName = if (firstDash != -1) {
                    nextEntry.name.substring(firstDash + 1)
                } else {
                    nextEntry.name
                }

                if (entryName == "values.yaml") {
                    data = ByteArrayInputStream(IOUtils.toByteArray(stream))
                    break
                }
            }
        }

        return data
    }

    override suspend fun deleteReleaseTarball(owner: Long, repo: Long, version: String) {
        if (!storage.exists("./repositories/$owner/$repo/tarballs/$version.tar.gz")) return

        val wasDeleted = storage.delete("./repositories/$owner/$repo/tarballs/$version.tar.gz")
        if (!wasDeleted) {
            log.warn("Unable to delete repository tarball release in [./repositories/$owner/$repo/tarballs/$version.tar.gz]")
        }
    }
}
