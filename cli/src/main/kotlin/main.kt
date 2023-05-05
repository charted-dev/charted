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

@file:JvmName("CliMainKt")

package org.noelware.charted.cli

import ch.qos.logback.classic.Level
import com.github.ajalt.clikt.completion.CompletionCommand
import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.core.subcommands
import com.github.ajalt.clikt.parameters.options.versionOption
import com.github.ajalt.mordant.terminal.ExperimentalTerminalApi
import com.github.ajalt.mordant.terminal.Terminal
import org.noelware.charted.ChartedInfo
import org.noelware.charted.cli.commands.GenerateConfigCommand
import org.noelware.charted.cli.commands.ServerCommand
import org.noelware.charted.cli.commands.ValidateKotlinScriptCommand
import org.noelware.charted.cli.commands.accounts.AccountsCommand
import org.slf4j.Logger
import org.slf4j.LoggerFactory

private class ChartedCli(terminal: Terminal): CliktCommand(
    "Command line runner for managing charted-server",
    name = "charted",
    printHelpOnEmptyArgs = true,
    allowMultipleSubcommands = true,
) {
    init {
        versionOption("${ChartedInfo.version}+${ChartedInfo.commitHash}", names = setOf("-v", "--version")) {
            """
            |charted-server v$it (build date: ${ChartedInfo.buildDate})
            |>> Website: https://charts.noelware.org
            |>> GitHub:  https://github.com/charted-dev/charted
            """.trimMargin("|")
        }

        subcommands(
            CompletionCommand(name = "completions"),
            ValidateKotlinScriptCommand(terminal),
            GenerateConfigCommand(terminal),
            AccountsCommand(terminal),
            ServerCommand(terminal),
        )
    }

    // we will run the help command
    override fun run() {}
}

@OptIn(ExperimentalTerminalApi::class)
fun main(args: Array<String>) {
    Thread.currentThread().name = "Charted-CliThread"

    val terminal = Terminal()
    try {
        if (args.isNotEmpty() && args.first() != "server") {
            // Remove `logback.*` from system properties,
            // so we don't get Logback information
            for (name in System.getProperties().keys) {
                if (name is String && name.startsWith("logback")) {
                    System.getProperties().remove(name)
                }
            }

            // Disable Logback from being used in non-server commands
            val log = LoggerFactory.getLogger(Logger.ROOT_LOGGER_NAME) as? ch.qos.logback.classic.Logger
            log?.level = Level.OFF
        }

        val cli = ChartedCli(terminal)
        cli.main(args)
    } catch (e: Throwable) {
        e.printStackTrace()
    }
}
