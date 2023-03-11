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

package org.noelware.charted.models

import com.fasterxml.jackson.annotation.JsonProperty
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.models.flags.ApiKeyScopes
import org.noelware.charted.models.users.User

public val ApiKeys.scopes: ApiKeyScopes get() = ApiKeyScopes(scopesBits)

@Serializable
public data class ApiKeys(
    val description: String? = null,

    @JsonProperty("expires_in")
    @SerialName("expires_in")
    val expiresIn: LocalDateTime? = null,

    @JsonProperty("bits")
    @SerialName("bits")
    val scopesBits: Long = 0,
    val token: String? = null,
    val owner: User,
    val name: String,
    val id: Long
)
