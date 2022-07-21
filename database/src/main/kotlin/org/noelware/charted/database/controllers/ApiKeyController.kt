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
import dev.floofy.utils.koin.inject
import io.lettuce.core.SetArgs
import kotlinx.coroutines.future.await
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.and
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.RandomGenerator
import org.noelware.charted.common.SHAUtils
import org.noelware.charted.common.Snowflake
import org.noelware.charted.database.entities.ApiKeyEntity
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.models.ApiKeys
import org.noelware.charted.database.tables.ApiKeysTable
import kotlin.time.Duration

object ApiKeyController {
    suspend fun get(owner: Long, name: String): ApiKeys? = asyncTransaction(ChartedScope) {
        ApiKeyEntity.find { (ApiKeysTable.name eq name) and (ApiKeysTable.owner eq owner) }.firstOrNull()?.let { entity ->
            ApiKeys.fromEntity(entity)
        }
    }

    suspend fun getByToken(token: String): ApiKeys? = asyncTransaction(ChartedScope) {
        val hashedToken = SHAUtils.sha256(token)
        println("token = $token; hashed = $hashedToken")
        ApiKeyEntity.find { ApiKeysTable.token eq hashedToken }.firstOrNull()?.let { entity ->
            ApiKeys.fromEntity(entity)
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
        expiresIn: Duration? = null
    ): ApiKeys {
        val redis: IRedisClient by inject()
        val token = RandomGenerator.generate(64)
        val id = Snowflake.generate()

        // Hash the api key, so it won't be exposed from the database.
        val hashedToken = SHAUtils.sha256(token)
        if (expiresIn != null) {
            redis.commands.set(
                "apikeys:$id",
                "this shouldn't be anything :D",
                SetArgs().apply {
                    ex(expiresIn.inWholeSeconds)
                }
            ).await()
        }

        return asyncTransaction(ChartedScope) {
            val user = UserEntity.findById(owner)!!

            ApiKeyEntity.new(id) {
                this.expiresIn = if (expiresIn != null) Clock.System.now().plus(expiresIn).toLocalDateTime(TimeZone.currentSystemDefault()) else null
                this.scopes = scopes
                this.owner = user
                this.token = hashedToken
                this.name = name
            }.let { entity -> ApiKeys.fromEntity(entity, token) }
        }
    }

    suspend fun delete(token: String): Boolean {
        // Check if the hashes are the same
        val hashed = SHAUtils.sha256(token)
        asyncTransaction(ChartedScope) {
            ApiKeyEntity.find { ApiKeysTable.token eq hashed }.firstOrNull()
        } ?: return false

        return asyncTransaction(ChartedScope) {
            ApiKeysTable.deleteWhere { ApiKeysTable.token eq hashed }
            true
        }
    }
}
