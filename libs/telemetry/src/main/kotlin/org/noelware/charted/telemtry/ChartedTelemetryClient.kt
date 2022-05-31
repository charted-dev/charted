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

package org.noelware.charted.telemtry

import kotlinx.serialization.json.JsonObject

/**
 * This is the telemetry client that is used to send out events like system metrics,
 * downloads, and such more.
 *
 * You can disable it using the `CHARTED_NO_TELEMETRY=1` environment variable or use the `config.telemetry`
 * configuration option.
 */
class ChartedTelemetryClient(private val enabled: Boolean = true) {
    companion object {
        /**
         * Represents the base URL that is used to send out. Do I really
         * care that this is public... probably not?
         *
         * It's impossible to send out events that aren't send by you.
         */
        const val BASE_URL = "https://telemetry.noelware.org/v1/send"
    }

    /**
     * Sends out a telemetry packet to the Noelware Telemetry
     * service.
     */
    suspend fun send(actions: JsonObject) {
        if (!enabled) return

        val packet = TelemetryPacket.create(actions)
    }
}
