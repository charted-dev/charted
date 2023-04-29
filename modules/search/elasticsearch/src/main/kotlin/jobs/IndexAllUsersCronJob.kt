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

package org.noelware.charted.modules.search.elasticsearch.jobs

import dev.floofy.utils.slf4j.logging
import kotlinx.atomicfu.atomic
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.joinAll
import kotlinx.coroutines.withContext
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.noelware.charted.ChartedScope
import org.noelware.charted.launch
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.search.SearchModule
import org.noelware.charted.modules.search.elasticsearch.DefaultElasticsearchModule
import org.noelware.charted.modules.search.elasticsearch.ElasticsearchModule
import org.noelware.charted.modules.tasks.scheduling.jobs.CronJob
import java.io.ByteArrayOutputStream

class IndexAllUsersCronJob(search: SearchModule, private val json: Json): CronJob(
    "index users into elasticsearch",
    "@hourly",
) {
    private val isStillExecuting = atomic(false)
    private val elasticsearch = search as ElasticsearchModule
    private val log by logging<IndexAllUsersCronJob>()

    override suspend fun execute() {
        if (!isStillExecuting.compareAndSet(expect = false, update = true)) {
            log.warn("We are still indexing! Please wait until we are done!")
            return
        }

        log.info("Now performing indexing on all users...")

        val users = asyncTransaction { UserEntity.all() }
        val jobs = mutableListOf<Job>()
        for ((bucket, usrs) in users.chunked(1000).withIndex()) {
            log.info("Performing indexing on bucket #$bucket with ${usrs.size} users!")
            jobs.add(
                ChartedScope.launch {
                    val baos = ByteArrayOutputStream()
                    for (user in usrs) {
                        withContext(Dispatchers.IO) {
                            baos.write("""{"index":{"_id":${user.id.value}}}""".toByteArray())
                            baos.write('\n'.code)
                            baos.write(json.encodeToString(User.fromEntity(user)).toByteArray())
                            baos.write('\n'.code)
                        }
                    }

                    baos.use { (elasticsearch as DefaultElasticsearchModule).runBulkRequest("charted-users", it) }
                },
            )
        }

        jobs.joinAll()
    }
}
