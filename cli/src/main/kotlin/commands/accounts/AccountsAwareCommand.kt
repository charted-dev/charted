/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.cli.commands.accounts

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.types.file
import com.github.ajalt.mordant.terminal.Terminal
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import kotlinx.serialization.modules.EmptySerializersModule
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.Slf4jSqlDebugLogger
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.cli.logger
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.configuration.kotlin.host.KotlinScriptHost
import org.noelware.charted.configuration.yaml.YamlConfigurationHost
import org.noelware.charted.databases.postgres.createOrUpdateEnums
import org.noelware.charted.databases.postgres.tables.*
import java.io.File
import kotlin.system.exitProcess
import kotlin.time.Duration.Companion.seconds

abstract class AccountsAwareCommand(private val terminal: Terminal, name: String, help: String = "") : CliktCommand(help, name = name) {
    private val configOption: File? by option(
        "--config", "-c",
        help = "The configuration path to use",
        envvar = "CHARTED_CONFIG_PATH",
    ).file(
        mustExist = true,
        canBeFile = true,
        canBeDir = false,
        mustBeWritable = false,
        mustBeReadable = true,
        canBeSymlink = true,
    )

    fun setup(runner: (config: Config) -> Unit = {}) {
        val configPath = if (configOption != null) {
            configOption!!
        } else {
            File("./config.yml")
        }

        terminal.logger.debug("Loading configuration from path [$configPath]")

        val config: Config = (
            if (listOf("yaml", "yml").contains(configPath.extension)) {
                YamlConfigurationHost(
                    Yaml(
                        EmptySerializersModule(),
                        YamlConfiguration(
                            encodeDefaults = true,
                            strictMode = true,
                        ),
                    ),
                )
            } else if (configPath.extension.contains("kts")) {
                KotlinScriptHost
            } else {
                throw IllegalStateException("Unable to determine which configuration host to use")
            }
            ).load(configPath.toPath().toString()) ?: return run {
            terminal.logger.error("Unable to initialize configuration in path [$configPath] :(")
        }

        if (config.sessions.type != SessionType.Local) {
            terminal.logger.warn("The accounts CLI manager is only allowed on local sessions, not running!")
            exitProcess(1)
        }

        terminal.logger.debug("Connecting to PostgreSQL...")
        val ds = HikariDataSource(
            HikariConfig().apply {
                leakDetectionThreshold = 30.seconds.inWholeMilliseconds
                driverClassName = "org.postgresql.Driver"
                isAutoCommit = false
                poolName = "Postgres-HikariPool"
                username = config.database.username
                password = config.database.password
                jdbcUrl = "jdbc:postgresql://${config.database.host}:${config.database.port}/${config.database.database}"
                schema = config.database.schema

                addDataSourceProperty("reWriteBatchedInserts", "true")
            },
        )

        Database.connect(
            ds,
            databaseConfig = DatabaseConfig {
                defaultRepetitionAttempts = 5
                sqlLogger = if (config.debug || System.getProperty("org.noelware.charted.debug", "false") == "true") {
                    Slf4jSqlDebugLogger
                } else {
                    null
                }
            },
        )

        terminal.logger.debug("Running all pending migrations!")
        transaction {
            createOrUpdateEnums()
            SchemaUtils.createMissingTablesAndColumns(
                ApiKeysTable,
                OrganizationTable,
                OrganizationMemberTable,
                RepositoryTable,
                RepositoryMemberTable,
                RepositoryReleasesTable,
                UserTable,
                UserConnectionsTable,
            )
        }

        return runner(config)
    }
}
