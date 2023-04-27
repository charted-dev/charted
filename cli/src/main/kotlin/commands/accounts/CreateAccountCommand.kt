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

package org.noelware.charted.cli.commands.accounts

import com.github.ajalt.clikt.parameters.arguments.argument
import com.github.ajalt.clikt.parameters.arguments.optional
import com.github.ajalt.clikt.parameters.options.flag
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.mordant.terminal.Terminal
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.toKotlinLocalDateTime
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.transactions.transaction
import org.jetbrains.exposed.sql.update
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.SNOWFLAKE_EPOCH
import org.noelware.charted.cli.commands.abstractions.resolveConfigHost
import org.noelware.charted.cli.ktor.NoOpApplicationCall
import org.noelware.charted.cli.logger
import org.noelware.charted.common.extensions.regexp.matchesPasswordRegex
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.postgresql.controllers.users.CreateUserPayload
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.snowflake.Snowflake
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import java.io.BufferedInputStream
import java.time.LocalDateTime
import java.util.NoSuchElementException
import java.util.Scanner
import kotlin.system.exitProcess

class CreateAccountCommand(private val terminal: Terminal): AccountsAwareCommand(
    "Creates an account in the database, regardless if registrations is enabled or if the server is in invite-only mode",
    name = "create",
) {
    private val username: String by argument(
        "username",
        help = "Username to identify the user",
    )

    private val email: String by argument(
        "email",
        help = "Email address to identify the user",
    )

    private val password: String? by argument(
        "password",
        help = "Password to the account",
    ).optional()

    private val verifiedPublisher: Boolean by option(
        "--verified-publisher", "--vp",
        help = "If the created user should be a verified publisher",
    ).flag(default = false)

    private val admin: Boolean by option(
        "--admin", "-a",
        help = "If the user should be an administrator of this instance or not",
    ).flag(default = false)

    private val stdin: Boolean by option(
        "--stdin", "-x",
        help = "If the password should be looked from the standard input",
    ).flag(default = false)

    override fun execute() {
        // so that CreateUserPayload can work
        startKoin {
            modules(
                module {
                    single { EmailValidator.getInstance(true, true) }
                },
            )
        }

        val argon2 = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
        val snowflake = Snowflake(0, SNOWFLAKE_EPOCH)
        val config = resolveConfigHost().load(resolveConfigFile())
        val controller = UserDatabaseController(argon2, config, snowflake)
        if (config.sessions.type != SessionType.Local) {
            terminal.logger.warn(
                """
            |Your configured session manager is [${config.sessions.type}], which means you will need
            |to create an account that can be resolved to the local account @$username.
            |
            |~> LDAP: Create a user in the specified group (from the `config.sessions.ldap.group_id` configuration
            |key) and it will automatically be queried and resolved on every login invocation.
            """.trimMargin("|"),
            )

            if (password != null) {
                terminal.logger.warn("Providing a password is optional and will not be resolved in the final local account creation.")
            }

            val userByUsername = runBlocking { controller.getOrNull(UserTable::username to username) }
            if (userByUsername != null) {
                terminal.logger.fatal("Username [$username] is already taken!")
            }

            val userByEmail = runBlocking { controller.getOrNull(UserTable::email to email) }
            if (userByEmail != null) {
                terminal.logger.fatal("Email [$email] is already taken!")
            }

            val id = runBlocking { snowflake.generate() }
            val user = transaction {
                UserEntity.new(id.value) {
                    createdAt = LocalDateTime.now().toKotlinLocalDateTime()
                    updatedAt = LocalDateTime.now().toKotlinLocalDateTime()
                    username = username
                    email = email
                }
            }.let { entity -> User.fromEntity(entity) }

            val abilities = listOfNotNull(
                if (user.admin) "Administrator" else null,
                if (user.verifiedPublisher) "Verified Publisher" else null,
            ).joinToString(", ")

            terminal.logger.info("Created user @$username with${if (abilities.isNotBlank()) " abilities [$abilities]" else " no abilities"}. (${user.id})")
            exitProcess(0) // force kill for koin
        }

        val user = runBlocking {
            controller.create(
                NoOpApplicationCall(),
                CreateUserPayload(
                    username,
                    collectPasswordInput(),
                    email,
                ),
            )
        }

        if (admin || verifiedPublisher) {
            transaction {
                UserTable.update({ UserTable.id eq user.id }) {
                    it[updatedAt] = LocalDateTime.now().toKotlinLocalDateTime()

                    if (this@CreateAccountCommand.admin) {
                        it[admin] = true
                    }

                    if (this@CreateAccountCommand.verifiedPublisher) {
                        it[verifiedPublisher] = verifiedPublisher
                    }
                }
            }
        }

        val abilities = listOfNotNull(
            if (user.admin) "Administrator" else null,
            if (user.verifiedPublisher) "Verified Publisher" else null,
        ).joinToString(", ")

        terminal.logger.info("Created user @$username with${if (abilities.isNotBlank()) " abilities [$abilities]" else " no abilities"}. (${user.id})")
        exitProcess(0) // force kill for koin
    }

    private fun collectPasswordInput(): String = when {
        password != null -> password!!
        stdin -> BufferedInputStream(System.`in`).use {
            val scanner = Scanner(it)
            try {
                scanner.nextLine().also { scanner.close() }
            } catch (e: NoSuchElementException) {
                terminal.logger.fatal("Unable to grab from standard input")
                "" // it will never reach here
            }
        }.trim()

        else -> terminal.prompt("Enter the password for the account: ", hideInput = true).also {
            terminal.println()
            if (it == null) {
                terminal.logger.fatal("Missing a password")
                return@also
            }

            if (!it.matchesPasswordRegex()) {
                terminal.logger.fatal("Password can only contain letters, digits, and special characters")
            }
        }!!
    }
}
