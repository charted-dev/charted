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
import org.noelware.charted.snowflake.Snowflake

/**
 * Represents a user's connections that are used to log-in from different session
 * integrations.
 *
 * @param noelwareAccountID The account ID that is sourced from [Noelware's User Accounts System](https://accounts.noelware.org).
 * @param googleAccountID   Account ID that is sourced from Google
 * @param githubAccountID   Account ID that is sourced from GitHub
 * @param appleAccountID    Account ID that is sourced from Apple
 * @param createdAt         Creation date of when this user was created. This will have a slight offset
 *                          from a user's creation time since the user is first created.
 * @param updatedAt         Date of when this entity was last updated.
 * @param id                The user's ID that this entity this belongs to.
 */
@Schema(description = "Represents a user's connections that are used to log-in from different session integrations.")
@Serializable
public data class UserConnections(
    @get:Schema(description = "The account ID that is sourced from [Noelware's User Accounts System](https://accounts.noelware.org).")
    @JsonProperty("noelware_account_id")
    @SerialName("noelware_account_id")
    val noelwareAccountID: Long? = null,

    @get:Schema(description = "Account ID that is sourced from Google")
    @JsonProperty("google_account_id")
    @SerialName("google_account_id")
    val googleAccountID: String? = null,

    @get:Schema(description = "Account ID that is sourced from GitHub")
    @JsonProperty("github_account_id")
    @SerialName("github_account_id")
    val githubAccountID: String? = null,

    @get:Schema(description = "Account ID that is sourced from Apple")
    @JsonProperty("apple_account_id")
    @SerialName("apple_account_id")
    val appleAccountID: String? = null,

    @get:Schema(description = "Creation date of when this user was created. This will have a slight offset from a user's creation time since the user is first created.")
    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @JsonProperty("updated_at")
    @SerialName("updated_at")
    @get:Schema(description = "Date of when the server has last updated this user's connections")
    val updatedAt: LocalDateTime,

    @get:Schema(implementation = Snowflake::class, description = "Unique identifier to locate this user with the API")
    val id: Long
)
