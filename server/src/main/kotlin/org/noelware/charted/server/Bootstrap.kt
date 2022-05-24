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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.server

import com.akuleshov7.ktoml.Toml
import com.akuleshov7.ktoml.TomlConfig
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import dev.floofy.utils.koin.retrieveOrNull
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import kotlinx.coroutines.cancel
import kotlinx.coroutines.runBlocking
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.Slf4jSqlDebugLogger
import org.jetbrains.exposed.sql.transactions.transaction
import org.koin.core.context.GlobalContext
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.core.*
import org.noelware.charted.core.config.Config
import org.noelware.charted.core.config.EngineClass
import org.noelware.charted.core.jobs.jobsModule
import org.noelware.charted.core.logging.SentryLogger
import org.noelware.charted.core.redis.IRedisClient
import org.noelware.charted.core.redis.RedisClient
import org.noelware.charted.core.sessions.SessionManager
import org.noelware.charted.database.createOrUpdateEnums
import org.noelware.charted.database.tables.*
import org.noelware.charted.engine.oci.OciBackendEngine
import org.noelware.charted.engines.charts.ChartBackendEngine
import org.noelware.charted.search.elastic.ElasticsearchBackend
import org.noelware.charted.search.meili.MeilisearchBackend
import org.noelware.charted.server.endpoints.endpointsModule
import java.io.File
import java.io.IOError
import java.util.UUID
import kotlin.concurrent.thread
import kotlin.system.exitProcess

object Bootstrap {
    private val log by logging<Bootstrap>()

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

        installShutdownHook()
        installDefaultThreadExceptionHandler()
        createUUID()

        log.info("Loading configuration...")

        // Configure the server config
        val fullConfigPath = System.getenv("CHARTED_CONFIG_PATH") ?: "./config.toml"
        val configFile = File(fullConfigPath)

        if (!configFile.exists())
            throw IllegalArgumentException("Missing configuration path in $fullConfigPath.")

        if (configFile.extension != "toml")
            throw IllegalStateException("Configuration file $fullConfigPath must be a TOML file (must be `.toml` extension, not ${configFile.extension})")

        val toml = Toml(
            TomlConfig(
                ignoreUnknownNames = true,
                allowEmptyToml = true,
                allowEmptyValues = false,
                allowEscapedQuotesInLiteralStrings = true
            )
        )

        val config = toml.decodeFromString(Config.serializer(), configFile.readText())

        if (config.jwtSecretKey.isEmpty())
            throw IllegalStateException("You need to set a JWT secret key!")

        log.info("Loaded configuration in $fullConfigPath! Connecting to PostgreSQL...")

        val dataSource = HikariDataSource(
            HikariConfig().apply {
                jdbcUrl = "jdbc:postgresql://${config.database.host}:${config.database.port}/${config.database.name}"
                username = config.database.username
                password = config.database.password
                schema = config.database.schema
                driverClassName = "org.postgresql.Driver"
                isAutoCommit = false
                leakDetectionThreshold = 30 * 1000
                poolName = "ChartedServer-HikariPool"
            }
        )

        Database.connect(
            dataSource,
            databaseConfig = DatabaseConfig {
                defaultRepetitionAttempts = 5
                sqlLogger = if ((System.getenv("org.noelware.charted.debug") ?: "false") == "true") {
                    Slf4jSqlDebugLogger
                } else {
                    null
                }
            }
        )

        runBlocking {
            asyncTransaction(ChartedScope) {
                createOrUpdateEnums()
            }
        }

        transaction {
            // the order matters :(
            SchemaUtils.createMissingTablesAndColumns(
                Organizations,
                Users,
                Repositories,
                RepositoryMembers,
                OrganizationMember,
                UserConnections
            )
        }

        log.info("Connected to PostgreSQL!")

        val wrapper = if (config.storage != null) {
            log.info("Storage is enabled on the server! Enabling...")
            StorageWrapper(config.storage!!)
        } else {
            log.info("Storage was not enabled on the server, assuming we're using OCI engine.")
            null
        }

        val redis = RedisClient(config.redis)
        redis.connect()

        // Check if we need to enable Sentry
        if (config.sentryDsn != null) {
            log.info("Installing Sentry...")
            Sentry.init {
                it.setLogger(SentryLogger)
                it.dsn = config.sentryDsn
                it.release = "hana v${ChartedInfo.version} (${ChartedInfo.commitHash})"
            }

            Sentry.configureScope {
                it.tags += mapOf(
                    "charted.version" to ChartedInfo.version,
                    "charted.commit" to ChartedInfo.commitHash,
                    "charted.build.date" to ChartedInfo.buildDate,
                    "system.user" to System.getProperty("user.name")
                )
            }

            log.info("Sentry is now enabled with DSN ${config.sentryDsn}")
        }

        val koin = startKoin {
            modules(
                chartedModule,
                endpointsModule,
                jobsModule,
                module {
                    single { toml }
                    single { config }
                    single { dataSource }
                    single<IRedisClient> { redis }
                    single { ChartedServer() }
                    single { SessionManager(get(), get(), get()) }

                    if (config.engine?.engineClass == EngineClass.CHART) {
                        single { ChartBackendEngine(get()) }
                    }

                    if (config.engine?.engineClass == EngineClass.OCI) {
                        require(config.engine?.ociConfig != null) { "Missing OCI engine configuration (`config.engine.oci`)" }
                        single { OciBackendEngine(get()) }
                    }

                    wrapper?.let {
                        single { wrapper }
                    }

                    config.search.elastic?.let { elastic ->
                        single { ElasticsearchBackend(elastic) }
                    }

                    config.search.meili?.let { meili ->
                        single { MeilisearchBackend(meili, get()) }
                    }
                }
            )
        }

        runBlocking {
            val server = koin.koin.get<ChartedServer>()
            try {
                // Initialize the session manager here
                koin.koin.get<SessionManager>()
                server.start()
            } catch (e: Exception) {
                log.error("Unable to bootstrap charted-server:", e)

                // we do not let the shutdown hooks run
                // since in some cases, it'll just error out or whatever
                //
                // example: Elasticsearch cannot index all data due to
                // I/O locks or what not (and it'll keep looping)
                halt(120)
                exitProcess(1)
            }
        }
    }

    private fun createUUID() {
        val file = File("./instance.uuid")
        if (!file.exists()) {
            file.writeBytes(UUID.randomUUID().toString().toByteArray())
            log.warn("Instance UUID didn't exist in ./instance.uuid, so I created it!")
            log.warn("If this was used with Noelware Analytics, edit the instance!!")
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

                // Check if Koin has started
                val koinStarted = GlobalContext.getKoinApplicationOrNull() != null
                if (koinStarted) {
                    val server by inject<ChartedServer>()
                    val elasticsearch = GlobalContext.retrieveOrNull<ElasticsearchBackend>()
                    val redis by inject<IRedisClient>()
                    val ds by inject<HikariDataSource>()
                    val sessions by inject<SessionManager>()

                    elasticsearch?.close()
                    redis.close()
                    ds.close()
                    server.destroy()
                    sessions.close()

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
        Thread.setDefaultUncaughtExceptionHandler { t, e ->
            if (e is Error) {
                log.error("Uncaught fatal error in thread ${t.name} (#${t.id}):", e)
                log.error("If this keeps occurring, please report it to Noelware: https://github.com/charted-dev/charted/issues")

                var success = false

                if (e is InternalError) {
                    success = true
                    halt(128)
                }

                if (e is OutOfMemoryError) {
                    success = true
                    halt(127)
                }

                if (e is StackOverflowError) {
                    success = true
                    halt(126)
                }

                if (e is UnknownError) {
                    success = true
                    halt(125)
                }

                if (e is IOError) {
                    success = true
                    halt(124)
                }

                if (e is LinkageError) {
                    success = true
                    halt(123)
                }

                if (!success) halt(120)

                exitProcess(1)
            } else {
                log.error("Uncaught exception in thread ${t.name} (#${t.id}):", e)

                // If any thread had an exception, let's check if:
                //  - The server has started (must be set if the Application hook has run)
                //  - If the thread names are the bootstrap or shutdown thread
                val started = ChartedServer.hasStarted != null && ChartedServer.hasStarted == true
                if (!started && (t.name == "Server-BootstrapThread" || t.name == "Server-ShutdownThread")) {
                    halt(120)
                    exitProcess(1)
                }
            }
        }
    }
}
