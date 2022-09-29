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

package org.noelware.charted.testing.server.junit

import dev.floofy.utils.slf4j.logging
import org.junit.jupiter.api.extension.AfterEachCallback
import org.junit.jupiter.api.extension.ExtensionContext
import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.testing.server.ChartedTestServer

class ServerCleanupExtension: AfterEachCallback {
    private val servers: MutableList<ChartedTestServer> = mutableListOf()
    private val log by logging<ServerCleanupExtension>()

    fun createServer(configure: Config.() -> Unit = {}): ChartedTestServer {
        val server = ChartedTestServer(configure)
        servers.add(server)

        return server
    }

    override fun afterEach(context: ExtensionContext) {
        log.info("Shutting down ${servers.size} servers!")
        for (server in servers) {
        }
    }
}
