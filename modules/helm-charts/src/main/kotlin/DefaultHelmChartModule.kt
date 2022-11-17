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
import kotlinx.serialization.encodeToString
import org.noelware.charted.databases.postgres.models.Organization
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.types.helm.ChartIndexYaml
import org.noelware.remi.filesystem.FilesystemStorageTrailer
import java.io.ByteArrayInputStream
import java.io.File
import java.nio.charset.Charset

class DefaultHelmChartModule(
    private val storage: StorageHandler,
    private val yaml: Yaml
): HelmChartModule {
    private val log by logging<DefaultHelmChartModule>()

    /**
     * Returns the `index.yaml` contents serialized as [ChartIndexYaml] with the given user.
     * @param user user that owns the `index.yaml` file
     * @return [ChartIndexYaml] object if it exists, `null` if not.
     */
    @Traced
    override suspend fun getIndexYaml(user: User): ChartIndexYaml? = storage.open("./users/${user.id}/index.yaml")?.use {
        yaml.decodeFromStream(it)
    }

    /**
     * Returns the `index.yaml` contents serialized as [ChartIndexYaml] with the given organization.
     * @param organization organization that owns the `index.yaml` file
     * @return [ChartIndexYaml] object if it exists, `null` if not.
     */
    @Traced
    override suspend fun getIndexYaml(organization: Organization): ChartIndexYaml? = storage.open("./organizations/${organization.id}/index.yaml")?.use {
        yaml.decodeFromStream(it)
    }

    /**
     * Creates an `index.yaml` for a user or organization
     * @param type "user" or "organization"
     * @param id   the id that this index.yaml belongs to
     */
    @Traced
    override suspend fun createIndexYaml(type: String, id: Long) {
        check(listOf("user", "organization").contains(type)) { "expecting `user` or `organization`" }

        log.info("writing index.yaml for $type $id!")
        if (storage.trailer is FilesystemStorageTrailer) {
            val folder = File((storage.trailer as FilesystemStorageTrailer).normalizePath("./${type}s/$id"))
            folder.mkdirs()
        }

        val index = ChartIndexYaml()
        val serialized = yaml.encodeToString(index)
        storage.upload(
            "./${type}s/$id/index.yaml",
            ByteArrayInputStream(serialized.toByteArray(Charset.defaultCharset())),
            "text/yaml; charset=utf-8"
        ).also {
            log.info("wrote index.yaml file for $type $id")
        }
    }

    /**
     * Deletes the `index.yaml` file of the [user] specified
     */
    override suspend fun destroyIndexYaml(user: User) {
        storage.delete("./users/${user.id}/index.yaml")
    }

    /**
     * Deletes the `index.yaml` file of the [organization] specified
     */
    override suspend fun destroyIndexYaml(organization: Organization) {
    }

    /**
     * Returns a release tarball from the specified [owner] and [repository][repo] IDs and the
     * specified version.
     *
     * @param owner   owner ID
     * @param repo    repository ID
     * @param version release version to fetch from
     */
    override suspend fun getReleaseTarball(owner: Long, repo: Long, version: String): ByteArrayInputStream {
        TODO("Not yet implemented")
    }

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
    override suspend fun uploadReleaseTarball(owner: Long, repo: Long, version: String, multipart: PartData) {
        TODO("Not yet implemented")
    }
}
