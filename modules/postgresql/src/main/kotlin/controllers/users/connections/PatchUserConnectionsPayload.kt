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

package org.noelware.charted.modules.postgresql.controllers.users.connections

import com.fasterxml.jackson.annotation.JsonProperty
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class PatchUserConnectionsPayload(
    @JsonProperty("noelware_account_id")
    @SerialName("noelware_account_id")
    val noelwareAccountID: Long? = null,

    @JsonProperty("google_account_id")
    @SerialName("google_account_id")
    val googleAccountID: String? = null,

    @JsonProperty("github_account_id")
    @SerialName("github_account_id")
    val githubAccountID: String? = null,

    @JsonProperty("apple_account_id")
    @SerialName("apple_account_id")
    val appleAccountID: String? = null
)
