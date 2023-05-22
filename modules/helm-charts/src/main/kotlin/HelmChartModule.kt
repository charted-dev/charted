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

import io.ktor.http.content.*
import org.noelware.charted.common.types.helm.ChartIndexYaml
import org.noelware.charted.modules.helm.charts.buildables.UploadReleaseTarball
import java.io.InputStream

interface HelmChartModule {
    /**
     * Returns the latest SemVer version that is available to look-up. If [allowPrereleases] is set to
     * `true`, then it will attempt to look up pre-releases (i.e, `0.1.2-alpha.3`).
     *
     * @param owner repository owner
     * @param repo repository ID
     * @param allowPrereleases If pre-releases should be allowed in this lookup or not
     * @return latest, valid SemVer version available, or `null` if not available.
     */
    suspend fun getLatestVersion(owner: Long, repo: Long, allowPrereleases: Boolean = false): String?

    /**
     * Returns the `index.yaml` contents serialized as [ChartIndexYaml] with the given user.
     * @param owner repository owner
     * @return [ChartIndexYaml] object if it exists, `null` if not.
     */
    suspend fun getIndexYaml(owner: Long): ChartIndexYaml?

    /**
     * Creates an `index.yaml` for the repository
     * @param owner repository owner
     */
    suspend fun createIndexYaml(owner: Long)

    /**
     * Deletes the `index.yaml` file.
     */
    suspend fun destroyIndexYaml(owner: Long)

    /**
     * Returns a release tarball from the specified [owner] and [repository][repo] IDs and the
     * specified version.
     *
     * @param owner   owner ID
     * @param repo    repository ID
     * @param version release version to fetch from
     * @return [InputStream] of the tarball itself, returns `null` if it was not found
     */
    suspend fun getReleaseTarball(owner: Long, repo: Long, version: String): InputStream?

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
    suspend fun uploadReleaseTarball(uploadDataDsl: UploadReleaseTarball.Builder.() -> Unit)

    /**
     * Retrieves a template from the given [repository][repo] and returns an [InputStream] that is
     * used to send the data to the end user.
     * @param owner     owner ID
     * @param repo      repository object
     * @param version   release id
     * @param template  template file to render
     */
    suspend fun getTemplate(owner: Long, repo: Long, version: String, template: String): InputStream?

    /**
     * Returns all the templates from a given repository's release tarball. The list contains
     * endpoint URLs that are used to access the template's data itself.
     *
     * @param owner     owner ID
     * @param repo      repository object
     * @param version   release id
     */
    suspend fun getAllTemplates(owner: Long, repo: Long, version: String): List<String>

    /**
     * Retrieves the `values.yaml` file from the given [repository][repo] and returns an [InputStream] that is
     * used to send the data to the end user.
     * @param owner     owner ID
     * @param repo      repository object
     * @param version   release id
     */
    suspend fun getValuesYaml(owner: Long, repo: Long, version: String): InputStream?

    /**
     * Retrieves the `Chart.yaml` file from the given [repository][repo] and returns an [InputStream] that is
     * used to send the data to the end user.
     * @param owner     owner ID
     * @param repo      repository object
     * @param version   release id
     */
    suspend fun getChartYaml(owner: Long, repo: Long, version: String): InputStream?

    /**
     * Deletes a release tarball from the storage service, if we can
     * @param owner   owner ID
     * @param repo    repository id
     * @param version release version
     */
    suspend fun deleteReleaseTarball(owner: Long, repo: Long, version: String)
}
