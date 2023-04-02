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

package org.noelware.charted.cli.commands.services

import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.parameters.arguments.argument
import com.github.ajalt.clikt.parameters.options.default
import com.github.ajalt.clikt.parameters.options.flag
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.mordant.terminal.Terminal
import org.noelware.charted.cli.commands.services.resolver.EmailServiceResolver
import org.noelware.charted.cli.logger
import kotlin.jvm.optionals.getOrDefault
import kotlin.jvm.optionals.getOrNull
import kotlin.system.exitProcess

private val services = mapOf<String, (info: ProcessHandle.Info) -> Boolean>(
    "emails" to { info -> info.command().getOrDefault("").contains("email-service") },
)

private fun findService(info: ProcessHandle.Info): Boolean {
    for ((name, isAvailable) in services) {
        if (isAvailable(info)) return true
    }

    return false
}

class StartServiceCommand(private val terminal: Terminal): CliktCommand(
    "Starts a microservice, if not already started",
    name = "start",
) {
    private val daemon: Boolean by option(
        "--daemon", "-d",
        help = "If the service process should run in the background",
    ).flag(default = true)

    private val service: String by argument(
        "service",
        help = "The service to launch",
    )

    override fun run() {
        if (!services.contains(service)) {
            terminal.logger.fatal("Unknown service to launch: $service")
        }

        val proc = ProcessHandle.allProcesses()
            .filter { findService(it.info()) }
            .limit(1)
            .findFirst()
            .getOrNull()

        if (proc != null) {
            terminal.logger.info("Service '$service' is already running (pid ${proc.pid()})")
            exitProcess(0)
        }

        val resolver = EmailServiceResolver(terminal)
        val binFile = resolver.resolve("0.1.0")

        val process = ProcessBuilder(binFile).apply {
            if (!daemon) {
                redirectOutput(ProcessBuilder.Redirect.PIPE)
                redirectError(ProcessBuilder.Redirect.PIPE)
            }
        }.start()

        if (daemon) {
            terminal.logger.info("Started process [$binFile] with PID ${process.pid()}")
        }
    }
}
