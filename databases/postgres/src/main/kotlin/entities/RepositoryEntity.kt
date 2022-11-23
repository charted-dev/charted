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

package org.noelware.charted.databases.postgres.entities

import org.jetbrains.exposed.dao.LongEntity
import org.jetbrains.exposed.dao.LongEntityClass
import org.jetbrains.exposed.dao.id.EntityID
import org.noelware.charted.databases.postgres.tables.RepositoryMemberTable
import org.noelware.charted.databases.postgres.tables.RepositoryTable

class RepositoryEntity(id: EntityID<Long>): LongEntity(id) {
    companion object: LongEntityClass<RepositoryEntity>(RepositoryTable)

    var description by RepositoryTable.description
    var deprecated by RepositoryTable.deprecated
    var createdAt by RepositoryTable.createdAt
    var updatedAt by RepositoryTable.updatedAt
    var iconHash by RepositoryTable.iconHash
    val members by RepositoryMemberEntity referrersOn RepositoryMemberTable.repository
    var owner by RepositoryTable.owner
    var flags by RepositoryTable.flags
    var name by RepositoryTable.name
    var type by RepositoryTable.type
}
