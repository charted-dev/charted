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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.server

import dev.floofy.utils.kotlin.sizeToStr
import dev.floofy.utils.slf4j.logging
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import kotlinx.coroutines.Job
import org.noelware.charted.analytics.AnalyticsServer
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.config.Config
import org.noelware.charted.common.launch
import org.slf4j.LoggerFactory
import java.lang.management.ManagementFactory

class ChartedServer(private val config: Config) {
    companion object {
        val bootTime = System.currentTimeMillis()
        val hasStarted: SetOnceGetValue<Boolean> = SetOnceGetValue()
    }

    private lateinit var analyticsJob: Job
    private lateinit var analytics: AnalyticsServer
    private lateinit var server: NettyApplicationEngine
    private val log by logging<ChartedServer>()

    suspend fun start() {
        val runtime = Runtime.getRuntime()
        val os = ManagementFactory.getOperatingSystemMXBean()
        val threads = ManagementFactory.getThreadMXBean()

        log.info("Runtime Information:")
        log.info("  * Free / Total Memory [Max]: ${runtime.freeMemory().sizeToStr()}/${runtime.totalMemory().sizeToStr()} [${runtime.maxMemory().sizeToStr()}]")
        log.info("  * Threads: ${threads.threadCount} (${threads.daemonThreadCount} background threads)")
        log.info("  * Operating System: ${os.name} with ${os.availableProcessors} processors (${os.arch}; ${os.version})")
        log.info("  * Versions:")
        log.info("      * JVM [JRE]: v${System.getProperty("java.version", "Unknown")} (${System.getProperty("java.vendor", "Unknown")}) [${Runtime.version()}]")
        log.info("      * Kotlin:    v${KotlinVersion.CURRENT}")
        log.info("      * charted:   v${ChartedInfo.version} (${ChartedInfo.commitHash} -- ${ChartedInfo.buildDate})")

        if (ChartedInfo.dediNode != null)
            log.info("  * Dedicated Node: ${ChartedInfo.dediNode}")

        val self = this
        val environment = applicationEngineEnvironment {
            developmentMode = self.config.debug
            log = LoggerFactory.getLogger("org.noelware.charted.server.KtorAppEnvironmentKt")

            connector {
                host = self.config.server.host
                port = self.config.server.port
            }

            module {
                module(self.config)
            }
        }

        server = embeddedServer(Netty, environment, configure = {
            requestQueueLimit = config.server.requestQueueLimit
            runningLimit = config.server.runningLimit
            shareWorkGroup = config.server.shareWorkGroup
            responseWriteTimeoutSeconds = config.server.responseWriteTimeoutSeconds
            requestReadTimeoutSeconds = config.server.requestReadTimeout
            tcpKeepAlive = config.server.tcpKeepAlive
        })

        if (config.analytics.enabled) {
            analyticsJob = ChartedScope.launch {
                analytics = AnalyticsServer(config.analytics)
                analytics.start()
            }
        }

        server.start(wait = true)
    }

    fun destroy() {
        if (!::server.isInitialized) return
        if (this::analytics.isInitialized) {
            analytics.close()
            analyticsJob.cancel()
        }

        server.stop()
    }
}
