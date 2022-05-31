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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.database.tests

import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import org.intellij.lang.annotations.Language
import org.junit.Test
import org.testcontainers.containers.PostgreSQLContainer
import java.sql.ResultSet
import kotlin.test.assertEquals

private val psql by lazy {
    val container = PostgreSQLContainer("postgres").apply {
        withUsername("charted")
        withPassword("charted")
        withDatabaseName("charted_server")
    }

    container.start()
    container
}

class PostgreSQLTests {
    private fun datastore(): HikariDataSource {
        val config = HikariConfig().apply {
            jdbcUrl = psql.jdbcUrl
            username = psql.username
            password = psql.password
            driverClassName = "org.postgresql.Driver"

            addDataSourceProperty("createDatabaseIfNotExists", "true")
        }

        return HikariDataSource(config)
    }

    private fun sql(@Language("sql") sql: String): ResultSet {
        val ds = datastore()
        val stmt = ds.connection.createStatement()
        stmt.execute(sql)

        val set = stmt.resultSet
        set.next()

        return set
    }

    @Test
    fun `test if connection successful`() {
        val result = sql("SELECT 1;")
        assertEquals(result.getInt(1), 1)
    }
}
