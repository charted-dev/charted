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

package org.noelware.charted.database.controllers

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import io.lettuce.core.SetArgs
import kotlinx.coroutines.future.await
import kotlinx.datetime.LocalDateTime
import org.jetbrains.exposed.sql.and
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.RandomGenerator
import org.noelware.charted.common.Snowflake
import org.noelware.charted.database.entities.ApiKeyEntity
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.models.ApiKeys
import org.noelware.charted.database.tables.ApiKeysTable

object ApiKeyController {
    suspend fun get(owner: Long, name: String): ApiKeys? = asyncTransaction(ChartedScope) {
        ApiKeyEntity.find { (ApiKeysTable.name eq name) and (ApiKeysTable.owner eq owner) }.firstOrNull()?.let { entity ->
            ApiKeys.fromEntity(entity)
        }
    }

    suspend fun getByToken(token: String, showToken: Boolean = false): ApiKeys? = asyncTransaction(ChartedScope) {
        ApiKeyEntity.find { ApiKeysTable.token eq token }.firstOrNull()?.let { entity ->
            ApiKeys.fromEntity(entity, showToken)
        }
    }

    suspend fun getAll(owner: Long): List<ApiKeys> = asyncTransaction(ChartedScope) {
        ApiKeyEntity.find {
            ApiKeysTable.owner eq owner
        }.map { entity -> ApiKeys.fromEntity(entity) }
    }

    suspend fun create(
        name: String,
        owner: Long,
        scopes: Long = 0L,
        expiresIn: LocalDateTime? = null
    ): ApiKeys {
        val redis: IRedisClient by inject()
        val token = RandomGenerator.generate(24)
        val id = Snowflake.generate()

        if (expiresIn != null) {
            redis.commands.set(
                "apikeys:$id",
                "this shouldn't be anything :D",
                SetArgs().apply {
                    ex(expiresIn.second.toLong())
                }
            ).await()
        }

        return asyncTransaction(ChartedScope) {
            val user = UserEntity.findById(owner)!!

            ApiKeyEntity.new(id) {
                this.expiresIn = expiresIn
                this.scopes = scopes
                this.owner = user
                this.token = token
                this.name = name
            }.let { entity -> ApiKeys.fromEntity(entity, true) }
        }
    }

    suspend fun delete(token: String): Boolean = asyncTransaction(ChartedScope) {
        ApiKeysTable.deleteWhere { ApiKeysTable.token eq token }
        true
    }
}
