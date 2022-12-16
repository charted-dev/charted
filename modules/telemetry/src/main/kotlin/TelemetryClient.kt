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

package org.noelware.charted.modules.telemetry

// You can view the source of the telemetry server: https://github.com/Noelware/telemetry
const val TELEMETRY_SERVER: String = "https://telemetry.noelware.org"

/**
 * Represents a client for [Noelware Telemetry](https://telemetry.noelware.org) that is completely
 * opt-in and disabled by default
 */
interface TelemetryClient {
    /** Returns if this client is enabled or not. */
    val enabled: Boolean

    /**
     * Sends a telemetry packet to the server if it is enabled.
     * @param packet The telemetry packet
     */
    suspend fun send(packet: TelemetryPacket)
}
