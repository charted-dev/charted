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

package org.noelware.charted.server.bootstrap

import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.koin.inject
import dev.floofy.utils.koin.injectOrNull
import dev.floofy.utils.slf4j.*
import kotlinx.coroutines.cancel
import kotlinx.coroutines.runBlocking
import okhttp3.internal.closeQuietly
import org.koin.core.context.GlobalContext
import org.noelware.charted.ChartedInfo
import org.noelware.charted.ChartedScope
import org.noelware.charted.databases.clickhouse.ClickHouseConnection
import org.noelware.charted.extensions.formatToSize
import org.noelware.charted.modules.analytics.AnalyticsDaemon
import org.noelware.charted.modules.elasticsearch.ElasticsearchModule
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.server.Bootstrap
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.hasStarted
import java.io.File
import java.io.IOError
import java.lang.management.ManagementFactory
import kotlin.concurrent.thread
import kotlin.system.exitProcess

object OnStartPhase: BootstrapPhase() {
    private val log by logging<OnStartPhase>()
    private val codes: Map<(Any) -> Boolean, Int> = mapOf(
        { i: Any -> i is InternalError } to 128,
        { i: Any -> i is OutOfMemoryError } to 127,
        { i: Any -> i is StackOverflowError } to 126,
        { i: Any -> i is UnknownError } to 125,
        { i: Any -> i is IOError } to 124,
        { i: Any -> i is LinkageError } to 123
    )

    private fun installShutdownHook() {
        Runtime.getRuntime().addShutdownHook(
            thread(name = "Server-ShutdownThread", start = false) {
                log.warn("API server is shutting down...")

                val koin = GlobalContext.getKoinApplicationOrNull()
                if (koin != null) {
                    val analyticsDaemon: AnalyticsDaemon? by injectOrNull()
                    val elasticsearch: ElasticsearchModule? by injectOrNull()
                    val clickhouse: ClickHouseConnection? by injectOrNull()
                    val sessions: SessionManager by inject()
                    val hikari: HikariDataSource by inject()
                    val server: ChartedServer by inject()
                    val redis: RedisClient by inject()

                    if (Bootstrap.analyticsDaemonThread.wasSet()) {
                        Bootstrap.analyticsDaemonThread.value.interrupt()
                    }

                    analyticsDaemon?.closeQuietly()
                    elasticsearch?.closeQuietly()
                    clickhouse?.closeQuietly()
                    sessions.closeQuietly()
                    hikari.closeQuietly()
                    redis.closeQuietly()
                    server.closeQuietly()

                    runBlocking {
                        ChartedScope.cancel()
                    }

                    koin.close()
                } else {
                    log.warn("Koin was not initialized, skipping...")
                }

                log.warn("charted-server has completely shutdown, goodbye! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡")
            }
        )
    }

    // credit: https://github.com/elastic/logstash/blob/main/logstash-core/src/main/java/org/logstash/Logstash.java#L98-L133
    private fun installDefaultThreadExceptionHandler() {
        Thread.setDefaultUncaughtExceptionHandler { thread, ex ->
            if (ex is Error) {
                log.error("Uncaught fatal error had occurred in thread [${thread.name} (#${thread.id}):", ex)
                log.error("If this keeps occurring, report it to Noelware: https://github.com/charted-dev/charted/issues")

                for ((func, int) in codes) {
                    if (func(ex)) {
                        Runtime.getRuntime().halt(int)
                    }
                }

                exitProcess(1)
            } else {
                log.error("Uncaught exception occurred in thread [${thread.name} (#${thread.id}):", ex)

                val hadStarted = hasStarted.get()
                if (!hadStarted && (thread.name matches "Server-(Shutdown|Bootstrap)Thread".toRegex())) {
                    exitProcess(128)
                }
            }
        }
    }

    override suspend fun bootstrap(configPath: File) {
        installDefaultThreadExceptionHandler()
        installShutdownHook()

        val runtime = Runtime.getRuntime()
        val os = ManagementFactory.getOperatingSystemMXBean()

        log.info("==> Initializing charted-server v${ChartedInfo.version} (${ChartedInfo.commitHash})")
        log.info("==> Memory: total=${runtime.totalMemory().formatToSize()} free=${runtime.freeMemory().formatToSize()}")
        log.info("==> Kotlin: ${KotlinVersion.CURRENT}")
        log.info("==> JVM:    version=${System.getProperty("java.version")} vendor=${System.getProperty("java.vendor")}")
        log.info("==> OS:     ${os.name.lowercase()}/${os.arch} with ${os.availableProcessors} processors")

        if (ChartedInfo.dedicatedNode != null) {
            log.info("==> Dedicated Node: ${ChartedInfo.dedicatedNode}")
        }

        log.info("===> JVM Arguments: [${ManagementFactory.getRuntimeMXBean().inputArguments.joinToString(" ")}]")
        for (pool in ManagementFactory.getMemoryPoolMXBeans())
            log.info("===> Memory Pool [${pool.name} <${pool.type}>] ~> ${pool.peakUsage}")
    }
}
