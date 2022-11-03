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

package org.noelware.charted.databases.clickhouse

import org.intellij.lang.annotations.Language
import java.io.Closeable
import java.sql.ResultSet

/**
 * Represents an abstraction layer over the ClickHouse server itself. You can execute
 * arbitrary queries with this interface.
 */
interface ClickHouseConnection: Closeable {
    /**
     * Returns the version of the server that it is running on. **charted-server** requires
     * the server version to be higher than v22.6
     */
    val serverVersion: String

    /**
     * Returns if the connection has been closed by the [#close()][close] method.
     */
    val closed: Boolean

    /**
     * Returns how many database calls were processed using the [#sql][sql] methods.
     */
    val calls: Long

    /**
     * Executes arbitrary SQL and returns the underlying [ResultSet] that was returned
     * from the server.
     *
     * @param sql Arbitrary SQL code to execute
     * @param args Any additional parameterized arguments for the SQL to be executable.
     */
    fun sql(
        @Language("sql") sql: String,
        vararg args: Any? = arrayOf()
    ): ResultSet?

    /**
     * Executes arbitrary SQL and returns [U] from the [FromResultSet] that was passed in. Read
     * the documentation for [FromResultSet] for more information.
     *
     * @param sql Arbitrary SQL code to execute
     * @param args Any additional parameterized arguments for the SQL to be executable.
     */
    fun <U: Any> sql(
        @Language("sql") sql: String,
        from: FromResultSet<U>,
        vararg args: Any? = arrayOf()
    ): U?

    /**
     * Connects to the server.
     */
    fun connect()
}
