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

package org.noelware.charted.cli.commands.accounts

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.github.ajalt.clikt.parameters.arguments.argument
import com.github.ajalt.clikt.parameters.arguments.optional
import com.github.ajalt.clikt.parameters.options.flag
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.mordant.terminal.Terminal
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.modules.EmptySerializersModule
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.cli.logger
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.databases.postgres.entities.UserConnectionEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.modules.helm.charts.DefaultHelmChartModule
import org.noelware.charted.modules.storage.DefaultStorageHandler
import org.noelware.charted.snowflake.Snowflake
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder

class CreateAccountCommand(private val terminal: Terminal): AccountsAwareCommand(terminal, "create", "Creates an account in the database") {
    private val username: String by argument("username", "The user's username")
    private val email: String by argument("email", "The user's email")
    private val passwordArgument: String? by argument("password", "The user's password to use, you can use the `-x`/`--stdin` flags to do the same").optional()
    private val useStdin: Boolean by option("-x", "--stdin", help = "If the password should be read from the standard input if the password was not provided.").flag()

    override fun run(): Unit = setup { config ->
        val password = if (passwordArgument == null) {
            if (!useStdin) {
                terminal.prompt("Enter the user's password: ")
            } else {
                terminal.readLineOrNull(true)
            }
        } else {
            passwordArgument
        } ?: return@setup run {
            terminal.logger.error("Unable to read from the standard input, please use the `password` argument.")
        }

        terminal.logger.info("Creating user @$username with email $email!")

        val userByUsername = transaction {
            UserEntity.find { UserTable.username eq username }.firstOrNull()
        }

        if (userByUsername != null) {
            terminal.logger.fatal("User @$username already exists in the database")
        }

        val userByEmail = transaction {
            UserEntity.find { UserTable.email eq email }.firstOrNull()
        }

        if (userByEmail != null) {
            terminal.logger.fatal("Email [$email] is already taken")
        }

        val snowflake = Snowflake(0, 1669791600000)
        val argon2 = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
        val id = runBlocking { snowflake.generate() }

        transaction {
            UserEntity.new(id.value) {
                createdAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                this.username = this@CreateAccountCommand.username
                this.password = argon2.encode(password)
                this.email = this@CreateAccountCommand.email
            }

            UserConnectionEntity.new(id.value) {
                createdAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
            }
        }

        val storage = DefaultStorageHandler(config.storage)
        storage.init()

        if (!config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            val charts = DefaultHelmChartModule(
                storage, config,
                Yaml(
                    EmptySerializersModule(),
                    YamlConfiguration(
                        encodeDefaults = true,
                        strictMode = true
                    )
                )
            )

            terminal.logger.info("New registered user [@$username], creating index.yaml entry!")
            runBlocking { charts.createIndexYaml(id.value) }
        }

        terminal.logger.info("Done!")
    }
}
