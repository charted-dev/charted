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

package org.noelware.charted.common.types.helm

import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable
import org.noelware.charted.models.VersionConstraint

@Serializable
public data class ChartIndexSpec(
    /**
     * The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous
     * Helm versions have an apiVersion set to v1 and are still installable by Helm 3.
     */
    @get:Schema(
        description = "The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous" +
            " Helm versions have an apiVersion set to v1 and are still installable by Helm 3.",
    )
    val apiVersion: ChartSpecVersion,

    /**
     * The name of the chart
     */
    @get:Schema(description = "The name of the chart")
    val name: String,

    /**
     * A SemVer 2 conformant version string of the chart.
     */
    @get:Schema(description = "A SemVer 2 conformant version string of the chart.", implementation = VersionConstraint::class)
    val version: String,

    /**
     * The optional kubeVersion field can define SemVer constraints on supported Kubernetes versions. Helm will validate the version constraints
     * when installing the chart and fail if the cluster runs an unsupported Kubernetes version.
     */
    @get:Schema(
        description = "The optional kubeVersion field can define SemVer constraints on supported Kubernetes versions. Helm will validate the version constraints" +
            " when installing the chart and fail if the cluster runs an unsupported Kubernetes version.",
        implementation = VersionConstraint::class,
    )
    val kubeVersion: String? = null,

    /**
     * A single-sentence description of this project
     */
    @get:Schema(description = "A single-sentence description of this project")
    val description: String? = null,

    /**
     * The type of the chart
     */
    @get:Schema(description = "The type of the chart")
    val type: RepoType? = null,

    /**
     * A list of keywords about this project. These keywords can be searched
     * via the /search endpoint if it's enabled.
     */
    @get:Schema(description = "List of keywords about this project. These keywords can be searched via the Repository Search API if a search backend is enabled.")
    val keywords: List<String> = listOf(),

    /**
     * The URL of this project's homepage.
     */
    @get:Schema(description = "The URL of this project's homepage.")
    val home: String? = null,

    /**
     * A list of URLs to the source code for this project
     */
    @get:Schema(description = "A list of URLs to the source code for this project")
    val sources: List<String> = listOf(),

    /**
     * In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies' field in Chart.yaml or brought in to the charts/ directory and managed manually.
     * The charts required by the current chart are defined as a list in the dependencies field.
     */
    @get:Schema(
        description = "In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies' field in Chart.yaml or brought in to the charts/ directory and managed manually." +
            " The charts required by the current chart are defined as a list in the dependencies field.",
    )
    val dependencies: List<ChartDependency> = listOf(),

    /**
     * A list of name and URL/email address combinations for the maintainer(s)
     */
    @get:Schema(description = "A list of name and URL/email address combinations for the maintainer(s)")
    val maintainers: List<ChartMaintainer> = listOf(),

    /**
     * A URL or an SVG or PNG image to be used as an icon
     */
    @get:Schema(description = "A URL or an SVG or PNG image to be used as an icon")
    val icon: String? = null,

    /**
     * Note that the appVersion field is not related to the version field. It is a way of specifying the version of the
     * application. For example, the drupal chart may have an appVersion: "8.2.1", indicating that the version of Drupal
     * included in the chart (by default) is 8.2.1. This field is informational, and has no impact on chart version calculations.
     *
     * Wrapping the version in quotes is highly recommended. It forces the YAML parser to treat the version number as a string.
     * Leaving it unquoted can lead to parsing issues in some cases. For example, YAML interprets 1.0 as a floating point value,
     * and a git commit SHA like 1234e10 as scientific notation.
     */
    @get:Schema(
        description = "Note that the appVersion field is not related to the version field. It is a way of specifying the version of the" +
            "application. For example, the drupal chart may have an appVersion: \"8.2.1\", indicating that the version of Drupal" +
            "included in the chart (by default) is 8.2.1. This field is informational, and has no impact on chart version calculations." +
            "\n" +
            "Wrapping the version in quotes is highly recommended. It forces the YAML parser to treat the version number as a string." +
            "Leaving it unquoted can lead to parsing issues in some cases. For example, YAML interprets 1.0 as a floating point value," +
            "and a git commit SHA like 1234e10 as scientific notation.",
    )
    val appVersion: String? = null,

    /**
     * When managing charts in a Chart Repository, it is sometimes necessary to deprecate a chart. The optional deprecated field
     * in Chart.yaml can be used to mark a chart as deprecated. If the latest version of a chart in the repository is marked
     * as deprecated, then the chart as a whole is considered to be deprecated.
     *
     * The chart name can be later reused by publishing a newer version that is not marked as deprecated.
     */
    @get:Schema(
        description = "When managing charts in a Chart Repository, it is sometimes necessary to deprecate a chart. The optional deprecated field" +
            "in Chart.yaml can be used to mark a chart as deprecated. If the latest version of a chart in the repository is marked" +
            "as deprecated, then the chart as a whole is considered to be deprecated." +
            "\n" +
            "The chart name can be later reused by publishing a newer version that is not marked as deprecated.",
    )
    val deprecated: Boolean = false,

    /**
     * A list of annotations keyed by name and value.
     */
    @get:Schema(description = "A list of annotations keyed by name and value.")
    val annotations: Map<String, String> = mapOf(),

    // not documented in Helm source code
    // https://github.com/helm/helm/blob/main/pkg/repo/index.go#L255
    val urls: List<String>,
    val created: Instant? = null,
    val removed: Boolean = false,
    val digest: String? = null
) {
    public companion object {
        /**
         * Creates a new [ChartIndexSpec] spec object with the appropriate
         * [ChartSpec].
         */
        @JvmStatic
        public fun fromSpec(
            urls: List<String>,
            created: Instant? = null,
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
            digest,
        )
    }
}
