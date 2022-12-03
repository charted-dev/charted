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

import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.parameters.options.*
import com.github.ajalt.mordant.terminal.Terminal
import dev.floofy.utils.kotlin.ifNotNull
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.ChartedInfo
import org.noelware.charted.cli.logger
import org.noelware.charted.common.Architecture
import org.noelware.charted.common.OperatingSystem
import org.noelware.charted.extensions.doFormatTime
import java.io.File
import kotlin.system.exitProcess

private val os = OperatingSystem.current()
private val arch = Architecture.current()

class ClickHouseMigrationsCommand(private val terminal: Terminal): CliktCommand(
    """
This command will run all the necessary ClickHouse migrations that need to be performed on the ClickHouse
cluster.

If the migrations binary is not available in the path, it will attempt to install it from Noelware's
Artifact Repository:
    https://artifacts.noelware.cloud/charted/server/${ChartedInfo.version}/ch-migrations-${os.key()}-${arch.key()}${if (OperatingSystem.current().isWindows) ".exe" else ""}

It will not install the new binary if `bin/ch-migrations` exists or if this is performed in development mode,
which will invoke Go itself.
    """.trimIndent(),

    name = "ch-migrations"
) {
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
                "--hosts=${hosts.joinToString(",")}",
                "--timeout=$timeout",
                "--database=$database"
            )

            username.ifNotNull { args.add("--username=$this") }
            password.ifNotNull { args.add("--password=$this") }

            val sw = StopWatch.createStarted()
            terminal.logger.info("$ ${args.joinToString(" ")}")

            val proc = ProcessBuilder(migrationsBin.toPath().toRealPath().toString())
            val process = proc.start()
            process.waitFor()

            sw.stop()
            terminal.logger.info("Took [${sw.doFormatTime()}] to complete all ClickHouse migrations!")
            exitProcess(process.exitValue())
        }

        println(migrationsBin.parentFile)
        migrationsBin.parentFile.mkdirs()

        val url = "https://artifacts.noelware.cloud/charted/server/${ChartedInfo.version}/ch-migrations-${os.key()}-${arch.key()}${if (os.isWindows) ".exe" else ""}"
        terminal.logger.info("Installing ch-migrations binary from URL [$url]")

        val sw = StopWatch.createStarted()
    }
}
