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

import org.junit.Test
import org.noelware.charted.testing.containers.AbstractClickHouseContainerTest
import java.sql.DriverManager
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class ClickHouseTest: AbstractClickHouseContainerTest() {
    @Test
    fun `can we query from container via java sql`() {
        val container = getContainer()
        val connection = DriverManager.getConnection("jdbc:clickhouse://${container.host}:${container.getMappedPort(9000)}?client_name=charted-test")
        val stmt = connection.createStatement()
        val rs = stmt.executeQuery("SELECT version() AS version;")

        assertTrue(rs.next(), "Was unable to query [SELECT version() AS version;]")
        assertEquals("22.6.2.12", rs.getString("version"))
    }
}
