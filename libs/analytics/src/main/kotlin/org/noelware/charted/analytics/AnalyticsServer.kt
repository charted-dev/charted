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

package org.noelware.charted.analytics

import dev.floofy.utils.slf4j.logging
import io.grpc.Server
import io.grpc.ServerBuilder
import org.noelware.charted.common.config.AnalyticsConfig
import java.util.concurrent.TimeUnit

class AnalyticsServer(private val config: AnalyticsConfig): AutoCloseable {
    private lateinit var server: Server
    private val log by logging<AnalyticsServer>()

    fun start() {
        log.info("Starting gRPC server...")

        server = ServerBuilder.forPort(config.port)
            .addService(AnalyticsService())
            .build()

        server.start()
        server.awaitTermination()
    }

    override fun close() {
        if (!this::server.isInitialized) {
            log.warn("gRPC server was never initialized, skipping")
            return
        }

        log.warn("Shutting down server...")
        server.shutdownNow()

        if (!server.awaitTermination(5, TimeUnit.SECONDS)) {
            log.warn("gRPC server couldn't be terminated, leaving it at this state.")
        } else {
            log.warn("gRPC server has been shutdown. :3")
        }
    }
}
