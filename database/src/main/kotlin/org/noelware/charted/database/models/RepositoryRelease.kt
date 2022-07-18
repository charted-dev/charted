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
import org.noelware.charted.database.entities.RepositoryReleaseEntity

@kotlinx.serialization.Serializable
data class RepositoryRelease(
    @SerialName("update_text")
    val updateText: String? = null,

    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,
    val tag: String,
    val id: Long
) {
    companion object {
        fun fromEntity(entity: RepositoryReleaseEntity): RepositoryRelease = RepositoryRelease(
            entity.updateText,
            entity.createdAt,
            entity.updatedAt,
            entity.tag,
            entity.id.value
        )
    }
}

/*
object RepositoryReleasesTable: SnowflakeTable("repository_releases") {
    val repository = reference("repository_id", RepositoryTable)
    val updateText = text("update_text").nullable().default(null)
    val createdAt = datetime("created_at").default(Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()))
    val updatedAt = datetime("updated_at").default(Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()))
    val tag = text("tag")
}

 */
