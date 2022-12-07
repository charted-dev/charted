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

import dev.floofy.utils.koin.injectOrNull
import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.Job
import kotlinx.coroutines.launch
import org.noelware.analytics.jvm.server.AnalyticsServer
import org.noelware.charted.ChartedScope
import org.noelware.charted.common.SetOnce
import org.noelware.charted.server.bootstrap.BootstrapPhase
import org.noelware.charted.server.bootstrap.ConfigureModulesPhase
import java.io.File

/**
 * Represents the server bootstrap, which... bootstraps and loads the server.
 */
object Bootstrap {
    internal val analyticsServerJob: SetOnce<Job> = SetOnce()
    private val log by logging<Bootstrap>()

    suspend fun start(configPath: File) {
        Thread.currentThread().name = "Server-BootstrapThread"

        for (phase in BootstrapPhase.PHASES) {
            log.debug("Initializing bootstrap phase [${phase::class}]")
            phase.bootstrap(configPath)

            if (phase == ConfigureModulesPhase) {
                val analyticsServer: AnalyticsServer? by injectOrNull()
                if (analyticsServer != null) {
                    analyticsServerJob.value = ChartedScope.launch {
                        analyticsServer!!.start()
                    }
                }
            }
        }
    }
}
