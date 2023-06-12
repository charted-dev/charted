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

package org.noelware.charted.models.users

import com.fasterxml.jackson.annotation.JsonProperty
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.models.Name
import org.noelware.charted.snowflake.Snowflake

@Schema(description = "Represents an account that can own repositories and organizations")
@Serializable
public data class User(
    @JsonProperty("verified_publisher")
    @SerialName("verified_publisher")
    @get:Schema(description = "Whether if this User is a Verified Publisher or not.")
    val verifiedPublisher: Boolean = false,

    @JsonProperty("gravatar_email")
    @SerialName("gravatar_email")
    @get:Schema(description = "Valid email address that points to an Gravatar avatar, or `null` if it shouldn't use one as the primary avatar")
    val gravatarEmail: String? = null,

    @get:Schema(description = "Short description about this user, can be `null` if none was provided.")
    val description: String? = null,

    @JsonProperty("avatar_hash")
    @SerialName("avatar_hash")
    @get:Schema(description = "Unique hash to locate a user's avatar, this also includes the extension that this avatar is, i.e, `png`.")
    val avatarHash: String? = null,

    @JsonProperty("created_at")
    @SerialName("created_at")
    @get:Schema(description = "Date of when this user was registered to this instance")
    val createdAt: LocalDateTime,

    @JsonProperty("updated_at")
    @SerialName("updated_at")
    @get:Schema(description = "Date of when the server has last updated this user")
    val updatedAt: LocalDateTime,

    @get:Schema(implementation = Name::class, description = "Unique username that can be used to locate this user with the API")
    val username: String,

    @get:Schema(description = "Whether if this User is an Administrator of this instance")
    val admin: Boolean = false,

    @get:Schema(description = "Display name for this user, it should be displayed as '{name} (@{username})' or just '@{username}' if there is no display name")
    val name: String? = null,

    @get:Schema(implementation = Snowflake::class, description = "Unique identifier to locate this user with the API")
    val id: Long
)
