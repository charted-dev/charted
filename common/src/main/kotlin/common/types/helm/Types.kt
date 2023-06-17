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

import com.fasterxml.jackson.annotation.JsonProperty
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ValidationException
import org.noelware.charted.common.extensions.string.toUriOrNull
import org.noelware.charted.models.VersionConstraint

/**
 * The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous
 * Helm versions have an apiVersion set to v1 and are still installable by Helm 3.
 */
@Schema(
    description = "The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous" +
        " Helm versions have an apiVersion set to v1 and are still installable by Helm 3.",
)
public enum class ChartSpecVersion {
    /**
     * Chart supports running on Helm 2 or 3.
     */
    @Schema(description = "Chart supports running on Helm 2 or 3.")
    @SerialName("v1")
    @JsonProperty("v1")
    V1,

    /**
     * Chart supports running only on Helm 3.
     */
    @Schema(description = "Chart supports running only on Helm 3.")
    @SerialName("v2")
    @JsonProperty("v2")
    V2
}

@Serializable
public enum class RepoType {
    @JsonProperty("application")
    @SerialName("application")
    APPLICATION,

    @JsonProperty("library")
    @SerialName("library")
    LIBRARY,

    @JsonProperty("operator")
    @SerialName("operator")
    OPERATOR
}

public val RepoType.key: String
    get() = when (this) {
        RepoType.APPLICATION -> "application"
        RepoType.LIBRARY -> "library"
        RepoType.OPERATOR -> "operator"
    }

public fun String.toRepoType(): RepoType = when (this) {
    "application" -> RepoType.APPLICATION
    "library" -> RepoType.LIBRARY
    "operator" -> RepoType.OPERATOR
    else -> error("Unknown repository type [$this]")
}

/**
 * In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies' field in Chart.yaml or brought in to the charts/ directory and managed manually.
 * The charts required by the current chart are defined as a list in the dependencies field.
 */
@Schema(
    description = "In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies' field in Chart.yaml or brought in to the charts/ directory and managed manually." +
        " The charts required by the current chart are defined as a list in the dependencies field.",
)
@Serializable
public data class ChartDependency(
    /**
     * The name of the chart
     */
    @get:Schema(description = "The name of the chart.")
    val name: String,

    /**
     * The version of the chart.
     */
    @get:Schema(description = "The version of the chart.", implementation = VersionConstraint::class)
    val version: String? = null,

    /**
     * The repository URL or alias
     */
    @get:Schema(description = "The repository URL or alias")
    val repository: String? = null,

    /**
     * A YAML path that resolves to a Boolean, used for enabling/disabling charts.
     */
    @get:Schema(description = "YAML path that resolves to a Boolean, used for enabling/disabling charts.")
    val condition: String? = null,

    /**
     * Tags can be used to group charts for enabling/disabling together
     */
    @get:Schema(description = "Tags can be used to group charts for enabling/disabling together")
    val tags: List<String> = listOf(),

    /**
     * [ImportValues][ImportValue] holds the mapping of source values to parent key to be imported.
     * Each item can be a string or pair of child/parent sublist items.
     */
    @JsonProperty("import-values")
    @SerialName("import-values")
    @get:Schema(
        description = "ImportValues holds the mapping of source values to parent key to be imported." +
            " Each item can be a string or pair of child/parent sublist items.",
    )
    val importValues: List<StringOrImportValue> = listOf(),

    /**
     * Alias to be used for the chart. Useful when you have to add the same chart multiple times
     */
    @get:Schema(description = "Alias to be used for the chart. Useful when you have to add the same chart multiple times")
    val alias: String? = null
) {
    init {
        if (repository != null) {
            when {
                repository.matches("^@".toRegex()) -> {} // skip
                repository.toUriOrNull() != null -> {} // skip
                else -> throw ValidationException("body.repository", "Didn't match '^@' regex or was a valid URI.")
            }
        }
    }
}

/**
 * Name and URL/email address combination as a maintainer. The `name` can be a [NameOrSnowflake][org.noelware.charted.models.NameOrSnowflake]
 * to reference a maintainer in the Hoshi UI.
 */
@Schema(description = "Name and URL/email address combination as a maintainer. The `name` can be a [NameOrSnowflake] to reference a maintainer in the Hoshi UI.")
@Serializable
public data class ChartMaintainer(
    /** The maintainer's name */
    val name: String,

    /** The maintainer's email */
    val email: String? = null,

    /** A URL for the maintainer */
    val url: String? = null
) {
    init {
        if (url != null && url.toUriOrNull() == null) {
            throw ValidationException("body.url", "The URL wasn't a valid URI.")
        }
    }
}

/**
 * Represents a `Chart.yaml` that can be serialized and deserialized. This is usually from
 * the `/repositories/<id>/Chart.yaml` endpoint or `/repositories/<id>/<release>/Chart.yaml`
 */
@Schema(description = "Represents a `Chart.yaml` definition object.")
@Serializable
public data class ChartSpec(
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
    val annotations: Map<String, String> = mapOf()
)
