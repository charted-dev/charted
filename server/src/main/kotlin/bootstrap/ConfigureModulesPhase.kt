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

package org.noelware.charted.server.bootstrap

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.engine.java.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import io.sentry.Sentry
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import kotlinx.serialization.modules.EmptySerializersModule
import org.apache.commons.lang3.time.StopWatch
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.Slf4jSqlDebugLogger
import org.jetbrains.exposed.sql.transactions.transaction
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.ChartedInfo
import org.noelware.charted.configuration.host.ConfigurationHost
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.configuration.kotlin.host.KotlinScriptHost
import org.noelware.charted.configuration.yaml.YamlConfigurationHost
import org.noelware.charted.databases.clickhouse.ClickHouseConnection
import org.noelware.charted.databases.clickhouse.DefaultClickHouseConnection
import org.noelware.charted.databases.postgres.createOrUpdateEnums
import org.noelware.charted.databases.postgres.metrics.PostgreSQLMetricsCollector
import org.noelware.charted.databases.postgres.tables.*
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.modules.analytics.AnalyticsDaemon
import org.noelware.charted.modules.apikeys.ApiKeyManager
import org.noelware.charted.modules.apikeys.DefaultApiKeyManager
import org.noelware.charted.modules.avatars.avatarsModule
import org.noelware.charted.modules.docker.registry.authorization.DefaultRegistryAuthorizationPolicyManager
import org.noelware.charted.modules.docker.registry.authorization.RegistryAuthorizationPolicyManager
import org.noelware.charted.modules.elasticsearch.DefaultElasticsearchModule
import org.noelware.charted.modules.elasticsearch.ElasticsearchModule
import org.noelware.charted.modules.elasticsearch.metrics.ElasticsearchStats
import org.noelware.charted.modules.email.DefaultEmailService
import org.noelware.charted.modules.email.EmailService
import org.noelware.charted.modules.helm.charts.DefaultHelmChartModule
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.metrics.collectors.JvmProcessInfoMetrics
import org.noelware.charted.modules.metrics.collectors.JvmThreadsMetrics
import org.noelware.charted.modules.metrics.collectors.OperatingSystemMetrics
import org.noelware.charted.modules.metrics.disabled.DisabledMetricsSupport
import org.noelware.charted.modules.metrics.prometheus.PrometheusMetricsSupport
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.redis.metrics.RedisMetricsCollector
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.modules.sessions.local.LocalSessionManager
import org.noelware.charted.modules.storage.DefaultStorageHandler
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.endpoints.v1.endpointsModule
import org.noelware.charted.server.internal.DefaultChartedServer
import org.noelware.charted.server.internal.analytics.ChartedAnalyticsExtension
import org.noelware.charted.server.internal.metrics.ServerInfoMetricsCollector
import org.noelware.charted.server.internal.sideLoadOtelJavaAgent
import org.noelware.charted.server.logging.KoinLogger
import org.noelware.charted.snowflake.Snowflake
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import java.io.File
import kotlin.time.Duration.Companion.seconds

object ConfigureModulesPhase : BootstrapPhase() {
    private val log by logging<ConfigureModulesPhase>()

    @OptIn(ExperimentalCoroutinesApi::class)
    override suspend fun bootstrap(configPath: File) {
        val yaml = Yaml(
            EmptySerializersModule(),
            YamlConfiguration(
                encodeDefaults = true,
                strictMode = true,
            ),
        )

        val configHost: ConfigurationHost = if (listOf("yaml", "yml").contains(configPath.extension)) {
            YamlConfigurationHost(yaml)
        } else if (configPath.extension.contains("kts")) {
            KotlinScriptHost
        } else {
            throw IllegalStateException("Unable to determine which configuration host to use")
        }

        val sw = StopWatch.createStarted()
        val realPath = withContext(Dispatchers.IO) {
            configPath.toPath().toRealPath()
        }

        log.info("Loading configuration from path [$realPath]")
        val config = configHost.load(realPath.toString())
            ?: throw IllegalStateException("Unable to load configuration in path [$realPath]")

        sw.suspend()
        log.info("Loaded configuration in [${sw.doFormatTime()}], configuring PostgreSQL...")

        sw.resume()
        DebugProbes.enableCreationStackTraces = config.debug
        DebugProbes.install()

        if (config.tracing != null) {
            sideLoadOtelJavaAgent()
        }

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
                sqlLogger = if (config.debug || System.getProperty("org.noelware.charted.debug", "false") == "true") {
                    Slf4jSqlDebugLogger
                } else {
                    null
                }
            },
        )

        sw.suspend()
        log.info("Connected to PostgreSQL in [${sw.doFormatTime()}], running migrations...")
        sw.resume()

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

        sw.suspend()
        log.info("Ran all migrations in [${sw.doFormatTime()}]")

        if (config.sentryDsn != null) {
            log.info("Enabling Sentry due to [config.sentryDsn] was set")
            Sentry.init {
                it.release = "charted-server v${ChartedInfo.version}+${ChartedInfo.commitHash}"
                it.dsn = config.sentryDsn
            }

            log.info("Sentry is now enabled!")
        }

        val redis = DefaultRedisClient(config.redis)
        redis.connect()

        val storage = DefaultStorageHandler(config.storage)
        storage.init()

        val json = Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }

        // epoch is Dec 1st, 2022
        val snowflake = Snowflake(0, 1669791600000)
        val argon2 = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
        val apiKeyManager = DefaultApiKeyManager(redis)
        val sessions: SessionManager = when (config.sessions.type) {
            SessionType.Local -> LocalSessionManager(argon2, redis, json, config)
            else -> throw IllegalStateException("Session type [${config.sessions.type}] is unsupported")
        }

        val metrics = if (config.metrics.enabled) {
            PrometheusMetricsSupport(ds)
        } else {
            DisabledMetricsSupport()
        }

        metrics.add(RedisMetricsCollector(redis, config.metrics))
        metrics.add(PostgreSQLMetricsCollector(config))
        metrics.add(JvmThreadsMetrics.Collector())
        metrics.add(JvmProcessInfoMetrics.Collector())
        metrics.add(OperatingSystemMetrics.Collector())
        metrics.add(ServerInfoMetricsCollector)

        val koinModule = module {
            single<HelmChartModule> { DefaultHelmChartModule(storage, config, yaml) }
            single { EmailValidator.getInstance(true, true) }
            single<StorageHandler> { storage }
            single<ChartedServer> { DefaultChartedServer(config) }
            single<ApiKeyManager> { apiKeyManager }
            single<RedisClient> { redis }
            single { snowflake }
            single { sessions }
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
                if (config.metrics.enabled) {
                    val collector = ElasticsearchStats.Collector(elasticsearch, config)
                    metrics.add(collector)
                }

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

        if (config.smtp != null) {
            val emailService = DefaultEmailService(config.smtp)
            modules.add(
                module {
                    single<EmailService> { emailService }
                },
            )
        }

        if (config.analytics != null) {
            val daemon = AnalyticsDaemon(json, config.analytics!!, ChartedAnalyticsExtension(metrics))
            modules.add(
                module {
                    single { daemon }
                },
            )
        }

        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            val registryAuthorizationPolicyManager = DefaultRegistryAuthorizationPolicyManager(sessions, redis, json, config)
            modules.add(
                module {
                    single<RegistryAuthorizationPolicyManager> { registryAuthorizationPolicyManager }
                },
            )
        }

        modules.add(
            module {
                single {
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
