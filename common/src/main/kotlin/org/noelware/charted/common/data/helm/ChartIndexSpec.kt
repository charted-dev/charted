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

package org.noelware.charted.common.data.helm

import kotlinx.datetime.LocalDateTime
import org.noelware.charted.common.serializers.HelmLocalDateTimeSerializer

@kotlinx.serialization.Serializable
data class ChartIndexSpec(
    val apiVersion: ChartSpecVersion,
    val name: String,
    val version: String,
    val kubeVersion: String? = null,
    val description: String? = null,
    val type: RepoType? = null,
    val keywords: List<String> = listOf(),
    val home: String? = null,
    val sources: List<String> = listOf(),
    val dependencies: List<ChartDependency> = listOf(),
    val maintainers: List<ChartMaintainer> = listOf(),
    val icon: String? = null,
    val appVersion: String? = null,
    val deprecated: Boolean = false,
    val annotations: Map<String, String> = mapOf(),
    val urls: List<String>,

    @kotlinx.serialization.Serializable(with = HelmLocalDateTimeSerializer::class)
    val created: LocalDateTime? = null,
    val removed: Boolean = false,
    val digest: String? = null
) {
    companion object {
        fun fromSpec(
            urls: List<String>,
            created: LocalDateTime? = null,
            removed: Boolean = false,
            digest: String? = null,
            spec: ChartSpec
        ): ChartIndexSpec = ChartIndexSpec(
            spec.apiVersion,
            spec.name,
            spec.version,
            spec.kubeVersion,
            spec.description,
            spec.type,
            spec.keywords,
            spec.home,
            spec.sources,
            spec.dependencies,
            spec.maintainers,
            spec.icon,
            spec.appVersion,
            spec.deprecated,
            spec.annotations,
            urls,
            created,
            removed,
            digest
        )
    }
}
