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

package org.noelware.charted.modules.postgresql.controllers.repositories.releases

import io.ktor.server.application.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.and
import org.jetbrains.exposed.sql.update
import org.noelware.charted.ValidationException
import org.noelware.charted.models.repositories.RepositoryRelease
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.controllers.AbstractDatabaseController
import org.noelware.charted.modules.postgresql.entities.RepositoryReleaseEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.ktor.repository
import org.noelware.charted.modules.postgresql.tables.RepositoryReleaseTable
import org.noelware.charted.snowflake.Snowflake

class RepositoryReleaseDatabaseController(
    private val snowflake: Snowflake
): AbstractDatabaseController<RepositoryRelease, RepositoryReleaseEntity, CreateRepositoryReleasePayload, PatchRepositoryReleasePayload>(
    RepositoryReleaseTable,
    RepositoryReleaseEntity,
    { entity -> RepositoryRelease.fromEntity(entity) },
) {
    override suspend fun create(call: ApplicationCall, data: CreateRepositoryReleasePayload): RepositoryRelease {
        val repo = call.repository ?: error("BUG: Missing Repository ID attribute key in this ApplicationCall ($call)")

        // Check if a tag is already exists
        val tagRelease = getEntityOrNull {
            (RepositoryReleaseTable.repository eq repo.id) and (RepositoryReleaseTable.tag eq data.tag)
        }

        if (tagRelease != null) {
            throw ValidationException("body.tag", "Tag [${data.tag}] already exists")
        }

        val id = snowflake.generate()
        return asyncTransaction {
            RepositoryReleaseEntity.new(id.value) {
                repository = repo
                updateText = data.updateText
                createdAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                tag = data.tag
            }.let { entity -> RepositoryRelease.fromEntity(entity) }
        }
    }

    override suspend fun update(call: ApplicationCall, id: Long, patched: PatchRepositoryReleasePayload) {
        if (patched.updateText == null) return
        return asyncTransaction {
            RepositoryReleaseTable.update({ RepositoryReleaseTable.id eq id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())

                if (patched.updateText.isBlank()) {
                    it[updateText] = null
                } else {
                    it[updateText] = patched.updateText
                }
            }
        }
    }
}
