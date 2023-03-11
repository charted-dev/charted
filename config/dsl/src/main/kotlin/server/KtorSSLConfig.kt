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

package org.noelware.charted.configuration.kotlin.dsl.server

import kotlinx.serialization.Serializable
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.serializers.SecretStringSerializer

/**
 * Represents configuration for enabling SSL transport for Ktor.
 */
@Serializable
public data class KtorSSLConfig(
    /** Keystore path to get the SSL certificate from */
    val keystore: String = "./ssl.keystore.jks",

    /** Keystore password to unlock the keystore, if necessary */
    @Serializable(with = SecretStringSerializer::class)
    val password: String? = null,

    /** SSL transport port to bind to for SSL connections */
    val port: Short = 3652
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: Buildable<KtorSSLConfig> {
        /** Keystore path to get the SSL certificate from */
        public var keystore: String = "./ssl.keystore.jks"

        /** Keystore password to unlock the keystore, if necessary */
        public var password: String? = null

        /** SSL transport port to bind to for SSL connections */
        public var port: Short = 3652

        override fun build(): KtorSSLConfig = KtorSSLConfig(keystore, password, port)
    }
}
