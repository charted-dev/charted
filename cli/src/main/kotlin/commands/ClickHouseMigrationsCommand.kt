/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

@file:Suppress("PrivatePropertyName")

package org.noelware.charted.cli.commands

import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.parameters.options.*
import com.github.ajalt.mordant.terminal.Terminal
import dev.floofy.utils.kotlin.every
import dev.floofy.utils.kotlin.ifNotNull
import okhttp3.OkHttpClient
import okhttp3.Request
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.ChartedInfo
import org.noelware.charted.cli.logger
import org.noelware.charted.common.Architecture
import org.noelware.charted.common.OperatingSystem
import org.noelware.charted.extensions.doFormatTime
import java.io.File
import kotlin.system.exitProcess

private val OPERATING_SYSTEM = OperatingSystem.current()
private val ARCH = Architecture.current()

class ClickHouseMigrationsCommand(private val terminal: Terminal): CliktCommand(
    """
This command will run all the necessary ClickHouse migrations that need to be performed on the ClickHouse
cluster.

If the migrations binary is not available in the path, it will attempt to install it from Noelware's
Artifacts Repository:
    https://artifacts.noelware.cloud/charted/server/${ChartedInfo.version}/ch-migrations-${OPERATING_SYSTEM.key()}-${ARCH.key()}${if (OPERATING_SYSTEM.isWindows) ".exe" else ""}

It will not install the new binary if `bin/ch-migrations` exists or if this is performed in development mode,
which will invoke Go itself.
    """.trimIndent(),

    name = "ch-migrations"
) {
    private val okhttp: OkHttpClient = OkHttpClient()
    private val tableName: String by option(
        "--table", "-t",
        help = "The table where migrations should live in [default: migrations]"
    ).default("migrations")

    private val hosts: List<String> by option(
        "--hosts",
        help = "list of ClickHouse nodes to connect to"
    ).multiple()

    private val timeout: String by option(
        "--timeout",
        help = "timeout from connecting to the ClickHouse nodes"
    ).default("15s")

    private val username: String? by option(
        "--username", "-u",
        help = "username for authentication when connecting"
    )

    private val password: String? by option(
        "--password", "-p",
        help = "password for authentication when connecting"
    )

    private val database: String by option(
        "--database", "-d",
        help = "database name"
    ).default("charted")

    override fun run() {
        // Check if `./bin/ch-migrations` exists
        val migrationsBin = File("./bin/ch-migrations")
        if (migrationsBin.exists()) {
            assert(!migrationsBin.isDirectory)

            val args = mutableListOf(
                migrationsBin.toPath().toRealPath().toString(),
                "--table=$tableName",
                "--timeout=$timeout",
                "--database=$database"
            )

            if (hosts.every { it.isNotBlank() }) {
                args.add("--hosts=${hosts.joinToString(",")}")
            } else {
                args.add("--hosts=localhost:9000")
            }

            username.ifNotNull { args.add("--username=$this") }
            password.ifNotNull { args.add("--password=$this") }

            val sw = StopWatch.createStarted()
            terminal.logger.info("$ ${args.joinToString(" ")}")

            val proc = ProcessBuilder(migrationsBin.toPath().toRealPath().toString())
            proc.redirectOutput(ProcessBuilder.Redirect.INHERIT)

            val process = proc.start()
            process.waitFor()

            sw.stop()
            terminal.logger.info("Took [${sw.doFormatTime()}] to run binary!")
            exitProcess(process.exitValue())
        }

        migrationsBin.parentFile.mkdirs()

        val url = "https://artifacts.noelware.cloud/charted/server/${ChartedInfo.version}/ch-migrations-${OPERATING_SYSTEM.key()}-${ARCH.key()}${if (OPERATING_SYSTEM.isWindows) ".exe" else ""}"
        terminal.logger.info("Installing ch-migrations binary from URL [$url]")

        val sw = StopWatch.createStarted()
        val req = Request.Builder().apply {
            url(url)
            header("User-Agent", "Noelware/charted-server (+https://github.com/charted-dev/charted; ${ChartedInfo.version}+${ChartedInfo.commitHash})")
            method("GET", null)
        }.build()

        okhttp.newCall(req).execute().use { res ->
            val body: ByteArray = res.body!!.bytes()
            migrationsBin.writeBytes(body)
        }

        // set the binary executable
        migrationsBin.setExecutable(true)
        terminal.logger.info("Took ${sw.doFormatTime()} to download in [$migrationsBin] from URL [$url]")
        return run() // run again since it will exist.
    }
}
