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

import com.cronutils.model.CronType
import com.cronutils.model.definition.CronDefinitionBuilder
import com.cronutils.model.time.ExecutionTime
import com.cronutils.parser.CronParser
import dev.floofy.utils.kotlin.threading.createThreadFactory
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import io.sentry.kotlin.SentryContext
import kotlinx.coroutines.*
import org.noelware.charted.modules.tasks.scheduling.jobs.CronJob
import java.time.Duration
import java.time.ZonedDateTime
import java.util.concurrent.Executors
import kotlin.coroutines.CoroutineContext
import kotlin.jvm.optionals.getOrNull
import kotlin.time.toKotlinDuration

class TaskSchedulerWorkerPool(numThreads: Int): CoroutineScope {
    private val commonExpressions = mapOf(
        "@daily" to "0 0 * * *",
        "@yearly" to "0 0 1 1 *",
        "@annually" to "0 0 1 1 *",
        "@monthly" to "0 0 1 * *",
        "@weekly" to "0 0 * * 0",
        "@hourly" to "0 * * * *",
    )

    private val parser = CronParser(CronDefinitionBuilder.instanceDefinitionFor(CronType.UNIX))
    private val log by logging<TaskSchedulerWorkerPool>()

    override val coroutineContext: CoroutineContext =
        Job() + Executors.newFixedThreadPool(numThreads, createThreadFactory("Charted-TaskSchedulerPool")).asCoroutineDispatcher()

    internal fun spawnJob(job: CronJob): Job {
        // calculate the time for the initial delay
        val now = ZonedDateTime.now()
        val executionTime = ExecutionTime.forCron(parser.parse(if (commonExpressions.containsKey(job.expression)) commonExpressions[job.expression]!! else job.expression))
        job.executionTime = executionTime

        val nextScheduledTime = executionTime.nextExecution(now).getOrNull()
            ?: throw IllegalStateException("Cannot get next execution time for job [${job.name}], did we overflow time?")

        val delayInMillis = Duration.between(now, nextScheduledTime)
        return launch(
            if (Sentry.isEnabled()) SentryContext() + coroutineContext else coroutineContext,
        ) {
            delay(delayInMillis.toKotlinDuration())
            while (isActive) {
                try {
                    job.execute()
                } catch (e: Throwable) {
                    log.error("Unable to execute job [${job.name}]", e)
                }

                job.getAndUpdateNextDelayMillis()
                delay(job.nextDelayMillis!!)
            }
        }
    }
}
