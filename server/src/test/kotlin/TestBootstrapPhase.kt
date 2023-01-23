package org.noelware.charted.server.testing

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.engine.java.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.serialization.json.Json
import kotlinx.serialization.modules.EmptySerializersModule
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.transactions.transaction
import org.koin.core.context.GlobalContext.startKoin
import org.koin.dsl.module
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.databases.clickhouse.ClickHouseConnection
import org.noelware.charted.databases.clickhouse.DefaultClickHouseConnection
import org.noelware.charted.databases.postgres.createOrUpdateEnums
import org.noelware.charted.databases.postgres.tables.*
import org.noelware.charted.modules.apikeys.ApiKeyManager
import org.noelware.charted.modules.apikeys.DefaultApiKeyManager
import org.noelware.charted.modules.avatars.avatarsModule
import org.noelware.charted.modules.docker.registry.authorization.DefaultRegistryAuthorizationPolicyManager
import org.noelware.charted.modules.docker.registry.authorization.RegistryAuthorizationPolicyManager
import org.noelware.charted.modules.elasticsearch.DefaultElasticsearchModule
import org.noelware.charted.modules.elasticsearch.ElasticsearchModule
import org.noelware.charted.modules.helm.charts.DefaultHelmChartModule
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.disabled.DisabledMetricsSupport
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.modules.sessions.local.LocalSessionManager
import org.noelware.charted.modules.storage.DefaultStorageHandler
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.endpoints.v1.endpointsModule
import org.noelware.charted.server.logging.KoinLogger
import org.noelware.charted.snowflake.Snowflake
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import kotlin.time.Duration.Companion.seconds

object TestBootstrapPhase {
    private val log by logging<TestBootstrapPhase>()

    @OptIn(ExperimentalCoroutinesApi::class)
    suspend fun bootstrap(config: Config) {
        log.info("Initializing bootstrapping phase for tests...")

        // Configure debug probes for kotlinx.coroutines
        DebugProbes.enableCreationStackTraces = false
        DebugProbes.install()

        // Configure the DataSource for Postgres
        val ds = HikariDataSource(
            HikariConfig().apply {
                leakDetectionThreshold = 30.seconds.inWholeMilliseconds
                driverClassName = "org.postgresql.Driver"
                isAutoCommit = false
                poolName = "Postgres-HikariPool"
                username = config.database.username
                password = config.database.password
                jdbcUrl = "jdbc:postgresql://${config.database.host}:${config.database.port}/${config.database.database}"
                schema = config.database.schema

                addDataSourceProperty("reWriteBatchedInserts", "true")
            },
        )

        Database.connect(
            ds,
            databaseConfig = DatabaseConfig {
                defaultRepetitionAttempts = 5
            },
        )

        log.info("Connected to Postgres test container, running all migrations!")
        transaction {
            createOrUpdateEnums()
            SchemaUtils.createMissingTablesAndColumns(
                ApiKeysTable,
                OrganizationTable,
                OrganizationMemberTable,
                RepositoryTable,
                RepositoryMemberTable,
                RepositoryReleasesTable,
                UserTable,
                UserConnectionsTable,
            )
        }

        log.info("Ran all pending migrations! Connecting to Redis...")
        val redis = DefaultRedisClient(config.redis)
        redis.connect()

        log.info("Connected to Redis! Initializing storage driver...")
        val storage = DefaultStorageHandler(config.storage)
        storage.init()

        log.info("Initialized storage driver, now starting up Koin...")
        val json = Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }

        // epoch is Dec 1st, 2022
        val snowflake = Snowflake(0, 1669791600000)
        val argon2 = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
        val apiKeyManager = DefaultApiKeyManager(redis)
        val sessionManager = LocalSessionManager(argon2, redis, json, config) // use default manager for tests
        val metrics = DisabledMetricsSupport() // disable metrics from being collected
        val yaml = Yaml(
            EmptySerializersModule(),
            YamlConfiguration(
                encodeDefaults = true,
                strictMode = true,
            ),
        )

        val koinModule = module {
            single<HelmChartModule> { DefaultHelmChartModule(storage, config, yaml) }
            single { EmailValidator.getInstance(true, true) }
            single<SessionManager> { sessionManager }
            single<StorageHandler> { storage }
            single<ApiKeyManager> { apiKeyManager }
            single<RedisClient> { redis }
            single { sessionManager }
            single { snowflake }
            single { metrics }
            single { argon2 }
            single { config }
            single { yaml }
            single { json }
            single { ds }

            single {
                HttpClient(Java) {
                    install(ContentNegotiation) {
                        this.json(json)
                    }
                }
            }
        }

        val modules = mutableListOf(*endpointsModule.toTypedArray(), avatarsModule, koinModule)
        if (config.search != null) {
            if (config.search!!.elasticsearch != null) {
                val elasticsearch = DefaultElasticsearchModule(config, json)
                elasticsearch.connect()

                modules.add(
                    module {
                        single<ElasticsearchModule> { elasticsearch }
                    },
                )
            }
        }

        if (config.clickhouse != null) {
            val clickhouse = DefaultClickHouseConnection(config.clickhouse!!)
            clickhouse.connect()

            modules.add(
                module {
                    single<ClickHouseConnection> { clickhouse }
                },
            )
        }

        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            val registryAuthorizationPolicyManager = DefaultRegistryAuthorizationPolicyManager(sessionManager, redis, json, config)
            modules.add(
                module {
                    single<RegistryAuthorizationPolicyManager> { registryAuthorizationPolicyManager }
                },
            )
        }

        modules.add(
            module {
                single<MetricsSupport> {
                    metrics
                }
            },
        )

        startKoin {
            logger(KoinLogger)
            modules(*modules.toTypedArray())
        }
    }
}
