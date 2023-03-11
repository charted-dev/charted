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

package org.noelware.charted.server.bootstrap

import dev.floofy.utils.kotlin.sizeToStr
import dev.floofy.utils.slf4j.logging
import org.noelware.charted.ChartedInfo
import org.noelware.charted.server.internal.hasStarted
import org.slf4j.MDC
import java.io.File
import java.io.IOError
import java.lang.management.ManagementFactory
import kotlin.system.exitProcess

object PreinitPhase: BootstrapPhase() {
    private val codes: Map<(Any) -> Boolean, Int> = mapOf(
        { i: Any -> i is InternalError } to 128,
        { i: Any -> i is OutOfMemoryError } to 127,
        { i: Any -> i is StackOverflowError } to 126,
        { i: Any -> i is UnknownError } to 125,
        { i: Any -> i is IOError } to 124,
        { i: Any -> i is LinkageError } to 123,
    )

    private val log by logging<PreinitPhase>()

    // credit: https://github.com/elastic/logstash/blob/main/logstash-core/src/main/java/org/logstash/Logstash.java#L98-L133
    private fun installDefaultThreadExceptionHandler() {
        Thread.setDefaultUncaughtExceptionHandler { thread, ex ->
            if (ex is Error) {
                // TODO(@auguwu): use threadId() instead of id when targeting next LTS
                log.error("Uncaught fatal exception occurred in thread [${thread.name} (${thread.id})]:", ex)
                log.error("If this keep occurring, please report it to Noelware: https://github.com/charted-dev/charted/issues/new")

                for ((func, exitCode) in codes) {
                    if (func(ex)) {
                        // use the appropriate exit code
                        Runtime.getRuntime().halt(exitCode)
                    }
                }

                exitProcess(1)
            } else {
                log.error("Uncaught exception occurred in thread [${thread.name} (${thread.id})]:", ex)

                // If we are in bootstrapping or shutting down, let's just halt the process
                // (for now), there might be a better solution
                val isInBootstrapOrShutdown = thread.name matches "Server-(Shutdown|Bootstrap)Thread".toRegex()
                if (!hasStarted.value && isInBootstrapOrShutdown) {
                    // we use #halt(Int) instead of exitProcess to not run the
                    // shutdown phase thread -- just in case
                    Runtime.getRuntime().halt(120)
                }
            }
        }
    }

    override suspend fun phaseThrough(config: File) {
        // If we ever need to debug this, we will know what phase it
        // was passing through.
        MDC.put("bootstrap.phase", "preinit")

        installDefaultThreadExceptionHandler()

        val runtime = Runtime.getRuntime()
        runtime.addShutdownHook(ShutdownPhaseThread)

        val os = ManagementFactory.getOperatingSystemMXBean()
        log.info("Initializing charted-server v${ChartedInfo.version} (commit hash=${ChartedInfo.commitHash} build date=${ChartedInfo.buildDate} distribution=${ChartedInfo.distribution})")
        log.info("~> Memory: total=${runtime.totalMemory().sizeToStr()} free=${runtime.freeMemory().sizeToStr()}")
        log.info("~> Kotlin: ${KotlinVersion.CURRENT}")
        log.info("~> JVM:    ${System.getProperty("java.version")} [vendor=${System.getProperty("java.vendor")}]")
        log.info("~> OS:     ${os.name.lowercase()}/${os.arch} with ${os.availableProcessors} processors")

        if (ChartedInfo.dedicatedNode != null) {
            log.info("~> Dedicated Node: ${ChartedInfo.dedicatedNode}")
        }

        log.info("~> JVM Arguments: [${ManagementFactory.getRuntimeMXBean().inputArguments.joinToString(" ")}]")
        for (pool in ManagementFactory.getMemoryPoolMXBeans())
            log.info("~> Memory Pool [${pool.name} <${pool.type}>] ~> ${pool.peakUsage}")

        MDC.remove("bootstrap.phase")
    }
}
