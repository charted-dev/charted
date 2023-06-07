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

package org.noelware.charted.cli.commands

import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.types.file
import com.github.ajalt.mordant.rendering.TextAlign
import com.github.ajalt.mordant.rendering.TextColors.*
import com.github.ajalt.mordant.rendering.TextColors.Companion.rgb
import com.github.ajalt.mordant.rendering.TextStyles
import com.github.ajalt.mordant.terminal.Terminal
import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.runBlocking
import org.noelware.charted.cli.commands.abstractions.ConfigAwareCliktCommand
import org.noelware.charted.server.Bootstrap
import java.io.File
import kotlin.system.exitProcess

class ServerCommand(private val terminal: Terminal): ConfigAwareCliktCommand(
    "Bootstrap and starts the server in the same process",
    name = "server",
) {
    private val log by logging<ServerCommand>()

    @Suppress("unused")
    private val logbackPath: File? by option(
        "--logback-config", "-lc",
        help = "Configuration file for customizing charted-server's logging system",
        envvar = "CHARTED_LOGBACK_CONFIG_PATH",
    ).file(
        mustExist = true,
        canBeFile = true,
        canBeDir = false,
        mustBeWritable = false,
        mustBeReadable = true,
        canBeSymlink = true,
    )

    override fun run() {
        val bannerColour = TextStyles.bold + rgb("#d4abd8")
        terminal.println(gray("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+"), align = TextAlign.CENTER)
        terminal.println(gray("+       ${bannerColour("_")}                ${bannerColour("_")}           ${bannerColour("_")}                                      +"), align = TextAlign.CENTER)
        terminal.println(gray("+    ${bannerColour("___| |__   __ _ _ __| |_ ___  __| |      ___  ___ _ ____   _____ _ __")}  +"), align = TextAlign.CENTER)
        terminal.println(gray("+   ${bannerColour("/ __| '_ \\ / _` | '__| __/ _ \\/ _` |_____/ __|/ _ \\ '__\\ \\ / / _ \\ '__|")} +"), align = TextAlign.CENTER)
        terminal.println(gray("+  ${bannerColour("| (__| | | | (_| | |  | ||  __/ (_| |_____\\__ \\  __/ |   \\ V /  __/ |")}    +"), align = TextAlign.CENTER)
        terminal.println(gray("+   ${bannerColour("\\___|_| |_|\\__,_|_|   \\__\\___|\\__,_|     |___/\\___|_|    \\_/ \\___|_|")}    +"), align = TextAlign.CENTER)
        terminal.println(gray("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+"), align = TextAlign.CENTER)
        terminal.println("")

        try {
            runBlocking {
                Bootstrap.start(resolveConfigFile())
            }
        } catch (e: Throwable) {
            log.error("Unable to bootstrap server", e)
            exitProcess(1)
        }
    }
}
