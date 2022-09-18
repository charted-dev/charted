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

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.decodeFromStream
import dev.floofy.utils.slf4j.logging
import io.ktor.http.content.*
import org.noelware.charted.common.data.helm.ChartSpec
import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.configuration.dsl.features.Feature
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.database.models.Repository

class DefaultChartsEngine(
    private val storage: StorageWrapper,
    private val config: Config,
    private val yaml: Yaml
): ChartsEngine {
    private val log by logging<DefaultChartsEngine>()

    override suspend fun getChartMetadata(repository: Repository): ChartSpec? {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            throw IllegalStateException("CHARTS_NOT_AVAILABLE")
        }

        val chart = storage.trailer.open("./metadata/${repository.ownerID}/${repository.id}/Chart.yaml")
            ?: return null

        return yaml.decodeFromStream(chart)
    }

    override suspend fun getChartValues(repository: Repository): String? {
        if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            throw IllegalStateException("CHARTS_NOT_AVAILABLE")
        }

        val chart = storage.trailer.open("./metadata/${repository.ownerID}/${repository.id}/values.yaml")
            ?: return null

        return yaml.decodeFromStream(chart)
    }

    override suspend fun getChartTarball(repository: Repository, version: String) {
        TODO("Not yet implemented")
    }

    override suspend fun uploadChartMetadata(repository: Repository, part: PartData.FileItem) {
        TODO("Not yet implemented")
    }

    override suspend fun uploadChartValues(repository: Repository, part: PartData.FileItem) {
        TODO("Not yet implemented")
    }
}
