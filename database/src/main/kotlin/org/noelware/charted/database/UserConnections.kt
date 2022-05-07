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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.database

import kotlinx.datetime.toKotlinLocalDateTime
import org.jetbrains.exposed.dao.id.EntityID
import org.jetbrains.exposed.dao.id.IdTable
import org.jetbrains.exposed.sql.Column
import org.jetbrains.exposed.sql.kotlin.datetime.datetime
import org.noelware.charted.database.Repository.default
import java.time.LocalDateTime

object UserConnections: IdTable<Long>("user_connections") {
    val createdAt = datetime("created_at").default(LocalDateTime.now().toKotlinLocalDateTime())
    val updatedAt = datetime("updated_at").default(LocalDateTime.now().toKotlinLocalDateTime())
    val noelwareAccountId = long("noelware_account_id").nullable().default(null)
    val appleAccountId = text("apple_account_id").nullable().default(null)
    val googleAccountId = text("google_account_id").nullable().default(null)

    override val id: Column<EntityID<Long>> = long("id").entityId()
    override val primaryKey: PrimaryKey = PrimaryKey(id, name = "UserConnectionsPK")
}
