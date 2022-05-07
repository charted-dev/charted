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

package org.noelware.charted.database.entity

import org.jetbrains.exposed.dao.LongEntity
import org.jetbrains.exposed.dao.LongEntityClass
import org.jetbrains.exposed.dao.id.EntityID
import org.noelware.charted.database.Repository
import org.noelware.charted.database.RepositoryMember

class RepositoryEntity(id: EntityID<Long>): LongEntity(id) {
    companion object: LongEntityClass<RepositoryEntity>(Repository)

    var description by Repository.description
    var deprecated by Repository.deprecated
    var createdAt by Repository.createdAt
    var updatedAt by Repository.updatedAt
    var members by RepositoryMemberEntity via RepositoryMember
    var homepage by Repository.homepage
    var iconHash by Repository.iconHash
    var ownerId by Repository.ownerId
    var flags by Repository.flags
    var name by Repository.name
    var type by Repository.type
}
