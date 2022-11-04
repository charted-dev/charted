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

package org.noelware.charted.server

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.cancel
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.modules.EmptySerializersModule
import org.koin.core.context.GlobalContext
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.ChartedInfo
import org.noelware.charted.ChartedScope
import org.noelware.charted.configuration.host.ConfigurationHost
import org.noelware.charted.configuration.kotlin.host.KotlinScriptHost
import org.noelware.charted.configuration.yaml.YamlConfigurationHost
import org.noelware.charted.extensions.formatToSize
import org.noelware.charted.modules.avatars.avatarsModule
import org.noelware.charted.server.endpoints.v1.endpointsModule
import org.noelware.charted.server.internal.DefaultChartedServer
import java.io.File
import java.io.IOError
import java.lang.management.ManagementFactory
import java.util.*
import kotlin.concurrent.thread
import kotlin.system.exitProcess

/**
 * Represents the server bootstrap, which... bootstraps and loads the server.
 */
object Bootstrap {
    private val log by logging<Bootstrap>()

    private fun createUUID() {
        val env = System.getenv("CHARTED_NO_ANALYTICS") ?: "true"
        if (env.matches("^(yes|true|si|si*|1)$".toRegex())) {
            val file = File("./instance.uuid")
            if (!file.exists()) {
                file.writeBytes(UUID.randomUUID().toString().toByteArray())

                val root = File(".").toPath().toRealPath()
                log.warn("Created instance UUID for Noelware Analytics in [$root/instance.uuid]")
                log.warn("If you do not wish to create this file to identify this product, you can use the `CHARTED_NO_ANALYTICS` environment variable to skip this step.")
                log.warn("If you do wish to use this instance UUID for Noelware Analytics, edit your instance to connect the instance UUID: https://analytics.noelware.org/instances")
            }
        }
    }

    private fun halt(code: Int) {
        Runtime.getRuntime().halt(code)
    }

    private fun installShutdownHook() {
        val runtime = Runtime.getRuntime()
        runtime.addShutdownHook(
            thread(start = false, name = "Server-ShutdownThread") {
                log.warn("Shutting down charted-server...")

                val koin = GlobalContext.getKoinApplicationOrNull()
                if (koin != null) {
                    val server: ChartedServer by inject()
                    server.close()

                    runBlocking {
                        ChartedScope.cancel()
                    }

                    koin.close()
                } else {
                    log.warn("Koin was not started, not destroying server (just yet!)")
                }

                log.warn("charted-server has completely shutdown, goodbye! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡")
            }
        )
    }

    // credit: https://github.com/elastic/logstash/blob/main/logstash-core/src/main/java/org/logstash/Logstash.java#L98-L133
    private fun installDefaultThreadExceptionHandler() {
        Thread.setDefaultUncaughtExceptionHandler { thread, ex ->
            if (ex is Error) {
                log.error("Uncaught fatal error in thread ${thread.name} (#${thread.id}):", ex)
                log.error("If this keeps occurring, please report it to Noelware: https://github.com/charted-dev/charted/issues")

                var hasHalted = false
                if (ex is InternalError) {
                    hasHalted = true
                    halt(128)
                }

                if (ex is OutOfMemoryError) {
                    hasHalted = true
                    halt(127)
                }

                if (ex is StackOverflowError) {
                    hasHalted = true
                    halt(126)
                }

                if (ex is UnknownError) {
                    hasHalted = true
                    halt(125)
                }

                if (ex is IOError) {
                    hasHalted = true
                    halt(124)
                }

                if (ex is LinkageError) {
                    hasHalted = true
                    halt(123)
                }

                if (!hasHalted) halt(120)
                exitProcess(1)
            } else {
                log.error("Uncaught exception in thread ${thread.name} (#${thread.id}):", ex)

                // If any thread had an exception, let's check if:
                //  - The server has started (must be set if the Application hook has run)
                //  - If the thread names are the bootstrap or shutdown thread
                val started = hasStarted.get()
                if (!started && (thread.name == "Server-ShutdownThread" || thread.name == "Server-BootstrapThread")) {
                    halt(120)
                }
            }
        }
    }

    /**
     * Bootstraps and starts the server.
     * @param configPath The configuration path
     */
    @OptIn(ExperimentalCoroutinesApi::class)
    fun start(configPath: File) {
        Thread.currentThread().name = "Charted-BootstrapThread"
        installDefaultThreadExceptionHandler()
        installShutdownHook()
        createUUID()

        val runtime = Runtime.getRuntime()
        val os = ManagementFactory.getOperatingSystemMXBean()

        log.info("===> JVM vendor/version: ${System.getProperty("java.vendor", "Unknown")} [${System.getProperty("java.version")}]")
        log.info("===> Kotlin version: ${KotlinVersion.CURRENT}")
        log.info("===> charted-server version: ${ChartedInfo.version} [${ChartedInfo.commitHash}]")
        log.info("===> Heap size: total=${runtime.totalMemory().formatToSize()} free=${runtime.freeMemory().formatToSize()}")
        log.info("===> Operating System: ${os.name.lowercase()}/${os.arch} (${os.availableProcessors} processors)")
        if (ChartedInfo.dedicatedNode != null) {
            log.info("===> Dedicated Node: ${ChartedInfo.dedicatedNode}")
        }

        for (pool in ManagementFactory.getMemoryPoolMXBeans())
            log.info("===> ${pool.name} <${pool.type}> -> ${pool.peakUsage}")

        log.info("===> JVM Arguments: [${ManagementFactory.getRuntimeMXBean().inputArguments.joinToString(" ")}]")
        val yaml = Yaml(
            EmptySerializersModule(),
            YamlConfiguration(
                encodeDefaults = true,
                strictMode = true
            )
        )

        val configHost: ConfigurationHost = if (listOf("yaml", "yml").contains(configPath.extension)) {
            YamlConfigurationHost(yaml)
        } else if (configPath.extension.contains("kts")) {
            KotlinScriptHost
        } else {
            throw IllegalStateException("Unable to determine which configuration host to use")
        }

        val config = configHost.load(configPath.toPath().toRealPath().toString()) ?: throw IllegalStateException("Unable to load configuration")
        log.info("Loaded configuration in path [${configPath.toPath().toRealPath()}]")

        DebugProbes.enableCreationStackTraces = config.debug
        DebugProbes.install()

        if (config.sentryDsn != null) {
            log.info("Enabling Sentry due to [sentryDsn] was set.")
            Sentry.init {
                it.release = "charted-server v${ChartedInfo.version}+${ChartedInfo.commitHash}"
                it.dsn = config.sentryDsn
            }

            log.info("Sentry is now enabled!")
        }

        val json = Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }

        val koinModule = module {
            single<ChartedServer> { DefaultChartedServer(config) }
            single { config }
            single { yaml }
            single { json }
        }

        val modules = mutableListOf(endpointsModule, avatarsModule, koinModule)
        startKoin {
            modules(*modules.toTypedArray())
        }

        try {
            val server: ChartedServer by inject()
            server.start()
        } catch (e: Exception) {
            log.error("Unable to bootstrap charted-server:", e)

            // we do not let the shutdown hooks run
            // since in some cases, it'll just error out or whatever
            //
            // example: Elasticsearch cannot index all data due to
            // I/O locks or what not (and it'll keep looping)
            halt(1)
        }
    }
}
