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

package org.noelware.charted.modules.postgresql.tables

import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import net.perfectdreams.exposedpowerutils.sql.postgresEnumeration
import org.jetbrains.exposed.sql.kotlin.datetime.datetime
import org.noelware.charted.common.types.helm.RepoType
import org.noelware.charted.modules.postgresql.SnowflakeTable

object RepositoryTable: SnowflakeTable("repositories") {
    val description = varchar("description", 64).nullable().default(null)
    val deprecated = bool("deprecated").default(false)
    val createdAt = datetime("created_at").default(Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()))
    val updatedAt = datetime("updated_at").default(Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()))
    val iconHash = text("icon_hash").nullable().default(null)
    val private = bool("private").default(false)
    val owner = long("owner_id").uniqueIndex()
    val name = varchar("name", 32).uniqueIndex()
    val type = postgresEnumeration<RepoType>("type").default(RepoType.APPLICATION)
}