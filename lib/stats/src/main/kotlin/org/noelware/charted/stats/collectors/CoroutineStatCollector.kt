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

package org.noelware.charted.stats.collectors

import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.debug.State
import org.noelware.charted.stats.StatCollector

@kotlinx.serialization.Serializable
data class CoroutineStats(
    val state: String,
    val context: String,
    val job: CoroutineJobInfo? = null,
    val stacktrace: List<ThreadStackTrace> = listOf()
)

@kotlinx.serialization.Serializable
data class CoroutineJobInfo(
    val active: Boolean,
    val completed: Boolean,
    val cancelled: Boolean
)

class CoroutineStatCollector: StatCollector<List<CoroutineStats>> {
    @OptIn(ExperimentalCoroutinesApi::class)
    override suspend fun collect(): List<CoroutineStats> {
        val coroutines = DebugProbes.dumpCoroutinesInfo()
        val stats = mutableListOf<CoroutineStats>()

        for (coroutine in coroutines) {
            val state = when (coroutine.state) {
                State.CREATED -> "created"
                State.RUNNING -> "running"
                State.SUSPENDED -> "suspended"
            }

            stats.add(
                CoroutineStats(
                    state,
                    coroutine.context.toString(),
                    if (coroutine.job != null) CoroutineJobInfo(
                        coroutine.job!!.isActive,
                        coroutine.job!!.isCompleted,
                        coroutine.job!!.isCancelled
                    ) else null,

                    coroutine.creationStackTrace.map { element ->
                        ThreadStackTrace(
                            element.classLoaderName,
                            element.moduleName,
                            element.moduleVersion,
                            element.className,
                            element.methodName,
                            element.fileName,
                            element.lineNumber,
                            element.isNativeMethod
                        )
                    }
                )
            )
        }

        return stats.toList()
    }
}
