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

package org.noelware.charted.core.apikeys

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.apache.commons.lang3.time.StopWatch
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.database.tables.ApiKeysTable
import java.io.Closeable
import java.util.concurrent.TimeUnit
import kotlin.time.Duration.Companion.seconds

class TokenExpirationManager(private val redis: IRedisClient): Closeable {
    private val expirationJobs = mutableListOf<Job>()
    private val log by logging<TokenExpirationManager>()

    init {
        log.info("Creating expiration jobs for API keys...")
        val tokens = runBlocking { redis.commands.keys("apikeys:*").await() }

        log.info("Found ${tokens.size} API keys~")
        val sw = StopWatch.createStarted()
        for (key in tokens) {
            val ttl = runBlocking { redis.commands.ttl(key).await() }

            val id = key.split(':').last().toLong()
            if (ttl == -2L) continue
            if (ttl == -1L) {
                log.warn("API key [$id] has expired!")

                runBlocking {
                    asyncTransaction(ChartedScope) {
                        ApiKeysTable.deleteWhere { ApiKeysTable.id eq id }
                    }

                    redis.commands.del(key)
                }
            } else {
                log.info("API key [$id] expires in ${ttl.seconds.inWholeSeconds} seconds")
                expirationJobs.add(
                    ChartedScope.launch {
                        delay(ttl.seconds.inWholeMilliseconds)

                        asyncTransaction(ChartedScope) {
                            ApiKeysTable.deleteWhere { ApiKeysTable.id eq id }
                        }

                        redis.commands.del(key)
                    }
                )
            }
        }

        sw.stop()
        log.info("Took ${sw.getTime(TimeUnit.MILLISECONDS)}ms to create expiration jobs!")
    }

    override fun close() {
        log.warn("Closing ${expirationJobs.size} jobs...")
        for (job in expirationJobs) job.cancel()

        log.warn("Done!")
    }
}
