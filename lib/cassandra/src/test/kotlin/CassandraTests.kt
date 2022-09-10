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

package org.noelware.charted.database.cassandra.tests

import kotlinx.coroutines.runBlocking
import okhttp3.internal.closeQuietly
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.api.assertThrows
import org.junit.jupiter.api.condition.DisabledOnOs
import org.junit.jupiter.api.condition.OS
import org.noelware.charted.common.extensions.toList
import org.noelware.charted.configuration.dsl.CassandraConfig
import org.noelware.charted.database.cassandra.CassandraConnection
import org.noelware.charted.database.cassandra.extensions.hasNext
import org.noelware.charted.database.cassandra.extensions.iterator
import org.testcontainers.containers.CassandraContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName
import kotlin.test.assertEquals
import kotlin.test.assertFalse

@DisabledOnOs(value = [OS.MAC, OS.WINDOWS])
@Testcontainers(disabledWithoutDocker = true)
class CassandraTests {
    @Container
    private val container: CassandraContainer<*> = CassandraContainer(DockerImageName.parse("cassandra").withTag("4.0"))
    private val config: CassandraConfig by lazy {
        CassandraConfig(nodes = listOf("${container.host}:${container.getMappedPort(9042)}"))
    }

    private suspend fun createConnection(block: suspend CassandraConnection.() -> Unit) {
        if (!container.isRunning) container.start()

        val connection = CassandraConnection(config)
        try {
            assertFalse(connection.closed)
            assertThrows<IllegalStateException> { connection.serverVersion }

            connection.connect()
            connection.block()
        } finally {
            connection.closeQuietly()
        }
    }

    @Test
    fun `if container started`() = runBlocking {
        createConnection { assertEquals("4.0.6", serverVersion) }
    }

    @Test
    fun `test keyspace creation`() = runBlocking {
        createConnection {
            assertDoesNotThrow {
                sql("CREATE KEYSPACE test WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};")
            }

            val keyspaces = sql("select * from system_schema.keyspaces;")
            if (!keyspaces.hasNext()) {
                throw IllegalStateException("Can't query keyspaces?")
            }

            assertEquals(1, keyspaces.iterator().toList().filter { it.getString("keyspace_name")?.startsWith("system") ?: false }.size)
        }
    }
}
