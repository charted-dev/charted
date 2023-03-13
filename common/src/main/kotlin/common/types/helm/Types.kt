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
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import org.noelware.charted.ValidationException
import org.noelware.charted.common.extensions.string.toUriOrNull

/**
 * The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous
 * Helm versions have an apiVersion set to v1 and are still installable by Helm 3.
 *
 * @param version The version key to make up this enum.
 */
@Serializable(with = ChartSpecVersion.Companion::class)
public enum class ChartSpecVersion(private val version: String) {
    @JsonProperty("v1")
    V2("v1"),

    @JsonProperty("v2")
    V3("v2");

    internal companion object : KSerializer<ChartSpecVersion> {
        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.ChartSpecVersion", PrimitiveKind.STRING)
        override fun deserialize(decoder: Decoder): ChartSpecVersion = values().single { it.version == decoder.decodeString() }
        override fun serialize(encoder: Encoder, value: ChartSpecVersion) {
            encoder.encodeString(value.version)
        }
    }
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
@Serializable
public data class ChartDependency(
    /**
     * The name of the chart
     */
    val name: String,

    /**
     * The version of the chart.
     */
    val version: String? = null,

    /**
     * The repository URL or alias
     */
    val repository: String? = null,

    /**
     * A YAML path that resolves to a Boolean, used for enabling/disabling charts.
     */
    val condition: String? = null,

    /**
     * Tags can be used to group charts for enabling/disabling together
     */
    val tags: List<String> = listOf(),

    /**
     * ImportValues holds the mapping of source values to parent key to be imported.
     * Each item can be a string or pair of child/parent sublist items.
     */
    @SerialName("import-values")
    val importValues: List<StringOrImportValue> = listOf(),

    /**
     * Alias to be used for the chart. Useful when you have to add the same chart multiple times
     */
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
@Serializable
public data class ChartSpec(
    /**
     * The apiVersion field should be v2 for Helm charts that require at least Helm 3. Charts supporting previous
     * Helm versions have an apiVersion set to v1 and are still installable by Helm 3.
     */
    val apiVersion: ChartSpecVersion,

    /**
     * The name of the chart
     */
    val name: String,

    /**
     * A SemVer 2 version
     */
    val version: String,

    /**
     * The optional kubeVersion field can define SemVer constraints on supported Kubernetes versions. Helm will validate the version constraints
     * when installing the chart and fail if the cluster runs an unsupported Kubernetes version.
     */
    val kubeVersion: String? = null,

    /**
     * A single-sentence description of this project
     */
    val description: String? = null,

    /**
     * The type of the chart
     */
    val type: RepoType? = null,

    /**
     * A list of keywords about this project. These keywords can be searched
     * via the /search endpoint if it's enabled.
     */
    val keywords: List<String> = listOf(),

    /**
     * The URL of this project's homepage.
     */
    val home: String? = null,

    /**
     * A list of URLs to the source code for this project
     */
    val sources: List<String> = listOf(),

    /**
     * In Helm, one chart may depend on any number of other charts. These dependencies can be dynamically linked using the dependencies' field in Chart.yaml or brought in to the charts/ directory and managed manually.
     * The charts required by the current chart are defined as a list in the dependencies field.
     */
    val dependencies: List<ChartDependency> = listOf(),
    val maintainers: List<ChartMaintainer> = listOf(),

    /** A URL or an SVG or PNG image to be used as an icon */
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
    val appVersion: String? = null,

    /**
     * When managing charts in a Chart Repository, it is sometimes necessary to deprecate a chart. The optional deprecated field
     * in Chart.yaml can be used to mark a chart as deprecated. If the latest version of a chart in the repository is marked
     * as deprecated, then the chart as a whole is considered to be deprecated.
     *
     * The chart name can be later reused by publishing a newer version that is not marked as deprecated.
     */
    val deprecated: Boolean = false,

    /**
     * A list of annotations keyed by name and value.
     */
    val annotations: Map<String, String> = mapOf()
)
