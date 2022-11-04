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

package org.noelware.charted.cli

import com.github.ajalt.mordant.terminal.Terminal
import joptsimple.OptionDescriptor
import joptsimple.OptionParser
import joptsimple.OptionSet
import joptsimple.OptionSpec
import org.noelware.charted.ChartedInfo

/**
 * Represents a CLI command that can be executed from the main command line.
 * @param name The name of the command, it must match the command name regex.
 * @param description The description of the command, which is displayed on the help command.
 */
abstract class Command(
    val name: String,
    private val description: String
) {
    val parser: OptionParser = OptionParser()
    protected val helpOption: OptionSpec<Void> = parser.acceptsAll(listOf("h", "help"), "Shows help").forHelp()
    private val _subcommands: MutableList<Command> = mutableListOf()

    val subcommands: List<Command>
        get() = _subcommands

    fun addSubcommand(subcommand: Command) {
        _subcommands.add(subcommand)
    }

    fun printHelp(terminal: Terminal) {
        if (name == "charted") {
            terminal.println("charted-server v${ChartedInfo.version}+${ChartedInfo.commitHash.trim()} - $description")
            terminal.println("USAGE ::")
            terminal.println("    charted [COMMAND] [...ARGS]")
            terminal.println()
            terminal.println("COMMANDS ::")

            val longestCommandName = subcommands.maxOf { it.name.length }
            for (subcmd in subcommands) {
                terminal.println("   charted ${subcmd.name.padEnd(longestCommandName, ' ')} ~ ${subcmd.description}")
            }

            terminal.println()
            terminal.println("GLOBAL FLAGS ::")
            terminal.println("--version, -v ~ Returns the current version of the command line runner")
            terminal.println("--help, -h    ~ Returns this help menu.")
        } else {
            terminal.println("charted $name :: $description")
            terminal.println()

            if (subcommands.isNotEmpty()) {
                terminal.println("SUBCOMMANDS ::")

                val longestCommandName = subcommands.maxOf { it.name.length }
                for (subcmd in subcommands) {
                    terminal.println("   charted $name ${subcmd.name.padEnd(longestCommandName, ' ')} ~ ${subcmd.description}")
                }

                terminal.println()
            }

            terminal.println("FLAGS ::")

            val flags = parser.recognizedOptions()
            val longestFlagName = flags.values.maxOf { (it as OptionDescriptor).options().joinToString(", ") { c -> if (c.length == 1) "-$c" else "--$c" }.length }
            for (option in flags.values.distinctBy { it -> it.options().joinToString(", ") { if (it.length == 1) "-$it" else "--$it" } }) {
                // Don't know why this pops up but ok
                if (option.options().contains("[arguments]")) continue

                val descriptor = option as OptionDescriptor
                terminal.println("${if (descriptor.isRequired) "* " else ""}${descriptor.options().joinToString(", ") { if (it.length == 1) "-$it" else "--$it" }.padEnd(longestFlagName, ' ')} :: ${descriptor.description()} ${if (descriptor.argumentTypeIndicator().isEmpty()) "" else "<typeof ${descriptor.argumentTypeIndicator()}>"}")
            }
        }
    }

    abstract fun execute(terminal: Terminal, options: OptionSet)
}
