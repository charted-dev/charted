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
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.models.flags.ApiKeyScopes
import org.noelware.charted.models.users.User
import org.noelware.charted.snowflake.Snowflake

public val ApiKeys.scopes: ApiKeyScopes get() = ApiKeyScopes(scopesBits)

@Schema(description = "Resource for a personal API key that is created by a user. This is useful for command line tools or scripts, but its main use-case is for the Helm plugin to help you push Helm charts easily into charted-server.")
@Serializable
public data class ApiKeys(
    @get:Schema(description = "Short descriptive text about this API key")
    val description: String? = null,

    @get:Schema(description = "Datetime of when this API key has expired, `null` is represented as this API key will never expire")
    @JsonProperty("expires_in")
    @SerialName("expires_in")
    val expiresIn: LocalDateTime? = null,

    @get:Schema(description = "Bitfield of the available scopes for this API key, useful to restrict access to charted-server APIs that this key doesn't need")
    @JsonProperty("scopes")
    @SerialName("scopes")
    val scopesBits: Long = 0,

    @get:Schema(description = "The token itself, this isn't revealed if you fetched it from the API, this is only populated when you create the key.")
    val token: String? = null,
    val owner: User,

    @get:Schema(description = "The name of this API key", implementation = Name::class)
    val name: String,

    @get:Schema(description = "Unique identifier to this API key", implementation = Snowflake::class)
    val id: Long
)
