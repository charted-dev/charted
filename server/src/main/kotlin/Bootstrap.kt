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

package org.noelware.charted.server

import co.elastic.apm.attach.ElasticApmAttacher
import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.koin.inject
import dev.floofy.utils.koin.injectOrNull
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import io.sentry.Sentry
import kotlinx.coroutines.*
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.serialization.json.Json
import kotlinx.serialization.modules.EmptySerializersModule
import okhttp3.internal.closeQuietly
import org.apache.commons.lang3.time.StopWatch
import org.apache.commons.validator.routines.EmailValidator
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.Slf4jSqlDebugLogger
import org.jetbrains.exposed.sql.transactions.transaction
import org.koin.core.context.GlobalContext
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.ChartedInfo
import org.noelware.charted.ChartedScope
import org.noelware.charted.configuration.host.ConfigurationHost
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
import org.noelware.charted.extensions.formatToSize
import org.noelware.charted.modules.avatars.avatarsModule
import org.noelware.charted.modules.elasticsearch.DefaultElasticsearchModule
import org.noelware.charted.modules.elasticsearch.ElasticsearchModule
import org.noelware.charted.modules.metrics.PrometheusMetrics
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.redis.metrics.RedisMetricsCollector
import org.noelware.charted.modules.redis.metrics.RedisStatCollector
import org.noelware.charted.modules.sessions.SessionManager
import org.noelware.charted.modules.sessions.local.LocalSessionManager
import org.noelware.charted.modules.storage.DefaultStorageHandler
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.endpoints.v1.endpointsModule
import org.noelware.charted.server.internal.DefaultChartedServer
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import java.io.File
import java.io.IOError
import java.lang.management.ManagementFactory
import java.net.InetAddress
import java.util.*
import kotlin.concurrent.thread
import kotlin.system.exitProcess
import kotlin.time.Duration.Companion.seconds

/**
 * Represents the server bootstrap, which... bootstraps and loads the server.
 */
object Bootstrap {
    private val log by logging<Bootstrap>()

    private fun createUUID() {
        val env = System.getenv("CHARTED_NO_ANALYTICS") ?: "true"
        if (env.matches("^(yes|true|si|si*|1)$".toRegex())) {
            val file = File("./instance.uuid")
            if (!file.exists()) {
                file.writeBytes(UUID.randomUUID().toString().toByteArray())

                val root = File(".").toPath().toRealPath()
                log.warn("Created instance UUID for Noelware Analytics in [$root/instance.uuid]")
                log.warn("If you do not wish to create this file to identify this product, you can use the `CHARTED_NO_ANALYTICS` environment variable to skip this step.")
                log.warn("If you do wish to use this instance UUID for Noelware Analytics, edit your instance to connect the instance UUID: https://analytics.noelware.org/instances")
            }
        }
    }

    private fun halt(code: Int) {
        Runtime.getRuntime().halt(code)
    }

    private fun installShutdownHook() {
        val runtime = Runtime.getRuntime()
        runtime.addShutdownHook(
            thread(start = false, name = "Server-ShutdownThread") {
                log.warn("Shutting down API server...")

                val koin = GlobalContext.getKoinApplicationOrNull()
                if (koin != null) {
                    val elasticsearch: ElasticsearchModule? by injectOrNull()
                    val clickhouse: ClickHouseConnection? by injectOrNull()
                    val hikari: HikariDataSource by inject()
                    val server: ChartedServer by inject()
                    val redis: RedisClient by inject()

                    elasticsearch?.closeQuietly()
                    clickhouse?.closeQuietly()
                    hikari.closeQuietly()
                    redis.closeQuietly()
                    server.closeQuietly()

                    runBlocking {
                        ChartedScope.cancel()
                    }

                    koin.close()
                } else {
                    log.warn("Koin was not started, not doing anything.")
                }

                log.warn("charted-server has completely shutdown, goodbye! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡")
            }
        )
    }

    // credit: https://github.com/elastic/logstash/blob/main/logstash-core/src/main/java/org/logstash/Logstash.java#L98-L133
    private fun installDefaultThreadExceptionHandler() {
        Thread.setDefaultUncaughtExceptionHandler { thread, ex ->
            if (ex is Error) {
                log.error("Uncaught fatal error in thread ${thread.name} (#${thread.id}):", ex)
                log.error("If this keeps occurring, please report it to Noelware: https://github.com/charted-dev/charted/issues")

                var hasHalted = false
                if (ex is InternalError) {
                    hasHalted = true
                    halt(128)
                }

                if (ex is OutOfMemoryError) {
                    hasHalted = true
                    halt(127)
                }

                if (ex is StackOverflowError) {
                    hasHalted = true
                    halt(126)
                }

                if (ex is UnknownError) {
                    hasHalted = true
                    halt(125)
                }

                if (ex is IOError) {
                    hasHalted = true
                    halt(124)
                }

                if (ex is LinkageError) {
                    hasHalted = true
                    halt(123)
                }

                if (!hasHalted) halt(120)
                exitProcess(1)
            } else {
                log.error("Uncaught exception in thread ${thread.name} (#${thread.id}):", ex)

                // If any thread had an exception, let's check if:
                //  - The server has started (must be set if the Application hook has run)
                //  - If the thread names are the bootstrap or shutdown thread
                val started = hasStarted.get()
                if (!started && (thread.name == "Server-ShutdownThread" || thread.name == "Server-BootstrapThread")) {
                    halt(120)
                }
            }
        }
    }

    /**
     * Bootstraps and starts the server.
     * @param configPath The configuration path
     */
    @OptIn(ExperimentalCoroutinesApi::class)
    suspend fun start(configPath: File) {
        Thread.currentThread().name = "Charted-BootstrapThread"
        installDefaultThreadExceptionHandler()
        installShutdownHook()
        createUUID()

        val runtime = Runtime.getRuntime()
        val os = ManagementFactory.getOperatingSystemMXBean()

        log.info("===> JVM vendor/version: ${System.getProperty("java.vendor", "Unknown")} [${System.getProperty("java.version")}]")
        log.info("===> Kotlin version: ${KotlinVersion.CURRENT}")
        log.info("===> charted-server version: ${ChartedInfo.version} [${ChartedInfo.commitHash}]")
        log.info("===> Heap size: total=${runtime.totalMemory().formatToSize()} free=${runtime.freeMemory().formatToSize()}")
        log.info("===> Operating System: ${os.name.lowercase()}/${os.arch} (${os.availableProcessors} processors)")
        if (ChartedInfo.dedicatedNode != null) {
            log.info("===> Dedicated Node: ${ChartedInfo.dedicatedNode}")
        }

        for (pool in ManagementFactory.getMemoryPoolMXBeans())
            log.info("===> ${pool.name} <${pool.type}> -> ${pool.peakUsage}")

        log.info("===> JVM Arguments: [${ManagementFactory.getRuntimeMXBean().inputArguments.joinToString(" ")}]")
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

        val config = configHost.load(realPath.toString()) ?: throw IllegalStateException("Unable to load configuration")
        log.info("Loaded configuration in path [$realPath]")

        DebugProbes.enableCreationStackTraces = config.debug
        DebugProbes.install()

        if (config.tracing != null && config.tracing!!.apm != null) {
            log.info("Loading Elastic APM for tracing!")

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
                    } catch (_: Exception) {
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
                "application_packages" to "org.noelware.charted,org.noelware.charted.server",
                "server_url" to tracing.serverUrl
            )

            if (tracing.apiKey != null) apmConfig["api_key"] = tracing.apiKey!!
            if (tracing.secretToken != null) apmConfig["secret_token"] = tracing.secretToken!!
            if (!nodeName.isNullOrBlank()) {
                apmConfig["service_node_name"] = nodeName
            }

            ElasticApmAttacher.attach(apmConfig)
        }

        sw.suspend()
        log.info("Initialized configuration in ${sw.doFormatTime()}, now connecting to PostgreSQL")
        sw.resume()

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

        sw.stop()
        log.info("Created Postgres connection in ${sw.doFormatTime()}, now running migrations!")

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

        log.info("Finished running migrations!")
        if (config.sentryDsn != null) {
            log.info("Enabling Sentry due to [config.sentryDsn] was set.")
            Sentry.init {
                it.release = "charted-server v${ChartedInfo.version}+${ChartedInfo.commitHash}"
                it.dsn = config.sentryDsn
            }

            log.info("Sentry is now enabled!")
        }

        val storage = DefaultStorageHandler(config.storage)
        storage.init()

        val json = Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }

        val httpClient = HttpClient(OkHttp) {
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

        val metrics = PrometheusMetrics(ds)
        val redis = DefaultRedisClient(config.redis)
        val argon2 = Argon2PasswordEncoder()
        val email = EmailValidator.getInstance(true, true)
        redis.connect()

        if (config.metrics.enabled) {
            metrics.addGenericCollector(RedisStatCollector(redis))
            metrics.addMetricCollector(RedisMetricsCollector(redis, config.metrics))

            metrics.addGenericCollector(PostgresStatsCollector)
            metrics.addMetricCollector(PostgresMetricsCollector(config))
        }

        val sessions: SessionManager = when (config.sessions.type) {
            is SessionType.Local -> LocalSessionManager(argon2, redis, json, config)
            else -> throw IllegalStateException("Session type [${config.sessions.type}] is unsupported")
        }

        val koinModule = module {
            single<StorageHandler> { storage }
            single<ChartedServer> { DefaultChartedServer(config) }
            single<RedisClient> { redis }
            single { httpClient }
            single { sessions }
            single { argon2 }
            single { config }
            single { email }
            single { yaml }
            single { json }
            single { ds }
        }

        val modules = mutableListOf(*endpointsModule.toTypedArray(), avatarsModule, koinModule)
        if (config.search != null && config.search!!.elasticsearch != null) {
            val elasticsearch = DefaultElasticsearchModule(config, json)
            elasticsearch.connect()
            modules.add(
                module {
                    single<ElasticsearchModule> { elasticsearch }
                }
            )
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

        startKoin {
            modules(*modules.toTypedArray())
        }

        try {
            val server: ChartedServer by inject()
            server.start()
        } catch (e: Exception) {
            log.error("Unable to bootstrap charted-server:", e)
            throw e
        }
    }
}
