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
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
public data class User(
    @JsonProperty("verified_publisher")
    @SerialName("verified_publisher")
    val verifiedPublisher: Boolean = false,

    @JsonProperty("gravatar_email")
    @SerialName("gravatar_email")
    val gravatarEmail: String? = null,
    val description: String? = null,

    @JsonProperty("avatar_hash")
    @SerialName("avatar_hash")
    val avatarHash: String? = null,

    @JsonProperty("created_at")
    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @JsonProperty("updated_at")
    @SerialName("updated_at")
    val updatedAt: LocalDateTime,
    val username: String,
    val admin: Boolean = false,
    val name: String? = null,
    val id: Long
)
