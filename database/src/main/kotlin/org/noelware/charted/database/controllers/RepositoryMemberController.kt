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

package org.noelware.charted.database.controllers

import dev.floofy.utils.exposed.asyncTransaction
import org.jetbrains.exposed.sql.Op
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.and
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.database.entities.RepositoryMemberEntity
import org.noelware.charted.database.models.RepositoryMember
import org.noelware.charted.database.tables.RepositoryMemberTable

object RepositoryMemberController {
    suspend fun getAll(repo: Long, showPrivate: Boolean = false): List<RepositoryMember> {
        val query: Op<Boolean> = if (showPrivate) { RepositoryMemberTable.id eq repo } else {
            (RepositoryMemberTable.id eq repo) and (RepositoryMemberTable.publicVisibility eq true)
        }

        return asyncTransaction(ChartedScope) {
            RepositoryMemberEntity
                .find(query)
                .toList()
                .map { entity ->
                    RepositoryMember.fromEntity(entity)
                }
        }
    }

    suspend fun get(repo: Long, member: Long): RepositoryMember? = asyncTransaction(ChartedScope) {
        RepositoryMemberEntity
            .find { (RepositoryMemberTable.repository eq repo) and (RepositoryMemberTable.id eq member) }
            .firstOrNull()
            ?.let { entity ->
                RepositoryMember.fromEntity(entity)
            }
    }

    suspend fun invite(): Boolean {
        return true
    }
}
