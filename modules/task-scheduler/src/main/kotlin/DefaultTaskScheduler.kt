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

package org.noelware.charted.modules.tasks.scheduling

import dev.floofy.utils.koin.retrieveAll
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.CancellationException
import org.koin.core.context.GlobalContext
import org.noelware.charted.modules.tasks.scheduling.jobs.CronJob

private const val MAX_POOL_SIZE_PROPERTY: String = "org.noelware.charted.tasks.maxPoolSize"

class DefaultTaskScheduler(
    numThreads: Int = System.getProperty(MAX_POOL_SIZE_PROPERTY).ifNotNull { toInt() }
        ?: (Runtime.getRuntime().availableProcessors()).coerceAtLeast(1)
): TaskScheduler {
    private val _jobs: MutableList<CronJob> = mutableListOf()
    private val log by logging<DefaultTaskScheduler>()
    private val pool = TaskSchedulerWorkerPool(numThreads)

    override val jobs: List<CronJob>
        get() = _jobs

    override fun scheduleAll() {
        val jobs = GlobalContext.retrieveAll<CronJob>()
        for (job in jobs) {
            if (_jobs.contains(job)) {
                log.warn("CronJob [${job.name}] was already registered, skipping.")
                continue
            }

            log.trace("Found CronJob [${job.name}] (${this::class})")

            // create the job and assign the coroutine-based job
            // to the cron job.
            val coroutineJob = pool.spawnJob(job)
            job.job = coroutineJob

            _jobs.add(job)
        }
    }

    override fun unscheduleAll() {
        for (job in jobs) {
            job.job?.cancel(CancellationException("Requested by TaskScheduler"))
        }

        _jobs.clear()
    }
}
