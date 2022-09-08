package org.noelware.charted.database.cassandra.tests

import kotlinx.coroutines.runBlocking
import okhttp3.internal.closeQuietly
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.api.assertThrows
import org.noelware.charted.common.data.CassandraConfig
import org.noelware.charted.common.extensions.toList
import org.noelware.charted.database.cassandra.CassandraConnection
import org.noelware.charted.database.cassandra.extensions.hasNext
import org.noelware.charted.database.cassandra.extensions.iterator
import org.testcontainers.containers.CassandraContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName
import kotlin.test.assertEquals
import kotlin.test.assertFalse

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
            if (!keyspaces.hasNext())
                throw IllegalStateException("Can't query keyspaces?")

            assertEquals(1, keyspaces.iterator().toList().filter { it.getString("keyspace_name")?.startsWith("system") ?: false }.size)
        }
    }
}
