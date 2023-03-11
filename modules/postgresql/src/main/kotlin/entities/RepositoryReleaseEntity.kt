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

package org.noelware.charted.modules.postgresql.entities

import org.jetbrains.exposed.dao.LongEntity
import org.jetbrains.exposed.dao.LongEntityClass
import org.jetbrains.exposed.dao.id.EntityID
import org.noelware.charted.modules.postgresql.tables.RepositoryReleaseTable

class RepositoryReleaseEntity(id: EntityID<Long>): LongEntity(id) {
    companion object: LongEntityClass<RepositoryReleaseEntity>(RepositoryReleaseTable)

    var repository by RepositoryEntity referencedOn RepositoryReleaseTable.repository
    var updateText by RepositoryReleaseTable.updateText
    var createdAt by RepositoryReleaseTable.createdAt
    var updatedAt by RepositoryReleaseTable.updatedAt
    var tag by RepositoryReleaseTable.tag
}
