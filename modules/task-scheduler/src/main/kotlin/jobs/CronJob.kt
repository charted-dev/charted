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

package org.noelware.charted.modules.tasks.scheduling.jobs

import com.cronutils.model.time.ExecutionTime
import dev.floofy.utils.java.SetOnce
import kotlinx.coroutines.Job
import org.noelware.charted.common.extensions.setonce.getValue
import org.noelware.charted.common.extensions.setonce.setValue
import java.time.ZonedDateTime

/**
 * Represents a way to construct jobs using CRON. The task scheduler will keep
 * track of the current state, and you can create children jobs that will be
 * pushed into its own queue.
 *
 * @param name The name of this CronJob
 * @param expression The cron expression to evaluate
 */
@Suppress("MemberVisibilityCanBePrivate")
abstract class CronJob(val name: String, val expression: String) {
    private var _nextDelayMillis: Long? = null
    private val _executionTime: SetOnce<ExecutionTime> = SetOnce()
    private val _job: SetOnce<Job> = SetOnce()

    /**
     * Millisecond-precision timestamp of when the next invocation of this
     * [CronJob] will be executed in.
     */
    internal val nextDelayMillis: Long? = _nextDelayMillis

    /**
     * The [ExecutionTime] that this [CronJob] was parsed by. Only the task
     * scheduler can edit
     */
    internal var executionTime: ExecutionTime? by _executionTime

    /**
     * Returns the inner coroutine job that this [CronJob] holds.
     */
    internal var job: Job? by _job

    /**
     * Executes this [CronJob].
     */
    abstract suspend fun execute()

    internal fun getAndUpdateNextDelayMillis() {
        requireNotNull(executionTime) { "Job must've be scheduled before-hand when updating state (${this::class})" }

        val delay = executionTime?.nextExecution(ZonedDateTime.now())?.orElse(null)?.toInstant()?.toEpochMilli()
        requireNotNull(delay) { "`delay` cannot be null. Did we exceeded time overflow?" }

        val now = ZonedDateTime.now().toInstant().toEpochMilli()
        _nextDelayMillis = ZonedDateTime.now().toInstant().toEpochMilli() - now
    }
}
