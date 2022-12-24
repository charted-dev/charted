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

import com.github.ajalt.mordant.terminal.Terminal
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.cli.logger
import org.noelware.charted.databases.postgres.entities.UserEntity

class ListAccountsCommand(private val terminal: Terminal): AccountsAwareCommand(
    terminal,
    "list",
    "Lists all the managed accounts by the database."
) {
    override fun run(): Unit = setup {
        // Get all accounts available
        val accounts = transaction {
            UserEntity.all().toList()
        }

        terminal.logger.info("Found ${accounts.size} accounts!")
        for (account in accounts) {
            println("${account.name ?: "@${account.username}"} [${account.id}] - ${account.description ?: "(no description)"}")
        }
    }
}
