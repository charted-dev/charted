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

package org.noelware.charted.database.tables

import kotlinx.datetime.toKotlinLocalDateTime
import org.jetbrains.exposed.sql.kotlin.datetime.datetime
import org.noelware.charted.database.SnowflakeTable
import java.time.LocalDateTime

object RepositoryMemberTable: SnowflakeTable("repository_members") {
    val publicVisibility = bool("public_visibility").default(false)
    val displayName = varchar("display_name", 32).nullable().default(null)
    val repository = reference("repository_id", RepositoryTable)
    val updatedAt = datetime("updated_at").default(LocalDateTime.now().toKotlinLocalDateTime())
    val joinedAt = datetime("joined_at").default(LocalDateTime.now().toKotlinLocalDateTime())
    val account = reference("account_id", UserTable)
}
