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

import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.core.subcommands
import com.github.ajalt.mordant.terminal.Terminal

class BaseAccountsCommand(terminal: Terminal) : CliktCommand(
    "CLI management for handling local users on the server. This doesn't apply to any other sessions that can be configured (i.e: LDAP)",
    name = "accounts",
    printHelpOnEmptyArgs = true,
    invokeWithoutSubcommand = true,
) {
    init {
        subcommands(
            CreateAccountCommand(terminal),
            ListAccountsCommand(terminal),
        )
    }

    override fun run() {
        /* do nothing */
    }
}
