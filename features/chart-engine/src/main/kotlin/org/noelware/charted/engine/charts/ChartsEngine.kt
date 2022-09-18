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

package org.noelware.charted.engine.charts

import io.ktor.http.content.*
import org.noelware.charted.common.data.helm.ChartSpec
import org.noelware.charted.database.models.Repository

/**
 * Represents the engine for handling Helm Charts.
 */
interface ChartsEngine {
    /**
     * Returns the chart's metadata from the repository.
     * @param repository The repository's ID.
     */
    suspend fun getChartMetadata(repository: Repository): ChartSpec?

    /**
     * Returns the chart's `values.yaml` file.
     * @param repository The repository's ID.
     */
    // TODO: should it be a String since I don't think kaml (YAML library
    //       for kotlinx.serialization) can do dynamic data.
    suspend fun getChartValues(repository: Repository): String?

    suspend fun uploadChartMetadata(
        repository: Repository,
        part: PartData.FileItem
    )

    suspend fun uploadChartValues(
        repository: Repository,
        part: PartData.FileItem
    )

    suspend fun getChartTarball(repository: Repository, version: String)
}
