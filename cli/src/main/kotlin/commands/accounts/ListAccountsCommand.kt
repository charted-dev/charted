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

import com.github.ajalt.clikt.parameters.options.default
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.types.int
import com.jakewharton.picnic.table
import com.github.ajalt.mordant.terminal.Terminal
import dev.floofy.utils.kotlin.ifNotNull
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toInstant
import kotlinx.datetime.toJavaInstant
import org.jetbrains.exposed.sql.transactions.transaction
import org.noelware.charted.modules.postgresql.entities.UserEntity
import java.text.SimpleDateFormat
import java.util.*
import kotlin.math.min

private fun <T> List<T>.paginate(
    page: Int,
    chunkSize: Int = 10
): List<T> {
    if (chunkSize <= 0 || page <= 0) {
        throw RuntimeException("invalid chunk size [$chunkSize] or page [$page]")
    }

    val from = (page - 1) * chunkSize
    if (size <= from) return this

    return subList(from, min(from + chunkSize, size))
}

class ListAccountsCommand(private val terminal: Terminal): AccountsAwareCommand(
    help = "Lists all the available accounts on this instance",
    name = "list",
) {
    private val dateTimeFormat = SimpleDateFormat("MMM d, yyyy 'at' k:mm:ss.SS zzz")
    private val page: Int by option(
        "--page", "-p",
        help = "Page to use for pagination",
    )
        .int()
        .default(1)

    override fun execute() {
        val accounts = transaction { UserEntity.all().toList() }
        val accountsToView = accounts.paginate(page)

        terminal.println(
            table {
                cellStyle {
                    paddingLeft = 1
                    paddingRight = 1
                    border = true
                }

                header {
                    row("Username", "ID", "Created At", "Last Updated At", "Abilities")
                }

                for (account in accountsToView) {
                    val abilities = listOfNotNull(
                        if (account.admin) "Administrator" else null,
                        if (account.verifiedPublisher) "Verified Publisher" else null,
                    ).joinToString(", ").ifBlank { "None" }

                    row(
                        account.name.ifNotNull { "$this (@${account.username}" } ?: "@${account.username}",
                        account.id,
                        dateTimeFormat.format(Date.from(account.createdAt.toInstant(TimeZone.currentSystemDefault()).toJavaInstant())),
                        dateTimeFormat.format(Date.from(account.updatedAt.toInstant(TimeZone.currentSystemDefault()).toJavaInstant())),
                        abilities,
                    )
                }
            },
        )
    }
}
