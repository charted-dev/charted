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

@file:JvmName("CliMainKt")

package org.noelware.charted.cli

import com.github.ajalt.clikt.completion.CompletionCommand
import com.github.ajalt.clikt.core.*
import com.github.ajalt.clikt.parameters.options.versionOption
import com.github.ajalt.mordant.rendering.TextColors.*
import com.github.ajalt.mordant.rendering.TextStyles.*
import com.github.ajalt.mordant.terminal.Terminal
import org.noelware.charted.ChartedInfo
import org.noelware.charted.cli.commands.GenerateConfigCommand
import org.noelware.charted.cli.commands.ServerCommand
import kotlin.reflect.jvm.jvmName
import kotlin.system.exitProcess

private class ChartedCli(private val terminal: Terminal): CliktCommand(
    help = "Command line runner for managing charted-server",
    name = "charted",
    printHelpOnEmptyArgs = true,
    allowMultipleSubcommands = true
) {
    init {
        versionOption("${ChartedInfo.version}+${ChartedInfo.commitHash}") {
            """
            |charted-server v$it (build date: ${ChartedInfo.buildDate})
            |>> Website: https://charts.noelware.org
            |>> GitHub:  https://github.com/charted-dev/charted
            """.trimMargin("|")
        }

        context {
            findOrSetObject {
                terminal
            }
        }

        subcommands(
            GenerateConfigCommand(terminal),
            ServerCommand(terminal),
            CompletionCommand(name = "completions")
        )
    }

    // It will print on help anyway, so we don't need to do anything fancy in here :D
    override fun run() {}
}

fun main(args: Array<String>) {
    val terminal = Terminal()
    try {
        val cli = ChartedCli(terminal)
        cli.main(args)
    } catch (e: Exception) {
        val urlColour = italic + gray
        terminal.println(
            """
        |Unable to execute the main command line runner. If this is a reoccurring issue,
        |please report it to the Noelware team:
        |
        |   ${urlColour("https://github.com/charted-dev/charted/issues/new")}
        |
        |${red(e::class.jvmName + ":")}${if (e.message != null) " " + e.message else ""}
        """.trimMargin("|")
        )

        for (element in e.stackTrace) {
            terminal.println("    * $element")
        }

        exitProcess(1)
    }
}
