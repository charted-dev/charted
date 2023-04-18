/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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
import dev.floofy.utils.koin.inject
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
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.ChartedInfo
import org.noelware.charted.SNOWFLAKE_EPOCH
import org.noelware.charted.Server
import org.noelware.charted.common.extensions.formatting.doFormatTime
import org.noelware.charted.configuration.ConfigurationHost
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.enumSets.serialName
import org.noelware.charted.configuration.kotlin.dsl.tracing.TracerType
import org.noelware.charted.configuration.kotlin.host.KotlinScriptConfigurationHost
import org.noelware.charted.configuration.yaml.YamlConfigurationHost
import org.noelware.charted.modules.avatars.AvatarModule
import org.noelware.charted.modules.avatars.DefaultAvatarModule
import org.noelware.charted.modules.emails.DefaultEmailService
import org.noelware.charted.modules.emails.EmailService
import org.noelware.charted.modules.helm.charts.DefaultHelmChartModule
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.metrics.collectors.ServerInfoMetrics
import org.noelware.charted.modules.metrics.disabled.DisabledMetricsSupport
import org.noelware.charted.modules.metrics.prometheus.PrometheusMetricsSupport
import org.noelware.charted.modules.postgresql.configure
import org.noelware.charted.modules.postgresql.controllers.controllersModule
import org.noelware.charted.modules.postgresql.metrics.PostgresServerStats
import org.noelware.charted.modules.postgresql.tables.*
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.search.SearchModule
import org.noelware.charted.modules.search.elasticsearch.DefaultElasticsearchModule
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.modules.sessions.local.LocalSessionManager
import org.noelware.charted.modules.storage.DefaultStorageModule
import org.noelware.charted.modules.storage.StorageModule
import org.noelware.charted.modules.tracing.Tracer
import org.noelware.charted.modules.tracing.multitenant.MultiTenantTracer
import org.noelware.charted.modules.tracing.sentry.SentryTracer
import org.noelware.charted.server.internal.DefaultServer
import org.noelware.charted.server.routing.routingModule
import org.noelware.charted.snowflake.Snowflake
import org.slf4j.MDC
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import java.io.File

private val yaml = Yaml(
    EmptySerializersModule(),
    YamlConfiguration(
        strictMode = true,
    ),
)

fun getConfigurationHost(configFile: File): ConfigurationHost = when {
    listOf("yaml", "yml").contains(configFile.extension) -> YamlConfigurationHost(yaml)
    configFile.extension.contains("kts") -> KotlinScriptConfigurationHost
    else -> throw IllegalStateException("Unable to determine what configuration host to use")
}

object ConfigureModulesPhase: BootstrapPhase() {
    private val log by logging<ConfigureModulesPhase>()

    override suspend fun phaseThrough(@Suppress("PARAMETER_NAME_CHANGED_ON_OVERRIDE") configFile: File) {
        MDC.put("bootstrap.phase", "configure modules")

        // Determine the configuration host to... actually load the configuration
        // file.
        val configHost = getConfigurationHost(configFile)
        val realConfigPath = withContext(Dispatchers.IO) {
            configFile.toPath().toRealPath()
        }

        log.info("Loading configuration file from path [$realConfigPath]")
        return phaseThrough(configHost.load(configFile))
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    suspend fun phaseThrough(config: Config) {
        val sw = StopWatch.createStarted()
        DebugProbes.enableCreationStackTraces = config.debug
        DebugProbes.install()

        sw.suspend()
        log.info("Loaded configuration in [${sw.doFormatTime()}], configuring PostgreSQL connection...")

        sw.resume()

        val ds = configure(config, sw)
        if (config.sentryDsn != null) {
            log.debug("Enabling Sentry with DSN [${config.sentryDsn}]")
            Sentry.init {
                it.release = "charted-server v${ChartedInfo.version}+${ChartedInfo.commitHash}"
                it.dsn = config.sentryDsn
            }
        }

        val redis = DefaultRedisClient(config.redis)
        redis.connect()

        val storage = DefaultStorageModule(config.storage)
        storage.init()

        val json = Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }

        // 1677654000000 = March 1st, 2023
        val snowflake = Snowflake(0, SNOWFLAKE_EPOCH)
        val argon2 = Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8()
        val metrics = if (config.metrics.enabled) {
            PrometheusMetricsSupport(ds)
        } else {
            DisabledMetricsSupport()
        }

        metrics.add(PostgresServerStats.Collector(config))
        metrics.add(
            ServerInfoMetrics.Collector {
                val server: Server by inject()
                (server as DefaultServer).requests
            },
        )

        val httpClient = HttpClient(Java) {
            install(ContentNegotiation) {
                this.json(json)
            }
        }

        val koinModule = module {
            single<AbstractSessionManager> { LocalSessionManager(json, config, redis, argon2) }
            single<HelmChartModule> { DefaultHelmChartModule(storage, config, yaml) }
            single { EmailValidator.getInstance(true, true) }
            single<AvatarModule> { DefaultAvatarModule(storage, httpClient) }
            single<Server> { DefaultServer(config) }
            single<StorageModule> { storage }
            single<RedisClient> { redis }
            single { httpClient }
            single { snowflake }
            single { argon2 }
            single { config }
            single { yaml }
            single { json }
            single { ds }
        }

        val modules = mutableListOf(
            *routingModule.toTypedArray(),
            controllersModule,
            koinModule,
        )

        if (config.emailsGrpcEndpoint != null) {
            log.debug("Configuring emails gRPC microservice with URI [${config.emailsGrpcEndpoint}]")
            val service = DefaultEmailService(config)

            val ok = service.ping()
            if (!ok) log.warn("Unable to ping gRPC endpoint, this might not work correctly!")

            modules.add(
                module {
                    single<EmailService> { service }
                },
            )
        }

        if (config.search != null) {
            if (config.search!!.elasticsearch != null) {
                val elasticsearchModule = DefaultElasticsearchModule(json, config)
                elasticsearchModule.init()

                modules.add(
                    module {
                        single<SearchModule> { elasticsearchModule }
                    },
                )
            }
        }

        if (config.tracers.isNotEmpty()) {
            log.info("Configuring tracers [${config.tracers.joinToString(", ") { it.type.serialName!! }}")

            val tracer = configureTracing(config)
            if (tracer != null) {
                Tracer.setGlobal(tracer)
            }
        }

        modules.add(
            module {
                single { metrics }
            },
        )

        // remove it so the koin logger doesn't inherit this
        MDC.remove("bootstrap.phase")
        startKoin {
            modules(*modules.toTypedArray())
        }
    }

    private fun configureTracing(config: Config): Tracer? {
        if (config.tracers.size == 1) {
            val tracer = config.tracers.first()
            return when (tracer.type) {
                TracerType.OpenTelemetry, TracerType.ElasticAPM -> null
                TracerType.Sentry -> {
                    if (config.sentryDsn == null) {
                        log.warn("Unable to configure Sentry tracer due to no Sentry DSN provided, not tracing with Sentry")
                        return null
                    }

                    SentryTracer
                }

                else -> null
            }
        }

        val tracers: MutableList<Tracer> = mutableListOf()
        for (tracer in config.tracers) {
            when (tracer.type) {
                TracerType.OpenTelemetry, TracerType.ElasticAPM -> {}
                TracerType.Sentry -> {
                    if (config.sentryDsn == null) {
                        log.warn("Unable to configure Sentry tracer due to no Sentry DSN provided, not tracing with Sentry")
                        continue
                    }

                    tracers.add(SentryTracer)
                }

                else -> {}
            }
        }

        return MultiTenantTracer(tracers)
    }
}
