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
import kotlinx.coroutines.cancel
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.ExperimentalSerializationApi
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
import org.noelware.charted.common.ChartedInfo
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.IRedisClient
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.charted.common.data.helm.RepoType
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.core.apikeys.TokenExpirationManager
import org.noelware.charted.core.interceptors.LogInterceptor
import org.noelware.charted.core.interceptors.SentryInterceptor
import org.noelware.charted.core.loggers.KoinLogger
import org.noelware.charted.core.loggers.SentryLogger
import org.noelware.charted.core.ratelimiter.Ratelimiter
import org.noelware.charted.core.redis.DefaultRedisClient
import org.noelware.charted.core.sessions.SessionManager
import org.noelware.charted.database.clickhouse.ClickHouseConnection
import org.noelware.charted.database.tables.*
import org.noelware.charted.features.audits.AuditLogsFeature
import org.noelware.charted.search.elasticsearch.ElasticsearchClient
import org.noelware.charted.search.meilisearch.MeilisearchClient
import org.noelware.charted.server.endpoints.endpointsModule
import java.io.File
import java.io.IOError
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

                val koinStarted = GlobalContext.getKoinApplicationOrNull() != null
                if (koinStarted) {
                    val server: ChartedServer by inject()
                    val elasticsearch: ElasticsearchClient? by injectOrNull()
                    val redis: IRedisClient by inject()
                    val clickhouse: ClickHouseConnection? by injectOrNull()
                    val ds: HikariDataSource by inject()
                    val sessions: SessionManager by inject()
                    val ratelimiter: Ratelimiter by inject()

                    elasticsearch?.closeQuietly()
                    clickhouse?.closeQuietly()
                    sessions.closeQuietly()
                    server.destroy()
                    ds.closeQuietly()
                    ratelimiter.closeQuietly()
                    redis.closeQuietly()

                    runBlocking {
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

    @OptIn(ExperimentalSerializationApi::class)
    @JvmStatic
    fun main(args: Array<String>) {
        Thread.currentThread().name = "Server-BootstrapThread"
        println("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+")
        println("+       _                _           _                                      +")
        println("+   ___| |__   __ _ _ __| |_ ___  __| |      ___  ___ _ ____   _____ _ __   +")
        println("+   / __| '_ \\ / _` | '__| __/ _ \\/ _` |_____/ __|/ _ \\ '__\\ \\ / / _ \\ '__| +")
        println("+  | (__| | | | (_| | |  | ||  __/ (_| |_____\\__ \\  __/ |   \\ V /  __/ |    +")
        println("+   \\___|_| |_|\\__,_|_|   \\__\\___|\\__,_|     |___/\\___|_|    \\_/ \\___|_|    +")
        println("+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+~+")
        println("")

        installDefaultThreadExceptionHandler()
        installShutdownHook()
        createUUID()

        log.info("Loading configuration...")
        val fullConfigPath = System.getenv("CHARTED_CONFIG_PATH") ?: "./config.yml"
        var configFile = File(fullConfigPath)

        if (Files.isSymbolicLink(configFile.toPath())) {
            log.warn("File is under a symbolic link, will be loading from the target file rather than this file.")
            configFile = File(Files.readSymbolicLink(configFile.toPath()).toString())
        }

        if (!configFile.exists()) {
            log.error("Missing configuration file in path '$configFile'!")
            exitProcess(1)
        }

        if (!listOf("yml", "yaml").contains(configFile.extension)) {
            log.error("Configuration file at path $configFile must be a YAML file. (`.yml` or `.yaml` extensions)")
            exitProcess(1)
        }

        val yaml = Yaml(
            EmptySerializersModule,
            YamlConfiguration(
                encodeDefaults = true,
                strictMode = true
            )
        )

        val config = yaml.decodeFromString(Config.serializer(), configFile.readText())
        log.info("Retrieved configuration in path $configFile! Now connecting to PostgreSQL...")

        val ds = HikariDataSource(
            HikariConfig().apply {
                leakDetectionThreshold = 30.seconds.inWholeMilliseconds
                driverClassName = "org.postgresql.Driver"
                isAutoCommit = false
                poolName = "Charted-HikariPostgresPool"
                username = config.postgres.username
                password = config.postgres.password
                jdbcUrl = "jdbc:postgresql://${config.postgres.host}:${config.postgres.port}/${config.postgres.name}"
                schema = config.postgres.schema
            }
        )

        val isDebugFromProperties = System.getProperty("org.noelware.charted.debug", "false") == "true"
        Database.connect(
            ds,
            databaseConfig = DatabaseConfig {
                defaultRepetitionAttempts = 5
                sqlLogger = if (config.debug || isDebugFromProperties) {
                    Slf4jSqlDebugLogger
                } else {
                    null
                }
            }
        )

        transaction {
            createOrUpdatePostgreSQLEnum(RepoType.values())
            SchemaUtils.createMissingTablesAndColumns(
                OrganizationTable,
                OrganizationMemberTable,
                RepositoryReleasesTable,
                RepositoryMemberTable,
                RepositoryTable,
                UserConnectionsTable,
                UserTable,
                ApiKeysTable
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

        val clickhouse = if (config.clickhouse != null) {
            log.info("ClickHouse is enabled via config, now connecting!")
            ClickHouseConnection(config.clickhouse!!)
        } else {
            null
        }

        val scheduler = Scheduler {
            handleError { job, t ->
                val logger by logging<Scheduler>()
                logger.error("Unable to execute job [${job.name}]:", t)
            }
        }

        val ratelimiter = Ratelimiter(json, redis, scheduler)
        val sessions = SessionManager(config, json, redis)
        val apiKeyExpiration = TokenExpirationManager(redis)
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

        val elastic: ElasticsearchClient? = if (config.search.enabled && config.search.elastic != null) {
            ElasticsearchClient(config.search.elastic!!, json)
        } else {
            null
        }

        val meili: MeilisearchClient? = if (config.search.enabled && config.search.meili != null) {
            MeilisearchClient(httpClient, config.search.meili!!)
        } else {
            null
        }

        val server = ChartedServer(config)
        startKoin {
            logger(KoinLogger(config))
            modules(
                chartedModule,
                *endpointsModule.toTypedArray(),
                module {
                    single<IRedisClient> { redis }
                    single { apiKeyExpiration }
                    single { ratelimiter }
                    single { httpClient }
                    single { scheduler }
                    single { sessions }
                    single { wrapper }
                    single { config }
                    single { server }
                    single { yaml }
                    single { json }
                    single { ds }

                    if (clickhouse != null) {
                        single { clickhouse }
                        if (config.isFeatureEnabled(Feature.AUDIT_LOGS)) {
                            single { AuditLogsFeature(get()) }
                        }
                    }

                    if (elastic != null) {
                        single { elastic }
                    }

                    if (meili != null) {
                        single { meili }
                    }
                }
            )
        }

        elastic?.connect()
        clickhouse?.connect()

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
