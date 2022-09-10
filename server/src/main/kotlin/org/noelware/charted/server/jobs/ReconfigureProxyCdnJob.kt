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

package org.noelware.charted.server.jobs

import dev.floofy.haru.abstractions.AbstractJob
import dev.floofy.utils.slf4j.logging
import io.ktor.server.application.*
import io.ktor.server.routing.*
import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.endpoints.proxyStorageTrailer

class ReconfigureProxyCdnJob(
    private val server: ChartedServer,
    private val storage: StorageWrapper,
    private val config: Config
): AbstractJob(
    name = "reconfigure proxy cdn",
    expression = "* * * * *"
) {
    private val log by logging<ReconfigureProxyCdnJob>()

    override suspend fun execute() {
        log.debug("Reconfiguring CDN proxy...")

        val routing = server.server.application.plugin(Routing)
        routing.proxyStorageTrailer(storage)
    }
}
