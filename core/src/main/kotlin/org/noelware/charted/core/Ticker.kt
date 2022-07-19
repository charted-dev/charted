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

package org.noelware.charted.core

import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.*
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.SetOnceGetValue
import kotlin.time.Duration
import kotlin.time.Duration.Companion.seconds

class Ticker(private val name: String, private val interval: Duration = 5.seconds): CoroutineScope by ChartedScope {
    private val _job = SetOnceGetValue<Job>()
    private val log by logging<Ticker>()

    fun launch(block: suspend () -> Unit): Job {
        if (_job.wasSet()) throw IllegalStateException("Can't create new ticker job.")
        _job.value = launch(start = CoroutineStart.DEFAULT) {
            delay(interval.inWholeMilliseconds)
            while (isActive) {
                try {
                    block()
                } catch (e: Exception) {
                    log.error("Unable to execute ticker with name $name:", e)
                }

                delay(interval.inWholeMilliseconds)
            }
        }

        return _job.value
    }

    fun cancel() {
        if (!_job.wasSet()) return
        _job.value.cancel()
    }
}
