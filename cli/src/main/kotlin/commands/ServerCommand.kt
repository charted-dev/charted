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

package org.noelware.charted.cli.commands

import com.github.ajalt.mordant.rendering.TextAlign
import com.github.ajalt.mordant.rendering.TextColors.*
import com.github.ajalt.mordant.rendering.TextColors.Companion.rgb
import com.github.ajalt.mordant.rendering.TextStyles.*
import com.github.ajalt.mordant.terminal.Terminal
import joptsimple.OptionSet
import joptsimple.OptionSpec
import joptsimple.ValueConverter
import kotlinx.coroutines.runBlocking
import org.noelware.charted.cli.Command
import org.noelware.charted.server.Bootstrap
import java.io.File
import java.nio.file.Files

object ServerCommand: Command("server", "Bootstraps and starts the server") {
    private val config: OptionSpec<File> = parser
        .acceptsAll(listOf("config", "c"), "The configuration file path")
        .withOptionalArg()
        .ofType(File::class.java)
        .withValuesConvertedBy(object: ValueConverter<File> {
            override fun valueType(): Class<out File> = File::class.java
            override fun valuePattern(): String? = null
            override fun convert(value: String): File {
                val file = File(value)
                if (!file.exists()) throw IllegalStateException("Path [$file] doesn't exist!")

                return if (Files.isSymbolicLink(file.toPath())) {
                    Files.readSymbolicLink(file.toPath()).toFile()
                } else {
                    file
                }
            }
        })

    override fun execute(terminal: Terminal, options: OptionSet) {
        val bannerColour = bold + rgb("#d4abd8")

        terminal.println(gray("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+"), align = TextAlign.CENTER)
        terminal.println(gray("+       ${bannerColour("_")}                ${bannerColour("_")}           ${bannerColour("_")}                                      +"), align = TextAlign.CENTER)
        terminal.println(gray("+    ${bannerColour("___| |__   __ _ _ __| |_ ___  __| |      ___  ___ _ ____   _____ _ __")}  +"), align = TextAlign.CENTER)
        terminal.println(gray("+   ${bannerColour("/ __| '_ \\ / _` | '__| __/ _ \\/ _` |_____/ __|/ _ \\ '__\\ \\ / / _ \\ '__|")} +"), align = TextAlign.CENTER)
        terminal.println(gray("+  ${bannerColour("| (__| | | | (_| | |  | ||  __/ (_| |_____\\__ \\  __/ |   \\ V /  __/ |")}    +"), align = TextAlign.CENTER)
        terminal.println(gray("+   ${bannerColour("\\___|_| |_|\\__,_|_|   \\__\\___|\\__,_|     |___/\\___|_|    \\_/ \\___|_|")}    +"), align = TextAlign.CENTER)
        terminal.println(gray("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+"), align = TextAlign.CENTER)
        terminal.println("")

        val configPath = if (options.has(config)) {
            options.valueOf(config)
        } else {
            File("./config.yml")
        }

        // It will block the main thread anyway.
        try {
            runBlocking {
                Bootstrap.start(configPath)
            }
        } catch (e: Exception) {
            // Interrupt the thread, so it can tear down successfully without
            // being stuck
            Thread.currentThread().interrupt()
            throw e
        }
    }
}
