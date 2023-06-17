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

package org.noelware.charted.models.organizations

import com.fasterxml.jackson.annotation.JsonProperty
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.models.Name
import org.noelware.charted.models.users.User
import org.noelware.charted.snowflake.Snowflake

/**
 * Represents a unified organization that can own and manage repositories. An "organization"
 * in the API server is used as a business that houses multiple Helm projects.
 */
@Schema(description = "Represents a unified organization that can own and manage repositories. An \"organization\" in the API server is used as a business that houses multiple Helm projects.")
@Serializable
public data class Organization(
    /**
     * Whether if this organization is a verified publisher on this instance
     */
    @JsonProperty("verified_publisher")
    @SerialName("verified_publisher")
    @get:Schema(description = "Whether if this organization is a verified publisher on this instance")
    val verifiedPublisher: Boolean = false,

    /**
     * Twitter handle for this organization that will probably lead to a valid
     * Twitter account.
     */
    @get:Schema(description = "Twitter handle for this organization that will probably lead to a valid Twitter account.")
    @JsonProperty("twitter_handle")
    @SerialName("twitter_handle")
    val twitterHandle: String? = null,

    /**
     * Valid email address that points to a Gravatar avatar, or `null` if it shouldn't use one as the primary avatar
     */
    @JsonProperty("gravatar_email")
    @SerialName("gravatar_email")
    @get:Schema(description = "Valid email address that points to a Gravatar avatar, or `null` if it shouldn't use one as the primary avatar")
    val gravatarEmail: String? = null,

    /**
     * Display name for this organization. It should be formatted as '{display_name} (@{name})'
     * or just '{name}' in applications that use the API and the default formatting that Hoshi
     * uses.
     */
    @get:Schema(
        description = "Display name for this organization. It should be formatted as '{display_name} (@{name})'" +
            "or just '{name}' in applications that use the API and the default formatting that Hoshi uses",
    )
    @JsonProperty("display_name")
    @SerialName("display_name")
    val displayName: String? = null,

    /**
     * Date of when this organization was registered to this instance
     */
    @JsonProperty("created_at")
    @SerialName("created_at")
    @get:Schema(description = "Date of when this organization was registered to this instance")
    val createdAt: LocalDateTime,

    /**
     * Date of when the server has last updated this organization
     */
    @JsonProperty("updated_at")
    @SerialName("updated_at")
    @get:Schema(description = "Date of when the server has last updated this organization")
    val updatedAt: LocalDateTime,

    /**
     * Unique hash to locate an organization's icon, this also includes the extension that this icon is, i.e, `png`.
     */
    @get:Schema(description = "Unique hash to locate an organization's icon, this also includes the extension that this icon is, i.e, `png`.")
    @JsonProperty("icon_hash")
    @SerialName("icon_hash")
    val iconHash: String? = null,

    /**
     * Whether this organization is private and only its member can access this resource.
     */
    @get:Schema(description = "Whether this organization is private and only its member can access this resource.")
    val private: Boolean = false,
    val owner: User,

    /**
     * The name for this organization.
     */
    @get:Schema(description = "The name for this organization.", implementation = Name::class)
    val name: String,

    /**
     * Unique identifier to locate this organization with the API
     */
    @get:Schema(implementation = Snowflake::class, description = "Unique identifier to locate this organization with the API")
    val id: Long
)
