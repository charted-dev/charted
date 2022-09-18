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
import org.jetbrains.exposed.sql.and
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.CryptoUtils
import org.noelware.charted.database.entities.ApiKeyEntity
import org.noelware.charted.database.models.ApiKeys
import org.noelware.charted.database.tables.ApiKeysTable

object ApiKeyController {
    suspend fun get(owner: Long, name: String): ApiKeys? = asyncTransaction(ChartedScope) {
        ApiKeyEntity.find { (ApiKeysTable.name eq name) and (ApiKeysTable.owner eq owner) }.firstOrNull()?.let { entity ->
            ApiKeys.fromEntity(entity)
        }
    }

    suspend fun getByToken(token: String): ApiKeys? = asyncTransaction(ChartedScope) {
        val hashedToken = CryptoUtils.sha256Hex(token)
        ApiKeyEntity.find { ApiKeysTable.token eq hashedToken }.firstOrNull()?.let { entity ->
            ApiKeys.fromEntity(entity)
        }
    }

    suspend fun getAll(owner: Long): List<ApiKeys> = asyncTransaction(ChartedScope) {
        ApiKeyEntity.find {
            ApiKeysTable.owner eq owner
        }.map { entity -> ApiKeys.fromEntity(entity) }
    }

    suspend fun delete(token: String): Boolean {
        // Check if the hashes are the same
        val hashed = CryptoUtils.sha256Hex(token)
        asyncTransaction(ChartedScope) {
            ApiKeyEntity.find { ApiKeysTable.token eq hashed }.firstOrNull()
        } ?: return false

        return asyncTransaction(ChartedScope) {
            ApiKeysTable.deleteWhere { ApiKeysTable.token eq hashed }
            true
        }
    }
}
