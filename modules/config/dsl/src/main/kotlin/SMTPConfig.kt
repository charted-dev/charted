/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.Serializable

/**
 * Represents the configuration for setting up email notifications to send
 * out per event.
 */
@Serializable
public data class SMTPConfig(
    val username: String? = null,
    val password: String? = null,
    val from: String,
    val host: String = "",
    val port: Int = 465,
    val tls: Boolean = true,
    val ssl: Boolean = false
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder(private val from: String) : org.noelware.charted.common.Builder<SMTPConfig> {
        public var username: String? = null
        public var password: String? = null
        public var host: String = ""
        public var port: Int = 465
        public var tls: Boolean = true
        public var ssl: Boolean = false

        override fun build(): SMTPConfig = SMTPConfig(username, password, from, host, port, tls, ssl)
    }
}
