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

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.haru.Scheduler
import dev.floofy.utils.koin.inject
import dev.floofy.utils.koin.injectOrNull
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import io.sentry.Sentry
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.cancel
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.modules.EmptySerializersModule
import net.perfectdreams.exposedpowerutils.sql.createOrUpdatePostgreSQLEnum
import okhttp3.internal.closeQuietly
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.Slf4jSqlDebugLogger
import org.jetbrains.exposed.sql.transactions.transaction
import org.koin.core.context.GlobalContext
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.analytics.AnalyticsServer
import org.noelware.charted.apikeys.ApiKeyManager
import org.noelware.charted.apikeys.DefaultApiKeyManager
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.data.helm.RepoType
import org.noelware.charted.common.extensions.formatToSize
import org.noelware.charted.configuration.ConfigurationHost
import org.noelware.charted.configuration.dsl.features.Feature
import org.noelware.charted.configuration.kotlin.KotlinScriptConfigurationHost
import org.noelware.charted.configuration.yaml.YamlConfigurationHost
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.core.interceptors.LogInterceptor
import org.noelware.charted.core.interceptors.SentryInterceptor
import org.noelware.charted.core.loggers.KoinLogger
import org.noelware.charted.core.loggers.SentryLogger
import org.noelware.charted.core.redis.DefaultRedisClient
import org.noelware.charted.database.PostgresStatCollector
import org.noelware.charted.database.cassandra.CassandraConnection
import org.noelware.charted.database.cassandra.CassandraMetricsCollector
import org.noelware.charted.database.cassandra.CassandraStatCollector
import org.noelware.charted.database.tables.*
import org.noelware.charted.elasticsearch.DefaultElasticsearchService
import org.noelware.charted.elasticsearch.ElasticsearchMetricsCollector
import org.noelware.charted.elasticsearch.ElasticsearchService
import org.noelware.charted.elasticsearch.ElasticsearchStatCollector
import org.noelware.charted.email.DefaultEmailService
import org.noelware.charted.email.EmailService
import org.noelware.charted.features.audits.auditLogsModule
import org.noelware.charted.features.webhooks.webhooksModule
import org.noelware.charted.invitations.DefaultInvitationManager
import org.noelware.charted.invitations.InvitationManager
import org.noelware.charted.metrics.PrometheusMetrics
import org.noelware.charted.search.meilisearch.MeilisearchClient
import org.noelware.charted.server.endpoints.endpointsModule
import org.noelware.charted.server.websockets.shutdownTickers
import org.noelware.charted.sessions.SessionManager
import org.noelware.charted.sessions.integrations.github.githubIntegration
import org.noelware.charted.sessions.local.LocalSessionManager
import org.noelware.charted.stats.StatisticsCollector
import org.noelware.charted.stats.collectors.RedisStatCollector
import java.io.File
import java.io.IOError
import java.lang.management.ManagementFactory
import java.nio.file.Files
import java.util.UUID
import kotlin.concurrent.thread
import kotlin.system.exitProcess
import kotlin.time.Duration.Companion.seconds

object Bootstrap {
    private val log by logging<Bootstrap>()
    private fun createUUID() {
        val file = File("./instance.uuid")
        if (!file.exists()) {
            file.writeBytes(UUID.randomUUID().toString().toByteArray())

            val root = File(".")
            log.warn("Instance UUID didn't exist in $root/instance.uuid! So I created it instance.")
            log.warn("If this was used with Noelware Analytics! Edit the instance via https://analytics.noelware.org/instances")
        }
    }

    private fun halt(code: Int) {
        Runtime.getRuntime().halt(code)
    }

    private fun installShutdownHook() {
        val runtime = Runtime.getRuntime()
        runtime.addShutdownHook(
            thread(start = false, name = "Server-ShutdownThread") {
                log.warn("Shutting down charted-server...")

                val koin = GlobalContext.getKoinApplicationOrNull()
                if (koin != null) {
                    val server: ChartedServer by inject()
                    val elasticsearch: ElasticsearchService? by injectOrNull()
                    val redis: IRedisClient by inject()
                    val ds: HikariDataSource by inject()
                    val sessions: SessionManager by inject()
                    val cassandra: CassandraConnection? by injectOrNull()

                    elasticsearch?.closeQuietly()
                    cassandra?.close()
                    sessions.closeQuietly()
                    server.closeQuietly()
                    ds.closeQuietly()
                    redis.closeQuietly()
                    koin.close()

                    runBlocking {
                        shutdownTickers()
                        ChartedScope.cancel()
                    }
                } else {
                    log.warn("Koin was not started, not destroying server (just yet!)")
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
                val started = ChartedServer.hasStarted.wasSet()
                if (!started && (thread.name == "Server-ShutdownThread" || thread.name == "Server-BootstrapThread")) {
                    halt(120)
                    exitProcess(1)
                }
            }
        }
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    @JvmStatic
    fun main(args: Array<String>) {
        Thread.currentThread().name = "Server-BootstrapThread"
        println("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+")
        println("+       _                _           _                                      +")
        println("+    ___| |__   __ _ _ __| |_ ___  __| |      ___  ___ _ ____   _____ _ __  +")
        println("+   / __| '_ \\ / _` | '__| __/ _ \\/ _` |_____/ __|/ _ \\ '__\\ \\ / / _ \\ '__| +")
        println("+  | (__| | | | (_| | |  | ||  __/ (_| |_____\\__ \\  __/ |   \\ V /  __/ |    +")
        println("+   \\___|_| |_|\\__,_|_|   \\__\\___|\\__,_|     |___/\\___|_|    \\_/ \\___|_|    +")
        println("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+")
        println("")

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

        val fullConfigPath = System.getenv("CHARTED_CONFIG_PATH") ?: "./config.yml"
        var configFile = File(fullConfigPath)
        if (Files.isSymbolicLink(configFile.toPath())) {
            val resolved = Files.readSymbolicLink(configFile.toPath())

            log.warn("Configuration file [$configFile] is a symbolic link towards [$resolved]")
            configFile = resolved.toFile()
        }

        val host: ConfigurationHost = if (listOf("yaml", "yml").contains(configFile.extension)) {
            YamlConfigurationHost(yaml)
        } else if (configFile.extension.contains("kts")) {
            KotlinScriptConfigurationHost
        } else {
            throw IllegalStateException("Unable to determine configuration host to use")
        }

        val config = host.loadConfig(configFile)
        log.info("Loaded configuration in path in [$configFile]")

        // Enable debug probes for Noelware Analytics and the administration
        // dashboard via Pak.
        DebugProbes.enableCreationStackTraces = config.debug
        DebugProbes.install()

        val ds = HikariDataSource(
            HikariConfig().apply {
                // transactionIsolation = IsolationLevel.TRANSACTION_REPEATABLE_READ.name
                leakDetectionThreshold = 30.seconds.inWholeMilliseconds
                driverClassName = "org.postgresql.Driver"
                isAutoCommit = false
                poolName = "Charted-HikariPool"
                username = config.postgres.username
                password = config.postgres.password
                jdbcUrl = "jdbc:postgresql://${config.postgres.host}:${config.postgres.port}/${config.postgres.name}"
                schema = config.postgres.schema

                addDataSourceProperty("reWriteBatchedInserts", "true")
            }
        )

        val isDebugFromProperties = System.getProperty("org.noelware.charted.debug", "false") == "true"
        Database.connect(
            ds,
            databaseConfig = DatabaseConfig {
                defaultRepetitionAttempts = 5
                // defaultIsolationLevel = IsolationLevel.TRANSACTION_REPEATABLE_READ.levelId
                sqlLogger = if (config.debug || isDebugFromProperties) {
                    Slf4jSqlDebugLogger
                } else {
                    null
                }
            }
        )

        transaction {
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

        log.info("Connected to PostgreSQL! Creating storage provider...")
        val wrapper = StorageWrapper(config.storage)
        val json = Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }

        log.info("Created storage provider! Connecting to Redis...")
        val redis = DefaultRedisClient(config.redis)
        redis.connect()

        log.info("Connected to Redis!")
        if (config.sentryDsn != null) {
            log.info("Enabling Sentry due to [sentryDsn] was set.")
            Sentry.init {
                it.setLogger(SentryLogger)

                it.release = "charted-server v${ChartedInfo.version} (${ChartedInfo.commitHash})"
                it.dsn = config.sentryDsn
            }

            Sentry.configureScope {
                it.tags += mapOf(
                    "charted.distribution.type" to ChartedInfo.distribution.key,
                    "charted.build.date" to ChartedInfo.buildDate,
                    "charted.version" to ChartedInfo.version,
                    "charted.commit" to ChartedInfo.commitHash,
                    "system.user" to System.getProperty("user.name")
                )
            }

            log.info("Sentry is now enabled!")
        }

        val scheduler = Scheduler {
            handleError { job, t ->
                val logger by logging<Scheduler>()
                logger.error("Unable to execute job [${job.name}]:", t)
            }
        }

        val sessions = LocalSessionManager(config, redis, json)
        val apiKeysManager = DefaultApiKeyManager(redis)
        val email = if (config.email != null) DefaultEmailService(config.email!!) else null
        val invitations = DefaultInvitationManager(email, config, redis, json)

        val httpClient = HttpClient(OkHttp) {
            engine {
                addInterceptor(LogInterceptor)
                if (Sentry.isEnabled()) {
                    addInterceptor(SentryInterceptor)
                }
            }

            install(ContentNegotiation) {
                this.json(json)
            }

            install(UserAgent) {
                agent = "Noelware/charted-server (v${ChartedInfo.version}; https://github.com/charted-dev/charted)"
            }
        }

        val elasticsearch = if (config.search.elastic != null) DefaultElasticsearchService(config, json) else null
        val meilisearch = if (config.search.meili != null) MeilisearchClient(httpClient, config.search.meili!!) else null
        val cassandra = if (config.cassandra != null) CassandraConnection(config.cassandra!!) else null
        val stats = StatisticsCollector().apply {
            register("postgres", PostgresStatCollector(config))
            register("redis", RedisStatCollector(redis))

            if (cassandra != null) {
                register("cassandra", CassandraStatCollector(cassandra))
            }

            if (elasticsearch != null) {
                register("elasticsearch", ElasticsearchStatCollector(elasticsearch))
            }
        }

        val analytics = if (config.analytics != null) AnalyticsServer(config.analytics!!) else null
        val server = ChartedServer(config, analytics)
        val metrics = PrometheusMetrics(ds, redis)
        val module = module {
            single<InvitationManager> { invitations }
            single<SessionManager> { sessions }
            single<ApiKeyManager> { apiKeysManager }
            single<IRedisClient> { redis }
            single { httpClient }
            single { scheduler }
            single { wrapper }
            single { config }
            single { server }
            single { stats }
            single { yaml }
            single { json }
            single { ds }

            if (email != null) {
                single<EmailService> { email }
            }

            if (elasticsearch != null) {
                single<ElasticsearchService> { elasticsearch }
            }

            if (meilisearch != null) {
                single { meilisearch }
            }

            if (cassandra != null) {
                single { cassandra }
            }

            if (config.metrics) {
                single { metrics }
            }

            if (analytics != null) {
                single { analytics }
            }
        }

        val modules = mutableListOf(chartedModule, *endpointsModule.toTypedArray(), module)
        if (config.isFeatureEnabled(Feature.AUDIT_LOGS)) {
            log.info("Audit Logs feature is enabled!")

            if (config.cassandra == null) {
                log.error("You will need to configure Cassandra before enabling audit logs!")
                exitProcess(1)
            }

            modules.add(auditLogsModule)
        }

        if (config.isFeatureEnabled(Feature.WEBHOOKS)) {
            log.info("Webhooks feature is enabled!")

            if (config.cassandra == null) {
                log.error("You will need to configure Cassandra before enabling webhooks!")
                exitProcess(1)
            }

            modules.add(webhooksModule)
        }

        if (config.sessions.integrations.github != null) {
            log.info("GitHub integration is enabled!")
            modules.add(githubIntegration)
        }

        startKoin {
            logger(KoinLogger(config))
            modules(*modules.toTypedArray())
        }

        runBlocking {
            cassandra?.connect()
            elasticsearch?.connect()
        }

        if (config.metrics && cassandra != null) {
            metrics.addCollector(CassandraMetricsCollector(cassandra))
        }

        if (config.metrics && elasticsearch != null) {
            metrics.addCollector(ElasticsearchMetricsCollector(elasticsearch))
        }

        try {
            server.start()
        } catch (e: Exception) {
            log.error("Unable to bootstrap charted-server:", e)

            // we do not let the shutdown hooks run
            // since in some cases, it'll just error out or whatever
            //
            // example: Elasticsearch cannot index all data due to
            // I/O locks or what not (and it'll keep looping)
            halt(1)
        }
    }
}
