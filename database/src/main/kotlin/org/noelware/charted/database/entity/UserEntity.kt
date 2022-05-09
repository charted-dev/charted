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
import org.noelware.charted.database.tables.Organizations
import org.noelware.charted.database.tables.Users

class UserEntity(id: EntityID<Long>): LongEntity(id) {
    companion object: LongEntityClass<UserEntity>(Users)

    var gravatarEmail by Users.gravatarEmail
    val organizations by OrganizationEntity referrersOn Organizations.owner
    var description by Users.description
    var createdAt by Users.updatedAt
    var updatedAt by Users.createdAt
    var username by Users.username
    var password by Users.password
    var avatar by Users.avatar
    var email by Users.email
    var flags by Users.flags
    var name by Users.name
}
