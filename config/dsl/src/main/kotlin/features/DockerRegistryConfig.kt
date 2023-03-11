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
import org.noelware.charted.common.Buildable

/**
 * Represents the configuration for an external Docker Registry server
 * that has hosted Helm charts.
 */
@Serializable
public data class DockerRegistryConfig(
    /**
     * If we should inherit the authentication mechanism that this server enables. This will also require all users
     * in this space to have an account on charted-server to keep things smoothly.
     */
    @SerialName("inherit_auth")
    val inheritAuth: Boolean = true,

    /**
     * Host domain or IP to connect to when making connections.
     */
    val host: String,

    /**
     * Port to bind to, if necessary
     */
    val port: Short
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: Buildable<DockerRegistryConfig> {
        /**
         * If we should inherit the authentication mechanism that this server enables. This will also require all users
         * in this space to have an account on charted-server to keep things smoothly.
         */
        public var inheritAuth: Boolean = true

        /**
         * Host domain or IP to connect to when making connections.
         */
        public var host: String = "localhost"

        /**
         * Port to bind to, if necessary
         */
        public var port: Short = 5000
        override fun build(): DockerRegistryConfig = DockerRegistryConfig(inheritAuth, host, port)
    }
}
