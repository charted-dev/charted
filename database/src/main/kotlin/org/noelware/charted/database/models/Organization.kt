/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

package org.noelware.charted.database.models

import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import org.noelware.charted.database.entities.OrganizationEntity
import org.noelware.charted.database.flags.OrganizationFlags

@kotlinx.serialization.Serializable
data class Organization(
    @SerialName("verified_publisher")
    val verifiedPublisher: Boolean = false,

    @SerialName("twitter_handle")
    val twitterHandle: String? = null,

    @SerialName("gravatar_email")
    val gravatarEmail: String? = null,

    @SerialName("display_name")
    val displayName: String? = null,

    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,

    @SerialName("icon_hash")
    val iconHash: String? = null,
    val flags: Long,
    val name: String,
    val id: Long
) {
    companion object {
        fun fromEntity(entity: OrganizationEntity): Organization = Organization(
            entity.verifiedPublisher,
            entity.twitterHandle,
            entity.gravatarEmail,
            entity.displayName,
            entity.createdAt,
            entity.updatedAt,
            entity.iconHash,
            entity.flags,
            entity.name,
            entity.id.value
        )
    }
}

val Organization.bitfield: OrganizationFlags
    get() = OrganizationFlags(flags)
