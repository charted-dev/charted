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

package org.noelware.charted.core.config

import kotlinx.serialization.SerialName

@kotlinx.serialization.Serializable
enum class EngineClass {
    @SerialName("charts")
    CHART,

    @SerialName("oci")
    OCI;
}

/**
 * Represents the configuration to use an OCI registry to store Helm charts. Which is
 * natively supported with `helm push` / `helm pull`.
 */
@kotlinx.serialization.Serializable
data class OciEngineConfig(
    /**
     * The username to use for basic authentication. If null, it will try to check
     * in `~/.docker/config.json` to decipher it, if the path doesn't exist, it'll
     * just be insecure if [insecure] is set to `true`.
     */
    val username: String? = null,

    /**
     * The password to use for basic authentication. If null, it will try to check
     * in `~/.docker/config.json` to decipher it, if the path doesn't exist, it'll
     * just be insecure if [insecure] is set to `true`.
     */
    val password: String? = null,

    /**
     * The registry URI to use to push and pull.
     */
    val registryUri: String,

    /**
     * If the connection should be insecure.
     */
    val insecure: Boolean = false
)

@kotlinx.serialization.Serializable
data class ChartEngineConfig(
    @SerialName("class")
    val engineClass: EngineClass,

    @SerialName("oci")
    val ociConfig: OciEngineConfig? = null
)
