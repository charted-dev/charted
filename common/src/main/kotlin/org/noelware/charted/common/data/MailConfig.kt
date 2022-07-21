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

package org.noelware.charted.common.data

/**
 * Represents the configuration for setting up email notifications to send
 * out per event.
 */
@kotlinx.serialization.Serializable
data class MailConfig(
    val username: String? = null,
    val password: String? = null,
    val from: String,
    val host: String = "",
    val port: Int = 465,
    val tls: Boolean = true,
    val ssl: Boolean = false
)
