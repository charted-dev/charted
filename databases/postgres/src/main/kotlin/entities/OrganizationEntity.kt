/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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
import org.noelware.charted.databases.postgres.tables.OrganizationMemberTable
import org.noelware.charted.databases.postgres.tables.OrganizationTable

class OrganizationEntity(id: EntityID<Long>) : LongEntity(id) {
    companion object : LongEntityClass<OrganizationEntity>(OrganizationTable)

    var verifiedPublisher by OrganizationTable.verifiedPublisher
    var twitterHandle by OrganizationTable.twitterHandle
    var gravatarEmail by OrganizationTable.gravatarEmail
    var displayName by OrganizationTable.displayName
    var createdAt by OrganizationTable.createdAt
    var updatedAt by OrganizationTable.updatedAt
    var iconHash by OrganizationTable.iconHash
    val members by OrganizationMemberEntity referrersOn OrganizationMemberTable.organization
    var owner by UserEntity referencedOn OrganizationTable.owner
    var flags by OrganizationTable.flags
    var name by OrganizationTable.name
}
