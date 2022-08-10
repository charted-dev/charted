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

            stats.add(CoroutineStats(
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
            ))
        }

        return stats.toList()
    }
}
