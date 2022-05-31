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

package org.noelware.charted.common.config

import kotlinx.serialization.SerialName

/**
 * Represents the configuration for setting up email notifications to send
 * out per event.
 */
@kotlinx.serialization.Serializable
data class MailConfig(
    val password: String? = null,
    val enabled: Boolean = false,
    val preset: MailServerPreset = MailServerPreset.NONE,
    val host: String = "",
    val port: Int = 465,
    val ssl: Boolean = false
)

/**
 * Represents the preset configuration for any common e-mail providers.
 */
@kotlinx.serialization.Serializable
enum class MailServerPreset {
    /**
     * Sets the mail server to use GMail.
     */
    @SerialName("gmail")
    GMAIL,

    /**
     * That we are providing values that aren't from any preset. This is the
     * default option.
     */
    @SerialName("none")
    NONE;
}
