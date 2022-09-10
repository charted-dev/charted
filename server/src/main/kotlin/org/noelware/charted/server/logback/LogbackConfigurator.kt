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

package org.noelware.charted.server.logback

import ch.qos.logback.classic.LoggerContext
import ch.qos.logback.classic.joran.JoranConfigurator
import ch.qos.logback.classic.spi.Configurator
import ch.qos.logback.core.spi.ContextAwareBase
import java.io.File
import java.nio.file.Files
import java.util.Properties

class LogbackConfigurator: ContextAwareBase(), Configurator {
    override fun configure(loggerContext: LoggerContext): Configurator.ExecutionStatus {
        context = loggerContext

        // Find properties file in `org.noelware.charted.logback.config` system property
        val systemProp = System.getProperty(SYSTEM_PROPERTY, "")
        val envVariable = System.getenv(ENVIRONMENT_VARIABLE)

        // priority: environment variable > system property > classpath > default config
        if (envVariable != null) {
            addInfo("Loading from environment variable [$ENVIRONMENT_VARIABLE] which resolved to [$envVariable]")

            var file = File(envVariable)
            if (!file.exists()) throw IllegalArgumentException("Path [$envVariable] doesn't exist.")
            if (!file.isFile) throw IllegalArgumentException("Path [$envVariable] was not a file.")

            if (Files.isSymbolicLink(file.toPath())) {
                val resolved = Files.readSymbolicLink(file.toPath())
                addInfo("File [$file] is a symbolic link that resolves towards [$resolved]")

                file = resolved.toFile()
            }

            val properties = file.inputStream().use { Properties().apply { load(it) } }
            for ((key, value) in properties.entries) {
                loggerContext.putProperty(key as String, value.toString())
            }

            return processJoran0(loggerContext)
        }

        if (systemProp.isNotEmpty() || systemProp.isNotBlank()) {
            addInfo("Loading from system property [$SYSTEM_PROPERTY] which resolves to [$systemProp]")

            var file = File(systemProp)
            if (!file.exists()) throw IllegalArgumentException("Path [$systemProp] doesn't exist.")
            if (!file.isFile) throw IllegalArgumentException("Path [$systemProp] was not a file.")

            if (Files.isSymbolicLink(file.toPath())) {
                val resolved = Files.readSymbolicLink(file.toPath())
                addInfo("File [$file] is a symbolic link that resolves towards [$resolved]")

                file = resolved.toFile()
            }

            val properties = file.inputStream().use { Properties().apply { load(it) } }
            for ((key, value) in properties.entries) {
                loggerContext.putProperty(key as String, value.toString())
            }

            return processJoran0(loggerContext)
        }

        val resourcePath = this.javaClass.getResource("/config/logback.properties")
        if (resourcePath != null) {
            addInfo("Loading from resource path [config/logback.properties]")

            val inputStream = this.javaClass.getResourceAsStream("/config/logback.properties")
            val properties = inputStream.use { Properties().apply { load(it) } }
            for ((key, value) in properties.entries) {
                loggerContext.putProperty(key as String, value.toString())
            }

            return processJoran0(loggerContext)
        }

        addInfo("Falling back to safe configuration defaults!")

        // Represents the default configuration if the configurator can't:
        // - Load it from system property
        // - Load it from classpath
        // - Load it from the environment variable
        //
        // The log level will be set to INFO, which will be minimal output
        // instead of the verbose DEBUG/TRACE levels.
        //
        // JSON is set to be the default output, so it'll be easier to used
        // with other services that can check up on the logging system.
        loggerContext.putProperty("charted.log.level", "info")
        loggerContext.putProperty("charted.console.json", "true")
        loggerContext.putProperty("charted.encoders", "") // set it to empty so no appenders except Console is used.

        return processJoran0(loggerContext)
    }

    private fun processJoran0(context: LoggerContext): Configurator.ExecutionStatus {
        try {
            val joran = JoranConfigurator()
            joran.context = context
            joran.doConfigure(this.javaClass.getResource("/config/logback.xml"))

            return Configurator.ExecutionStatus.DO_NOT_INVOKE_NEXT_IF_ANY
        } catch (e: Exception) {
            throw e
        }
    }

    companion object {
        private const val SYSTEM_PROPERTY = "org.noelware.charted.logback.config"
        private const val ENVIRONMENT_VARIABLE = "CHARTED_LOGBACK_CONFIG_PATH"
    }
}
