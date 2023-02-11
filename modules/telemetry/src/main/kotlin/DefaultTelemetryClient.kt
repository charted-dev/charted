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

package org.noelware.charted.modules.telemetry

import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.request.*
import org.noelware.charted.configuration.kotlin.dsl.Config

class DefaultTelemetryClient(config: Config, private val httpClient: HttpClient) : TelemetryClient {
    private val log by logging<DefaultTelemetryClient>()
    override val enabled: Boolean = config.telemetry

    override suspend fun send(packet: TelemetryPacket) {
        log.info("Sending telemetry packet @ POST $TELEMETRY_SERVER/api/track")
        httpClient.post("$TELEMETRY_SERVER/api/track") {
            header("Content-Type", "application/json; charset=utf-8")
            setBody(packet)
        }
    }
}
