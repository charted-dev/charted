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

import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.core.subcommands
import com.github.ajalt.clikt.parameters.options.versionOption
import com.github.ajalt.mordant.terminal.Terminal
import org.noelware.charted.ChartedInfo
import org.noelware.charted.cli.commands.ServerCommand

private class ChartedCli(private val terminal: Terminal): CliktCommand(
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
            ServerCommand(terminal),
        )
    }

    // we will run the help command
    override fun run() {}
}

fun main(args: Array<String>) {
    Thread.currentThread().name = "Charted-CliThread"

    val terminal = Terminal()
    try {
        val cli = ChartedCli(terminal)
        cli.main(args)
    } catch (e: Throwable) {
        e.printStackTrace()
    }
}
