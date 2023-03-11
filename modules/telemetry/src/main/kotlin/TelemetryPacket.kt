/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.descriptors.element
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.encoding.encodeStructure
import org.noelware.charted.ChartedInfo

@Serializable(with = TelemetryPacket.Serializer::class)
data class TelemetryPacket<T>(
    val distribution: ChartedInfo.Distribution,

    @SerialName("build_date")
    val buildDate: String,
    val version: String,
    val commit: String,
    val event: String,
    val data: T? = null
) {
    internal class Serializer<T>(private val innerSerializer: KSerializer<T>): KSerializer<TelemetryPacket<T>> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.telemetry.Packet") {
            element<ChartedInfo.Distribution>("distribution")
            element<String>("build_date")
            element<String>("version")
            element<String>("commit")
            element<String>("event")
            element("data", innerSerializer.descriptor, isOptional = true)
        }

        override fun serialize(encoder: Encoder, value: TelemetryPacket<T>) = encoder.encodeStructure(descriptor) {
            encodeSerializableElement(descriptor, 0, ChartedInfo.Distribution.serializer(), value.distribution)
            encodeStringElement(descriptor, 1, value.buildDate)
            encodeStringElement(descriptor, 2, value.version)
            encodeStringElement(descriptor, 3, value.commit)
            encodeStringElement(descriptor, 4, value.event)

            if (value.data != null) encodeSerializableElement(descriptor, 5, innerSerializer, value.data)
        }

        override fun deserialize(decoder: Decoder): TelemetryPacket<T> {
            TODO("We have no reason to implement deserialisation for TelemetryPacket<T>")
        }
    }
}
