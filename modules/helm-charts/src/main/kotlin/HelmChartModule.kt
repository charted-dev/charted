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

import io.ktor.http.content.*
import org.noelware.charted.databases.postgres.models.Organization
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.types.helm.ChartIndexYaml
import java.io.ByteArrayInputStream

interface HelmChartModule {
    /**
     * Returns the `index.yaml` contents serialized as [ChartIndexYaml] with the given user.
     * @param user user that owns the `index.yaml` file
     * @return [ChartIndexYaml] object if it exists, `null` if not.
     */
    suspend fun getIndexYaml(user: User): ChartIndexYaml?

    /**
     * Returns the `index.yaml` contents serialized as [ChartIndexYaml] with the given organization.
     * @param organization organization that owns the `index.yaml` file
     * @return [ChartIndexYaml] object if it exists, `null` if not.
     */
    suspend fun getIndexYaml(organization: Organization): ChartIndexYaml?

    /**
     * Creates an `index.yaml` for a user or organization
     * @param type "user" or "organization"
     * @param id   the id that this index.yaml belongs to
     */
    suspend fun createIndexYaml(type: String, id: Long)

    /**
     * Deletes the `index.yaml` file of the [user] specified
     */
    suspend fun destroyIndexYaml(user: User)

    /**
     * Deletes the `index.yaml` file of the [organization] specified
     */
    suspend fun destroyIndexYaml(organization: Organization)

    /**
     * Returns a release tarball from the specified [owner] and [repository][repo] IDs and the
     * specified version.
     *
     * @param owner   owner ID
     * @param repo    repository ID
     * @param version release version to fetch from
     */
    suspend fun getReleaseTarball(owner: Long, repo: Long, version: String): ByteArrayInputStream

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
    suspend fun uploadReleaseTarball(
        owner: Long,
        repo: Long,
        version: String,
        multipart: PartData
    )
}
