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

package org.noelware.charted.models.repositories

import com.fasterxml.jackson.annotation.JsonProperty
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.types.helm.RepoType
import org.noelware.charted.models.Name
import org.noelware.charted.snowflake.Snowflake

@Serializable
public data class Repository(
    @get:Schema(description = "Short description about this user, can be `null` if none was provided.")
    val description: String? = null,

    @get:Schema(description = "Whether if this repository is deprecated")
    val deprecated: Boolean = false,

    @JsonProperty("created_at")
    @SerialName("created_at")
    @get:Schema(description = "Date of when this repository was registered to this instance")
    val createdAt: LocalDateTime,

    @JsonProperty("updated_at")
    @SerialName("updated_at")
    @get:Schema(description = "Date of when the server has last updated this repository")
    val updatedAt: LocalDateTime,

    @JsonProperty("icon_hash")
    @SerialName("icon_hash")
    @get:Schema(description = "Unique hash to locate a repository's icon, this also includes the extension that this avatar is, i.e, `png`.")
    val iconHash: String? = null,

    @get:Schema(description = "Whether if this repository is private or not")
    val private: Boolean = false,

    @JsonProperty("owner_id")
    @SerialName("owner_id")
    @get:Schema(description = "Unique identifier that points to a User or Organization resource that owns this repository", implementation = Snowflake::class)
    val ownerID: Long,

    @get:Schema(description = "Unique [Name] to locate this repository from the API", implementation = Name::class)
    val name: String,

    @get:Schema(description = "The chart type that this repository is")
    val type: RepoType,

    @get:Schema(description = "Unique identifier to locate this repository from the API", implementation = Snowflake::class)
    val id: Long
)
