/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.server

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.server.bootstrap.BootstrapPhase
import org.slf4j.bridge.SLF4JBridgeHandler
import java.io.File

/**
 * Represents the server bootstrap, which... bootstraps and loads the server.
 */
object Bootstrap {
    private val log by logging<Bootstrap>()

    suspend fun start(configPath: File) {
        // Replace java.util.logging => slf4j so OpenTelemetry logs will match the ones
        // we configured in Logback
        SLF4JBridgeHandler.install()
        Thread.currentThread().name = "Server-BootstrapThread"

        for (phase in BootstrapPhase.PHASES) {
            log.debug("Initializing bootstrap phase [${phase::class.simpleName}]")
            phase.bootstrap(configPath)
        }
    }
}
