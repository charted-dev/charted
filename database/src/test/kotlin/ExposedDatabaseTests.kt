package org.noelware.charted.database.tests

import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.exposed.asyncTransaction
import kotlinx.coroutines.runBlocking
import net.perfectdreams.exposedpowerutils.sql.createOrUpdatePostgreSQLEnum
import okhttp3.internal.closeQuietly
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.Slf4jSqlDebugLogger
import org.junit.Test
import org.noelware.charted.common.data.helm.RepoType
import org.noelware.charted.database.tables.*
import org.testcontainers.containers.PostgreSQLContainer
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName
import kotlin.time.Duration.Companion.seconds

/**
 * Represents tests for connecting to PostgreSQL, for now.
 */
@Testcontainers(disabledWithoutDocker = true)
class ExposedDatabaseTests {
    private val container: PostgreSQLContainer<*> = PostgreSQLContainer(DockerImageName.parse("postgres").withTag("14.5"))
    private val hikariDs: HikariDataSource by lazy {
        HikariDataSource(HikariConfig().apply {
            leakDetectionThreshold = 30.seconds.inWholeMilliseconds
            driverClassName = "org.postgresql.Driver"
            poolName = "Charted-HikariPool"
            username = container.username
            password = container.password
            jdbcUrl = container.jdbcUrl

            addDataSourceProperty("reWriteBatchedInserts", "true")
        })
    }

    @Test
    fun `can we connect to psql`(): Unit = runBlocking {
        if (!container.isRunning) container.start()

        Database.connect(hikariDs, databaseConfig = DatabaseConfig {
            defaultRepetitionAttempts = 5
            sqlLogger = Slf4jSqlDebugLogger
        })

        // Run all migrations
        asyncTransaction {
            createOrUpdatePostgreSQLEnum(RepoType.values())
            createOrUpdatePostgreSQLEnum(WebhookEvent.values())

            SchemaUtils.createMissingTablesAndColumns(
                OrganizationTable,
                OrganizationMemberTable,
                RepositoryReleasesTable,
                RepositoryMemberTable,
                RepositoryTable,
                UserConnectionsTable,
                UserTable,
                ApiKeysTable,
                WebhookSettingsTable,
                User2faTable
            )
        }

        // close the datasource
        hikariDs.closeQuietly()
    }
}
