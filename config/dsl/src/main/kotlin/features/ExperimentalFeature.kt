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

package org.noelware.charted.configuration.kotlin.dsl.features

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * List of experimental features to enable globally.
 */
@Serializable
public enum class ExperimentalFeature {
    /**
     * Features that allows to use an external OCI registry (i.e, [Docker Registry](https://hub.docker.com/_/registry))
     * instead our own home-made implementation.
     *
     * This is useful if you have other Helm charts exported from one registry and want to migrate
     * to **charted-server** without heavily trying to migrate all your charts into **charted-server**.
     */
    @SerialName("external_oci_registry")
    ExternalOciRegistry,

    /**
     * Wildcard to enable all experimental features.
     */
    @SerialName("*")
    Wildcard
}

/**
 * Determines if a specific experimental feature is enabled or not.
 */
public infix fun List<ExperimentalFeature>.enabled(feature: ExperimentalFeature): Boolean = isWildcard() || any { it == feature }

/**
 * Determine if the given list of [ExperimentalFeature] is a wildcard, so we list
 * all the options in the key-set instead of specific keys.
 */
public fun List<ExperimentalFeature>.isWildcard(): Boolean {
    if (isEmpty()) return false
    if (size == 1) return first() == ExperimentalFeature.Wildcard

    return any { it == ExperimentalFeature.Wildcard }
}
