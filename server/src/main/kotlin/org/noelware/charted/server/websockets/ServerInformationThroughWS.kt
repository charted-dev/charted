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

package org.noelware.charted.server.websockets

import dev.floofy.utils.slf4j.logging
import io.ktor.server.application.*
import io.ktor.server.routing.*
import io.ktor.server.websocket.*
import io.ktor.websocket.*
import org.noelware.charted.core.Ticker
import org.noelware.charted.server.plugins.IsAdminGuard
import org.noelware.charted.server.plugins.Sessions
import org.noelware.charted.server.session
import org.noelware.ktor.realIP
import kotlin.time.Duration.Companion.seconds

private val clients = mutableListOf<DefaultWebSocketServerSession>()
private val tickers = mutableListOf<Ticker>()

suspend fun shutdownTickers() {
    for (client in clients) {
        client.close(CloseReason(CloseReason.Codes.GOING_AWAY, "Server is shutting down!"))
    }

    tickers.forEach { it.cancel() }
    tickers.clear()
}

/**
 * This method will enable Pak to pull server information via a WebSocket. The WebSocket can
 * only be accessed if you are an administrator.
 */
fun Routing.handleServerInfoThroughWebSockets(path: String = "/admin/stats") {
    val log by logging("org.noelware.charted.server.websockets.ServerInfoWebSocket")
    webSocket(path) {
        install(Sessions)
        install(IsAdminGuard)

        clients.add(this)
        val session = call.session
        log.debug("Received WebSocket connection from [${call.realIP}]")
        log.info("User [${session.userID}] has connected to the admin statistics socket~")

        val ticker = Ticker("update admin stats [${session.sessionID}]", 15.seconds)
        ticker.launch {}

        tickers.add(ticker)
    }
}
