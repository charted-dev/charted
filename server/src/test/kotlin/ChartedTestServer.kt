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

package org.noelware.charted.server.tests

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlException
import com.zaxxer.hikari.HikariConfig
import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.plugins.autohead.*
import io.ktor.server.plugins.contentnegotiation.*
import io.ktor.server.plugins.doublereceive.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.testing.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.jetbrains.exposed.sql.Database
import org.jetbrains.exposed.sql.DatabaseConfig
import org.jetbrains.exposed.sql.SchemaUtils
import org.jetbrains.exposed.sql.Slf4jSqlDebugLogger
import org.jetbrains.exposed.sql.transactions.transaction
import org.koin.core.context.GlobalContext.startKoin
import org.koin.dsl.module
import org.noelware.charted.MultiValidationException
import org.noelware.charted.ValidationException
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.storage.FilesystemStorageConfig
import org.noelware.charted.configuration.kotlin.dsl.storage.StorageConfig
import org.noelware.charted.databases.postgres.createOrUpdateEnums
import org.noelware.charted.databases.postgres.tables.*
import org.noelware.charted.modules.apikeys.ApiKeyManager
import org.noelware.charted.modules.apikeys.DefaultApiKeyManager
import org.noelware.charted.modules.avatars.avatarsModule
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
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.endpoints.v1.endpointsModule
import org.noelware.charted.server.hasStarted
import org.noelware.charted.server.plugins.Logging
import org.noelware.charted.snowflake.Snowflake
import org.noelware.charted.types.responses.ApiError
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.NoelKtorRouting
import org.noelware.ktor.loader.koin.KoinEndpointLoader
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import kotlin.time.Duration.Companion.seconds

/**
 * Creates a new [ChartedTestServer] with the given [config] and [testFunction] block.
 * @param config The configuration to apply
 * @param module The module to apply (if any)
 * @param testFunction Function to perform testing on
 */
internal fun withChartedServer(config: Config, module: Application.() -> Unit = {}, testFunction: suspend ApplicationTestBuilder.() -> Unit) {
    val server = ChartedTestServer(config)
    server.testFunction = {
        application {
            module()
        }

        testFunction()
    }

    server.start()
}

internal class ChartedTestServer(private val config: Config): ChartedServer {
    private val _test: SetOnce<suspend ApplicationTestBuilder.() -> Unit> = SetOnce()
    private val log by logging<ChartedServer>()

    /**
     * Returns the function used to test the server.
     */
    var testFunction: suspend ApplicationTestBuilder.() -> Unit
        get() = _test.value
        set(value) {
            _test.value = value
        }

    /**
     * Checks if the server has started or not.
     */
    override val started: Boolean
        get() = hasStarted.get()

    /**
     * The application engine that Ktor is using for the server.
     */
    override val server: ApplicationEngine
        get() = throw IllegalStateException("Server is not allowed to be fetched")

    /**
     * Extension function to tailor the application module for this [ChartedServer]
     * instance.
     */
    override fun Application.module() {
        val self = this@ChartedTestServer

        install(AutoHeadResponse)
        install(DoubleReceive)
        install(Logging)

        install(ContentNegotiation) {
            json(
                Json {
                    ignoreUnknownKeys = true
                    isLenient = true
                },
            )
        }

        install(StatusPages) {
            // We have to do this to guard the content length since it can be null! If it is,
            // display a generic 404 message.
            statuses[HttpStatusCode.NotFound] = { call, content, _ ->
                if (content.contentLength == null) {
                    call.respond(
                        HttpStatusCode.NotFound,
                        ApiResponse.err(
                            "REST_HANDLER_NOT_FOUND", "Route handler was not found",
                            buildJsonObject {
                                put("method", call.request.httpMethod.value)
                                put("url", call.request.path())
                            },
                        ),
                    )
                }
            }

            status(HttpStatusCode.MethodNotAllowed) { call, _ ->
                call.respond(
                    HttpStatusCode.MethodNotAllowed,
                    ApiResponse.err(
                        "INVALID_REST_HANDLER", "Route handler was not the right method",
                        buildJsonObject {
                            put("method", call.request.httpMethod.value)
                            put("url", call.request.path())
                        },
                    ),
                )
            }

            status(HttpStatusCode.UnsupportedMediaType) { call, _ ->
                val header = call.request.header("Content-Type")
                call.respond(
                    HttpStatusCode.UnsupportedMediaType,
                    ApiResponse.err("UNSUPPORTED_CONTENT_TYPE", "Invalid content type [$header], was expecting \"application/json\""),
                )
            }

            status(HttpStatusCode.NotImplemented) { call, _ ->
                call.respond(
                    HttpStatusCode.NotImplemented,
                    ApiResponse.err(
                        "REST_HANDLER_UNAVAILABLE", "Route handler is not implemented at this moment!",
                        buildJsonObject {
                            put("method", call.request.httpMethod.value)
                            put("url", call.request.path())
                        },
                    ),
                )
            }

            exception<MultiValidationException> { call, cause ->
                self.log.error("Received multiple validation exceptions on REST handler [${call.request.httpMethod.value} ${call.request.path()}]")
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    cause.exceptions().map { ApiError("VALIDATION_EXCEPTION", it.validationMessage) },
                )
            }

            exception<ValidationException> { call, cause ->
                self.log.error("Received an validation exception on REST handler [${call.request.httpMethod.value} ${call.request.path()}] ~> ${cause.path} [${cause.validationMessage}]")
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    ApiResponse.err("VALIDATION_EXCEPTION", cause.validationMessage),
                )
            }

            exception<SerializationException> { call, cause ->
                self.log.error("Received serialization exception in handler [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                call.respond(
                    HttpStatusCode.PreconditionFailed,
                    ApiResponse.err("SERIALIZATION_FAILED", cause.message!!),
                )
            }

            exception<YamlException> { call, cause ->
                self.log.error("Unknown YAML exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]:", cause)
                call.respond(
                    HttpStatusCode.NotAcceptable,
                    ApiResponse.err(cause),
                )
            }

            exception<Exception> { call, cause ->
                self.log.error("Unknown exception had occurred while handling request [${call.request.httpMethod.value} ${call.request.path()}]", cause)
                call.respond(
                    HttpStatusCode.InternalServerError,
                    ApiResponse.err(
                        "INTERNAL_SERVER_ERROR", cause.message ?: "(unknown)",
                        buildJsonObject {
                            if (cause.cause != null) {
                                put(
                                    "cause",
                                    buildJsonObject {
                                        put("message", cause.cause!!.message ?: "(unknown)")
                                        if (self.config.debug) {
                                            put("stacktrace", cause.cause!!.stackTraceToString())
                                        }
                                    },
                                )
                            }

                            if (self.config.debug) {
                                put("stacktrace", cause.stackTraceToString())
                            }
                        },
                    ),
                )
            }
        }

        routing {}

        install(NoelKtorRouting) {
            endpointLoader(KoinEndpointLoader)
        }
    }

    /**
     * Starts the server, this will be a no-op if [started] was already
     * set to `true`.
     */
    override fun start() {
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
                sqlLogger = Slf4jSqlDebugLogger
            },
        )

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

        val redis = DefaultRedisClient(config.redis)
        runBlocking { redis.connect() }

        val argon2 = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
        val sessions = LocalSessionManager(argon2, redis, Json, config)
        val httpClient = HttpClient(OkHttp) {
            engine {
                config {
                    followSslRedirects(true)
                    followRedirects(true)
                }
            }

            install(io.ktor.client.plugins.contentnegotiation.ContentNegotiation) {
                json()
            }
        }

        val storage = DefaultStorageHandler(StorageConfig(filesystem = FilesystemStorageConfig("./.data")))
        storage.init()

        val apiKeyManager = DefaultApiKeyManager(redis)

        // Start Koin with only the configuration (I think?)
        startKoin {
            modules(
                avatarsModule,
                *endpointsModule.toTypedArray(),
                module {
                    single<StorageHandler> { storage }
                    single { Snowflake(0, 1669791600000) }
                    single { httpClient }
                    single { config }
                    single { argon2 }
                    single<ApiKeyManager> { apiKeyManager }
                    single<SessionManager> { sessions }
                    single<RedisClient> { redis }
                    single<HelmChartModule> { DefaultHelmChartModule(storage, config, Yaml.default) }
                    single<MetricsSupport> { DisabledMetricsSupport() }
                },
            )
        }

        // Run the test server
        testApplication {
            application {
                module()
            }

            testFunction()
        }
    }

    override fun close() {
        /* we don't close it since the test application does for us */
    }
}
