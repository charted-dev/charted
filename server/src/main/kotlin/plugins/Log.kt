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

package org.noelware.charted.server.plugins

import dev.floofy.utils.koin.inject
import dev.floofy.utils.kotlin.doFormatTime
import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.application.hooks.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.*
import io.prometheus.client.Histogram
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.Server
import org.noelware.charted.common.extensions.formatting.doFormatTime
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.prometheus.PrometheusMetricsSupport
import org.noelware.charted.server.internal.DefaultServer
import org.noelware.charted.server.internal.bootTime
import org.noelware.charted.server.internal.hasStarted
import org.slf4j.MDC

object Log: BaseApplicationPlugin<ApplicationCallPipeline, Unit, Log> {
    private val histogramKey: AttributeKey<Histogram.Timer> = AttributeKey("Prometheus Histogram Timer")
    private val stopwatchKey: AttributeKey<StopWatch> = AttributeKey("Stopwatch")
    private val log by logging("org.noelware.charted.server.plugins.KtorLogger")

    private val metrics: MetricsSupport by inject()
    private val server: Server by inject()

    override val key: AttributeKey<Log> = AttributeKey("Logging Plugin")
    override fun install(pipeline: ApplicationCallPipeline, configure: Unit.() -> Unit): Log {
        install(pipeline)
        return this
    }

    private fun install(pipeline: ApplicationCallPipeline) {
        val environment = pipeline.environment!!
        environment.monitor.subscribe(ApplicationStarted, this::onApplicationStarted)
        environment.monitor.subscribe(ApplicationStopped, this::onApplicationStopped)

        // equivalent: onCall { call -> }
        pipeline.intercept(ApplicationCallPipeline.Plugins) {
            call.attributes.put(stopwatchKey, StopWatch.createStarted())
            (server as DefaultServer).requestsHandled.incrementAndGet()

            MDC.put("http.method", call.request.httpMethod.value)
            MDC.put("http.version", call.request.httpVersion)
            MDC.put("http.url", call.request.path())

            if (metrics is PrometheusMetricsSupport) {
                val m = metrics as PrometheusMetricsSupport
                m.serverRequests.inc()

                call.attributes.put(
                    histogramKey,
                    m.serverRequestLatency.labels(
                        call.request.httpMethod.value,
                        call.request.path(),
                        call.request.httpVersion,
                    ).startTimer(),
                )
            }
        }

        // equivalent: on(ResponseSent) { call -> }
        pipeline.sendPipeline.intercept(ApplicationSendPipeline.Engine) {
            val method = call.request.httpMethod
            val version = call.request.httpVersion
            val endpoint = call.request.path()
            val status = call.response.status() ?: HttpStatusCode(-1, "Unknown HTTP Method")
            val histogram = call.attributes.getOrNull(histogramKey)
            val stopwatch = call.attributes[stopwatchKey]
            val userAgent = call.request.userAgent() ?: "Unknown"

            stopwatch.stop()
            histogram?.observeDuration()
            log.info("~> ${method.value} $endpoint [HTTP $version] ~ ${status.value} ${status.description} [$userAgent (${stopwatch.doFormatTime()})]")

            MDC.remove("http.method")
            MDC.remove("http.version")
            MDC.remove("http.url")
        }
    }

    private fun onApplicationStarted(app: Application) {
        val time = (System.nanoTime() - bootTime).doFormatTime()

        log.info("API server has started in $time")
        hasStarted.value = true
    }

    private fun onApplicationStopped(app: Application) {
        app.environment.monitor.unsubscribe(ApplicationStarted, this::onApplicationStarted)
        app.environment.monitor.unsubscribe(ApplicationStopped, this::onApplicationStopped)

        log.warn("API server has successfully shut down!")
        hasStarted.value = false
    }
}
