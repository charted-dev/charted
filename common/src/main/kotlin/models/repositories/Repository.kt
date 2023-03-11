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
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.types.helm.RepoType

@Serializable
public data class Repository(
    val description: String? = null,
    val deprecated: Boolean = false,

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

    @JsonProperty("owner_id")
    @SerialName("owner_id")
    val ownerID: Long,
    val name: String,
    val type: RepoType,
    val id: Long
)
