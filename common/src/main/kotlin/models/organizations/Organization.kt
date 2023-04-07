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
import org.noelware.charted.models.users.User

@Serializable
public data class Organization(
    @JsonProperty("verified_publisher")
    @SerialName("verified_publisher")
    @get:Schema(description = "Whether if this organization is a verified publisher on this instance")
    val verifiedPublisher: Boolean = false,

    @JsonProperty("twitter_handle")
    @SerialName("twitter_handle")
    val twitterHandle: String? = null,

    @JsonProperty("gravatar_email")
    @SerialName("gravatar_email")
    val gravatarEmail: String? = null,

    @JsonProperty("display_name")
    @SerialName("display_name")
    val displayName: String? = null,

    @JsonProperty("created_at")
    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @JsonProperty("updated_at")
    @SerialName("updated_at")
    val updatedAt: LocalDateTime,

    @JsonProperty("icon_hash")
    @SerialName("icon_hash")
    val iconHash: String? = null,
    val private: Boolean = false,
    val owner: User,
    val name: String,
    val id: Long
)
