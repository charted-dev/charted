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

package org.noelware.charted.server.bootstrap

import co.elastic.apm.attach.ElasticApmAttacher
import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.grpc.protobuf.services.ProtoReflectionService
import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
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
import org.noelware.analytics.jvm.server.AnalyticsServerBuilder
import org.noelware.analytics.jvm.server.extensions.jvm.JvmMemoryPoolsExtension
import org.noelware.analytics.jvm.server.extensions.jvm.JvmThreadsExtension
import org.noelware.analytics.protobufs.v1.BuildFlavour
import org.noelware.charted.ChartedInfo
import org.noelware.charted.configuration.host.ConfigurationHost
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.configuration.kotlin.host.KotlinScriptHost
import org.noelware.charted.configuration.yaml.YamlConfigurationHost
import org.noelware.charted.databases.clickhouse.ClickHouseConnection
import org.noelware.charted.databases.clickhouse.DefaultClickHouseConnection
import org.noelware.charted.databases.postgres.createOrUpdateEnums
import org.noelware.charted.databases.postgres.metrics.PostgresMetricsCollector
import org.noelware.charted.databases.postgres.metrics.PostgresStatsCollector
import org.noelware.charted.databases.postgres.tables.*
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.modules.apikeys.ApiKeyManager
import org.noelware.charted.modules.apikeys.DefaultApiKeyManager
import org.noelware.charted.modules.avatars.avatarsModule
import org.noelware.charted.modules.elasticsearch.DefaultElasticsearchModule
import org.noelware.charted.modules.elasticsearch.ElasticsearchModule
import org.noelware.charted.modules.elasticsearch.metrics.ElasticsearchMetricCollector
import org.noelware.charted.modules.elasticsearch.metrics.ElasticsearchStats
import org.noelware.charted.modules.email.DefaultEmailService
import org.noelware.charted.modules.email.EmailService
import org.noelware.charted.modules.helm.charts.DefaultHelmChartModule
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.metrics.PrometheusMetrics
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.redis.metrics.RedisMetricsCollector
import org.noelware.charted.modules.redis.metrics.RedisStatCollector
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.modules.sessions.local.LocalSessionManager
import org.noelware.charted.modules.storage.DefaultStorageHandler
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.endpoints.v1.endpointsModule
import org.noelware.charted.server.internal.DefaultChartedServer
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import java.io.File
import java.net.InetAddress
import java.time.Instant
import kotlin.time.Duration.Companion.seconds

object ConfigureModulesPhase: BootstrapPhase() {
    private val log by logging<ConfigureModulesPhase>()

    @OptIn(ExperimentalCoroutinesApi::class)
    override suspend fun bootstrap(configPath: File) {
        val yaml = Yaml(
            EmptySerializersModule(),
            YamlConfiguration(
                encodeDefaults = true,
                strictMode = true
            )
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

        if (config.tracing != null && config.tracing!!.apm != null) {
            configureElasticAPMTracing(config)
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
            }
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
            }
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
                UserConnectionsTable
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

        val argon2 = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
        val apiKeyManager = DefaultApiKeyManager(redis)
        val sessions: SessionManager = when (config.sessions.type) {
            SessionType.Local -> LocalSessionManager(argon2, redis, json, config)
            else -> throw IllegalStateException("Session type [${config.sessions.type}] is unsupported")
        }

        val metrics = PrometheusMetrics(config.metrics.enabled, ds)
        metrics.addGenericCollector(RedisStatCollector(redis))
        metrics.addGenericCollector(PostgresStatsCollector)

        if (config.metrics.enabled) {
            metrics.addMetricCollector(RedisMetricsCollector(redis, config.metrics))
            metrics.addMetricCollector(PostgresMetricsCollector(config))
        }

        val koinModule = module {
            single<HelmChartModule> { DefaultHelmChartModule(storage, config, yaml) }
            single { EmailValidator.getInstance(true, true) }
            single<StorageHandler> { storage }
            single<ChartedServer> { DefaultChartedServer(config) }
            single<ApiKeyManager> { apiKeyManager }
            single<RedisClient> { redis }
            single { sessions }
            single { argon2 }
            single { config }
            single { yaml }
            single { json }
            single { ds }

            single {
                HttpClient(OkHttp) {
                    engine {
                        config {
                            followSslRedirects(true)
                            followRedirects(true)
                        }
                    }

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
                    val collector = ElasticsearchStats.Collector(elasticsearch)
                    metrics.addGenericCollector(collector)
                    metrics.addMetricCollector(ElasticsearchMetricCollector(collector))
                }

                modules.add(
                    module {
                        single<ElasticsearchModule> { elasticsearch }
                    }
                )
            }
        }

        if (config.clickhouse != null) {
            val clickhouse = DefaultClickHouseConnection(config.clickhouse!!)
            clickhouse.connect()

            modules.add(
                module {
                    single<ClickHouseConnection> { clickhouse }
                }
            )
        }

        if (config.smtp != null) {
            val emailService = DefaultEmailService(config.smtp)
            modules.add(
                module {
                    single<EmailService> { emailService }
                }
            )
        }

        if (config.analytics != null) {
            val server = AnalyticsServerBuilder(config.analytics!!.port).apply {
                withServiceToken(config.analytics!!.serviceToken)
                withExtension(JvmThreadsExtension())
                withExtension(JvmMemoryPoolsExtension())

                withServerBuilder { server ->
                    server.addService(ProtoReflectionService.newInstance())
                }

                withServerMetadata { metadata ->
                    metadata.setDistributionType(BuildFlavour.ENTERPRISE)
                    metadata.setBuildDate(Instant.parse(ChartedInfo.buildDate))
                    metadata.setProductName("charted-server")
                    metadata.setCommitHash(ChartedInfo.commitHash)
                    metadata.setVersion(ChartedInfo.version)
                    metadata.setVendor("Noelware")
                }
            }.build()

            modules.add(
                module {
                    single { server }
                }
            )
        }

        modules.add(
            module {
                single {
                    metrics
                }
            }
        )

        startKoin {
            modules(*modules.toTypedArray())
        }
    }

    private suspend fun configureElasticAPMTracing(config: Config) {
        val sw = StopWatch.createStarted()
        log.info("Configuring Elastic APM for tracing...")

        val nodeNameEnv = System.getenv("NODE_NAME")
        val podNameEnv = System.getenv("POD_NAME")
        val nodeName = when {
            config.tracing!!.apm!!.serviceNodeName != null -> config.tracing!!.apm!!.serviceNodeName
            ChartedInfo.dedicatedNode != null -> ChartedInfo.dedicatedNode
            nodeNameEnv != null && podNameEnv != null -> "$podNameEnv@$nodeNameEnv"
            else -> {
                log.warn("Getting node name from hostname!")
                try {
                    withContext(Dispatchers.IO) {
                        InetAddress.getLocalHost()
                    }?.hostAddress.ifNotNull { "${System.getProperty("user.name")}@$this" } ?: ""
                } catch (e: Exception) {
                    log.warn("Unable to get local address, defaulting to empty name...", e)
                    ""
                }
            }
        }

        val tracing = config.tracing!!.apm!!
        val apmConfig = mutableMapOf(
            "recording" to "${tracing.recording}",
            "instrument" to "${tracing.enableInstrumentation}",
            "service_name" to "charted_server",
            "service_version" to "${ChartedInfo.version}+${ChartedInfo.commitHash}",
            "transaction_sample_rate" to "${tracing.transactionSampleRate}",
            "transaction_max_spans" to "${tracing.transactionMaxSpans}",
            "capture_body" to if (tracing.captureBody) "ON" else "OFF",
            // "global_labels" to tracing.globalLabels.map { "${it.key}=${it.value}" }.joinToString(",")
            "application_packages" to "org.noelware.charted",
            "server_url" to tracing.serverUrl
        )

        if (tracing.apiKey != null) apmConfig["api_key"] = tracing.apiKey!!
        if (tracing.secretToken != null) apmConfig["secret_token"] = tracing.secretToken!!
        if (!nodeName.isNullOrBlank()) {
            apmConfig["service_node_name"] = nodeName
        }

        ElasticApmAttacher.attach(apmConfig)
        sw.suspend()

        log.info("Configured Elastic APM tracing in [${sw.doFormatTime()}]")
    }
}
