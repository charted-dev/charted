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

package org.noelware.charted.apikeys

import dev.floofy.utils.exposed.asyncTransaction
import io.lettuce.core.SetArgs
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
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
import kotlin.time.Duration.Companion.seconds
import kotlin.time.toJavaDuration

class DefaultApiKeyManager(private val redis: IRedisClient): ApiKeyManager {
    private val jobs = mutableMapOf<String, Job>()

    init {
        val expirations = runBlocking { redis.commands.keys("charted:apikeys:*").await() }
        for (key in expirations) {
            val ttl = runBlocking { redis.commands.ttl(key).await() }

            val id = key.split(':').last().toLong()
            if (ttl == -2L) continue
            if (ttl == -1L) {
                runBlocking {
                    asyncTransaction(ChartedScope) {
                        ApiKeysTable.deleteWhere { ApiKeysTable.id eq id }
                    }

                    redis.commands.hdel("charted:apikeys", key)
                }
            } else {
                jobs[key] = ChartedScope.launch {
                    delay(ttl.seconds.inWholeMilliseconds)
                    asyncTransaction(ChartedScope) {
                        ApiKeysTable.deleteWhere { ApiKeysTable.id eq id }
                    }

                    redis.commands.hdel("charted:apikeys", key)
                }
            }
        }
    }

    override suspend fun createApiKey(
        name: String,
        description: String?,
        owner: Long,
        scopes: Long,
        expiresIn: Duration?
    ): ApiKeys {
        val token = RandomGenerator.generate(128)
        val id = Snowflake.generate()

        val hashedToken = SHAUtils.sha256(token)
        if (expiresIn != null) {
            redis.commands.set("charted:apikeys:$id", "What are you doing in here, trying to steal something? You know, you could be caught, you know~ <3", SetArgs().ex(expiresIn.toJavaDuration())).await()
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

    override fun close() {
        for (job in jobs.values) job.cancel()
    }
}
