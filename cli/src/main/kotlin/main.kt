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

import com.github.ajalt.mordant.rendering.TextColors.*
import com.github.ajalt.mordant.rendering.TextStyles.*
import com.github.ajalt.mordant.terminal.Terminal
import dev.floofy.utils.kotlin.doFormatTime
import joptsimple.OptionSet
import joptsimple.OptionSpec
import kotlinx.datetime.Instant
import org.noelware.charted.ChartedInfo
import org.noelware.charted.cli.commands.ServerCommand
import kotlin.reflect.jvm.jvmName
import kotlin.system.exitProcess

private class ChartedCli: Command("charted", "Command line runner for managing charted-server") {
    init {
        addSubcommand(ServerCommand)
    }

    private val command: OptionSpec<String> = parser.nonOptions("command")
    override fun execute(terminal: Terminal, options: OptionSet) {
        if (!options.has(command)) {
            printHelp(terminal)
            exitProcess(0)
        }

        val subcommand = subcommands.find { it.name == options.valueOf(command) }
            ?: run {
                printHelp(terminal)
                exitProcess(0)
            }

        if (options.has(helpOption)) {
            subcommand.printHelp(terminal)
            exitProcess(0)
        }

        return subcommand.execute(terminal, options)
    }
}

fun main(args: Array<String>) {
    val terminal = Terminal()
    if (args.any { it.startsWith("--version") || it.startsWith("-v") }) {
        val since = Instant.parse(ChartedInfo.buildDate).nanosecondsOfSecond

        terminal.println("charted-server v${ChartedInfo.version}+${ChartedInfo.commitHash.trim()} built ${(System.nanoTime() - since).doFormatTime()} ago")
        terminal.println(">> Website: https://charts.noelware.org")
        terminal.println(">> GitHub:  https://github.com/charted-dev/charted")
        exitProcess(0)
    }

    val main = ChartedCli()
    try {
        val options = main.parser.parse(*args)
        main.execute(terminal, options)
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
