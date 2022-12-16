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

package org.noelware.charted.modules.apikeys

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.slf4j.logging
import io.lettuce.core.SetArgs
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.apache.commons.lang3.time.StopWatch
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.ChartedScope
import org.noelware.charted.databases.postgres.models.ApiKeys
import org.noelware.charted.databases.postgres.tables.ApiKeysTable
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.modules.redis.RedisClient
import java.util.*
import kotlin.time.Duration
import kotlin.time.Duration.Companion.seconds
import kotlin.time.DurationUnit
import kotlin.time.toDuration
import kotlin.time.toJavaDuration

private const val API_KEY_EXPIRATION_KEY = "charted:apikeys:expiration"

class DefaultApiKeyManager(private val redis: RedisClient): ApiKeyManager {
    private val expirationJobs: MutableMap<Long, Job> = mutableMapOf()
    private val log by logging<DefaultApiKeyManager>()

    init {
        log.info("Collecting all API key expirations from Redis...")
        val sw = StopWatch.createStarted()
        val sessions = runBlocking { redis.commands.keys("$API_KEY_EXPIRATION_KEY:*").await() }

        sw.suspend()
        log.info("Took ${sw.doFormatTime()} to collect ${sessions.size} sessions from Redis!")
        for (key in sessions) {
            sw.resume()

            log.debug("Collecting TTL for session [$key]")
            val ttl = runBlocking { redis.commands.ttl(key).await() }
            sw.suspend()

            if (ttl == -1L) {
                log.warn("API key $key has expired [${sw.doFormatTime()}]")
                runBlocking {
                    redis.commands.del(key).await()
                    asyncTransaction(ChartedScope) {
                        ApiKeysTable.deleteWhere { ApiKeysTable.id eq key.toLong() }
                    }
                }
            } else {
                log.debug("API key [$key] expires in ${ttl.seconds} seconds [${sw.doFormatTime()}]")
                expirationJobs[key.toLong()] = ChartedScope.launch {
                    delay((ttl.toDuration(DurationUnit.SECONDS)).inWholeMilliseconds)
                    redis.commands.del(key).await()
                    asyncTransaction(ChartedScope) {
                        ApiKeysTable.deleteWhere { ApiKeysTable.id eq key.toLong() }
                    }
                }
            }
        }
    }

    /**
     * Accepts that the API key will be expiring in the specified [duration][expiresIn] and is sent in Redis.
     * @param apiKey    The API key that is expiring
     * @param expiresIn The duration of when the API key will expire
     */
    override suspend fun send(apiKey: ApiKeys, expiresIn: Duration) {
        redis.commands.set("$API_KEY_EXPIRATION_KEY:${apiKey.id}", "<nothing to see here~!>", SetArgs().ex(expiresIn.toJavaDuration())).await()
        expirationJobs[apiKey.id] = ChartedScope.launch {
            delay(expiresIn.inWholeMilliseconds)
            redis.commands.del("$API_KEY_EXPIRATION_KEY:${apiKey.id}").await()
            asyncTransaction(ChartedScope) {
                ApiKeysTable.deleteWhere { ApiKeysTable.id eq apiKey.id }
            }
        }
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * As noted in [AutoCloseable.close], cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * *mark* the `Closeable` as closed, prior to throwing
     * the `IOException`.
     *
     * @throws java.io.IOException if an I/O error occurs
     */
    override fun close() {
        TODO("Not yet implemented")
    }
}
