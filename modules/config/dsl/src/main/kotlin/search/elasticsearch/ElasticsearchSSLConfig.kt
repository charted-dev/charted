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

package org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.serializers.SecretStringSerializer
import kotlin.properties.Delegates

/**
 * Represents the configuration for configuring Elasticsearch with SSL security.
 * @param validateHostnames If the Elasticsearch REST client should validate the hostname of the certificates
 * @param keystorePassword  The keystore password from the [clientPath], if specified
 * @param clientPath        Keystore path
 * @param certKind          certificate kind
 */
@Serializable
data class ElasticsearchSSLConfig(
    @SerialName("validate_hostnames")
    val validateHostnames: Boolean = false,

    @SerialName("keystore_password")
    @Serializable(with = SecretStringSerializer::class)
    val keystorePassword: String? = null,

    @SerialName("ca_path")
    val caPath: String
) {
    class Builder: org.noelware.charted.common.Builder<ElasticsearchSSLConfig> {
        var validateHostnames: Boolean = false
        var keystorePassword: String? = null
        var caPath: String by Delegates.notNull()

        override fun build(): ElasticsearchSSLConfig = ElasticsearchSSLConfig(validateHostnames, keystorePassword, caPath)
    }
}
