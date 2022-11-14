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

import com.github.ajalt.mordant.rendering.TextColors.Companion.rgb
import com.github.ajalt.mordant.rendering.TextStyles.*
import com.github.ajalt.mordant.terminal.Terminal
import java.text.SimpleDateFormat
import java.util.*
import kotlin.system.exitProcess

private val DATE_FORMATTER = SimpleDateFormat("MM/dd/yyyy '~' hh:mm:ss a", Locale.getDefault())

// rgb(241, 204, 209)
private val PINK = rgb("#F1CCD1")

// rgb(165, 204, 165)
private val INFO_COLOR = rgb("#A5CCA5")

// rgb(81, 81, 140)
private val DEBUG_COLOR = rgb("#51518C")

// rgb(166, 76, 76)
private val ERROR_COLOR = rgb("#A64C4C")

// rgb(233, 233, 130)
private val WARN_COLOR = rgb("#E9E982")

private fun getCurrentDate(): String = DATE_FORMATTER.format(Date())

object Logger {
    fun info(vararg messages: String) {
        println("${(INFO_COLOR + bold)("info")}    | ${(PINK + bold)(getCurrentDate())} ~ ${messages.joinToString(System.lineSeparator())}")
    }

    fun warn(vararg messages: String) {
        println("${(WARN_COLOR + bold)("warn")}    | ${(PINK + bold)(getCurrentDate())} ~ ${messages.joinToString(System.lineSeparator())}")
    }

    fun error(vararg messages: String) {
        println("${(ERROR_COLOR + bold)("error")}   | ${(PINK + bold)(getCurrentDate())} ~ ${messages.joinToString(System.lineSeparator())}")
    }

    fun fatal(vararg messages: String) {
        println("${(ERROR_COLOR + bold)("fatal")}   | ${(PINK + bold)(getCurrentDate())} ~ ${messages.joinToString(System.lineSeparator())}")
        exitProcess(1)
    }

    fun debug(vararg messages: String) {
        val debugEnv = System.getenv("CHARTED_DEBUG")
        if ((debugEnv ?: "") matches "^(yes|true|1|si|si*)$".toRegex()) {
            println("${(DEBUG_COLOR + bold)("debug")}   | ${(PINK + bold)(getCurrentDate())} ~ ${messages.joinToString(System.lineSeparator())}")
        }
    }
}

val Terminal.logger: Logger
    get() = Logger
