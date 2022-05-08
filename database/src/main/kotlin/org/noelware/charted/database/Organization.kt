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
import org.jetbrains.exposed.sql.Table
import org.jetbrains.exposed.sql.kotlin.datetime.datetime
import java.time.LocalDateTime

object Organization: IdTable<Long>("organizations") {
    val verifiedPublisher = bool("verified_publisher").default(false)
    val twitterHandle = text("twitter_handle").nullable().default(null)
    val description = varchar("description", 240).nullable().default(null)
    val displayName = varchar("display_name", 140).nullable().default(null)
    val createdAt = datetime("created_at").default(LocalDateTime.now().toKotlinLocalDateTime())
    val updatedAt = datetime("updated_at").default(LocalDateTime.now().toKotlinLocalDateTime())
//    val members = reference("members", OrganizationMember)
    val handle = varchar("handle", 60)
    val avatar = text("avatar").nullable().default(null)

    override val id: Column<EntityID<Long>> = long("id").entityId()
    override val primaryKey: Table.PrimaryKey = PrimaryKey(id)
}
