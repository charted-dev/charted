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

package org.noelware.charted.database.clickhouse.tests

import com.clickhouse.jdbc.JdbcConfig
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import org.intellij.lang.annotations.Language
import org.junit.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.noelware.charted.testing.containers.AbstractClickHouseContainerTest
import java.sql.ResultSet

class ClickHouseTest: AbstractClickHouseContainerTest() {
    private val dataSource: HikariDataSource
        get() = HikariDataSource(
            HikariConfig().apply {
                jdbcUrl = getContainer().jdbcUrl
                driverClassName = "com.clickhouse.jdbc.ClickHouseDriver"

                addDataSourceProperty(JdbcConfig.PROP_WRAPPER_OBJ, "true")
            }
        )

    private fun sql(@Language("sql") sql: String): ResultSet? {
        if (!getContainerState().wasSet()) {
            throw IllegalStateException("#startContainer() was never called, can't query SQL [$sql]")
        }

        val stmt = dataSource.connection.createStatement()
        stmt.execute(sql)

        val set = stmt.resultSet
        if (!set.next()) return null

        return set
    }

    @Test
    fun `can we query from container`() {
        assertDoesNotThrow { sql("SELECT 1;") }
    }
}
