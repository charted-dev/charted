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

import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonObject
import java.lang.management.ManagementFactory

private const val PRODUCT = "charted-server"
private const val VENDOR = "Noelware"

/**
 * Represents a telemetry packet.
 *
 * ## Example Structure
 * ```json
 * {
 *    "format": 1,
 *    "product": "charted-server",
 *    "vendor": "Noelware",
 *    "os": "Linux",
 *    "arch": "x86_64",
 *    "distribution": "Docker",
 *    "java_version": "v17.0.1 [Eclipse Temurin]",
 *    "actions": {
 *       "downloads": { "increment": 1 }
 *    }
 * }
 * ```
 */
@kotlinx.serialization.Serializable
data class TelemetryPacket(
    val product: String = PRODUCT,
    val vendor: String = VENDOR,
    val arch: String,
    val os: String,
    val version: String,
    val distribution: String,
    val actions: JsonObject,

    @SerialName("java_version")
    val javaVersion: String,
) {
    companion object {
        fun create(actions: JsonObject): TelemetryPacket {
            val os = ManagementFactory.getOperatingSystemMXBean()
            return TelemetryPacket(
                arch = os.arch,
                os = os.name,
                version = "?",
                distribution = "local",
                javaVersion = "${System.getProperty("java.version")} [${System.getProperty("java.vendor")}]",
                actions = actions
            )
        }
    }
}
