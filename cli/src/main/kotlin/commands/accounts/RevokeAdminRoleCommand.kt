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
import com.github.ajalt.clikt.parameters.arguments.validate
import com.github.ajalt.mordant.terminal.Terminal
import kotlinx.datetime.toKotlinLocalDateTime
import org.jetbrains.exposed.sql.transactions.transaction
import org.jetbrains.exposed.sql.update
import org.noelware.charted.cli.logger
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.postgresql.tables.UserTable
import java.time.LocalDateTime
import kotlin.system.exitProcess

class RevokeAdminRoleCommand(private val terminal: Terminal): AccountsAwareCommand(
    "Revokes a user's admin privileges",
    name = "revoke-admin",
) {
    private val username: String by argument(
        "username",
        help = "Username",
    ).validate { name ->
        if (!name.matchesNameAndIdRegex()) {
            terminal.logger.fatal("Username [$name] is not a valid username")
        }
    }

    override fun execute() {
        val user = transaction {
            UserEntity.find { UserTable.username eq username }.firstOrNull()
        } ?: return terminal.logger.fatal("User [$username] doesn't exist in the database")

        if (!user.admin) {
            terminal.logger.warn("User ${if (user.name != null) "${user.name} (@${user.username})" else user.username} doesn't have admin permissions already!")
            exitProcess(0)
        }

        transaction {
            UserTable.update({ UserTable.id eq user.id }) {
                it[updatedAt] = LocalDateTime.now().toKotlinLocalDateTime()
                it[admin] = false
            }
        }

        terminal.logger.info("Revoke admin permissions for [${if (user.name != null) "${user.name} (@${user.username})" else user.username}]")
    }
}
