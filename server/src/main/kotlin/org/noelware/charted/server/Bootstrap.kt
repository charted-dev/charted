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

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import dev.floofy.utils.slf4j.logging
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.modules.EmptySerializersModule
import org.koin.core.context.GlobalContext
import java.io.File
import java.io.IOError
import java.util.*
import kotlin.concurrent.thread
import kotlin.system.exitProcess

object Bootstrap {
    private val log by logging<Bootstrap>()

    private fun createUUID() {
        val file = File("./instance.uuid")
        if (!file.exists()) {
            file.writeBytes(UUID.randomUUID().toString().toByteArray())
            log.warn("Instance UUID didn't exist in ./instance.uuid, so I created it!")
            log.warn("If this was used with Noelware Analytics, edit the instance!!")
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

                // Check if Koin has started
                val koinStarted = GlobalContext.getKoinApplicationOrNull() != null
                if (koinStarted) {
                } else {
                    log.warn("Koin was not started, not destroying server (just yet!)")
                }

                log.warn("charted-server has completely shutdown, goodbye! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡")
            }
        )
    }

    // credit: https://github.com/elastic/logstash/blob/main/logstash-core/src/main/java/org/logstash/Logstash.java#L98-L133
    private fun installDefaultThreadExceptionHandler() {
        Thread.setDefaultUncaughtExceptionHandler { t, e ->
            if (e is Error) {
                log.error("Uncaught fatal error in thread ${t.name} (#${t.id}):", e)
                log.error("If this keeps occurring, please report it to Noelware: https://github.com/charted-dev/charted/issues")

                var success = false

                if (e is InternalError) {
                    success = true
                    halt(128)
                }

                if (e is OutOfMemoryError) {
                    success = true
                    halt(127)
                }

                if (e is StackOverflowError) {
                    success = true
                    halt(126)
                }

                if (e is UnknownError) {
                    success = true
                    halt(125)
                }

                if (e is IOError) {
                    success = true
                    halt(124)
                }

                if (e is LinkageError) {
                    success = true
                    halt(123)
                }

                if (!success) halt(120)

                exitProcess(1)
            } else {
                log.error("Uncaught exception in thread ${t.name} (#${t.id}):", e)

                // If any thread had an exception, let's check if:
                //  - The server has started (must be set if the Application hook has run)
                //  - If the thread names are the bootstrap or shutdown thread
                val started = ChartedServer.hasStarted.valueOrNull != null && ChartedServer.hasStarted.value
                if (!started && (t.name == "Server-BootstrapThread" || t.name == "Server-ShutdownThread")) {
                    halt(120)
                    exitProcess(1)
                }
            }
        }
    }

    @OptIn(ExperimentalSerializationApi::class)
    @JvmStatic
    fun main(args: Array<String>) {
        println("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+")
        println("+       _                _           _                                      +")
        println("+   ___| |__   __ _ _ __| |_ ___  __| |      ___  ___ _ ____   _____ _ __   +")
        println("+   / __| '_ \\ / _` | '__| __/ _ \\/ _` |_____/ __|/ _ \\ '__\\ \\ / / _ \\ '__| +")
        println("+  | (__| | | | (_| | |  | ||  __/ (_| |_____\\__ \\  __/ |   \\ V /  __/ |    +")
        println("+   \\___|_| |_|\\__,_|_|   \\__\\___|\\__,_|     |___/\\___|_|    \\_/ \\___|_|    +")
        println("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+")
        println("")

        installDefaultThreadExceptionHandler()
        installShutdownHook()
        createUUID()

        log.info("Loading configuration...")
        val fullConfigPath = System.getenv("CHARTED_CONFIG_PATH") ?: "./config.yml"
        val configFile = File(fullConfigPath)

        if (!configFile.exists()) {
            log.error("Missing configuration file in path '$configFile'!")
            exitProcess(1)
        }

        if (configFile.extension != "yml" || configFile.extension != "yaml") {
            log.error("Configuration file at path $configFile must be a YAML file. (`.yml` or `.yaml` extensions)")
            exitProcess(1)
        }

        val yaml = Yaml(
            EmptySerializersModule,
            YamlConfiguration(
                encodeDefaults = true,
                strictMode = true
            )
        )
    }
}
