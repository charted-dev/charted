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

package org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.serializers.SecretStringSerializer
import kotlin.properties.Delegates

@Serializable
public data class ElasticsearchSSLConfig(
    /** If the Elasticsearch REST client should validate the hostname of the certificates */
    @SerialName("validate_hostnames")
    val validateHostnames: Boolean = false,

    /** Keystore password to unlock the keystore to collect the SSL certificate for */
    @Serializable(with = SecretStringSerializer::class)
    @SerialName("keystore_password")
    val keystorePassword: String? = null,

    /** Path to the client CA */
    @SerialName("ca_path")
    val caPath: String
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: Buildable<ElasticsearchSSLConfig> {
        /** If the Elasticsearch REST client should validate the hostname of the certificates */
        public var validateHostnames: Boolean = false

        /** Keystore password to unlock the keystore to collect the SSL certificate for */
        public var keystorePassword: String? = null

        /** Path to the client CA */
        public var caPath: String by Delegates.notNull()

        override fun build(): ElasticsearchSSLConfig = ElasticsearchSSLConfig(validateHostnames, keystorePassword, caPath)
    }
}
