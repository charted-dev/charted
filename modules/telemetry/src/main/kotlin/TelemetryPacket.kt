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

import kotlinx.serialization.*
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.encoding.encodeStructure
import org.noelware.charted.ChartedInfo
import org.noelware.charted.DistributionType
import org.noelware.charted.common.Architecture
import org.noelware.charted.common.OperatingSystem

/** Returns the product name */
const val PRODUCT = "charted-server"

/** Returns the software vendor of charted-server. */
const val VENDOR = "Noelware, LLC."

@Serializable(with = TelemetryPacketSerializer::class)
data class TelemetryPacket(
    val distribution: DistributionType = ChartedInfo.distribution,
    val product: String = PRODUCT,
    val version: String = ChartedInfo.version,
    val vendor: String = VENDOR,
    val arch: String = Architecture.current().key(),
    val os: String = OperatingSystem.current().key(),

    @Contextual
    val data: Any
) {
    companion object {
        @JvmStatic
        fun <T : Any> create(value: T): TelemetryPacket = TelemetryPacket(data = value)
    }
}

@OptIn(ExperimentalSerializationApi::class)
private object TelemetryPacketSerializer : KSerializer<TelemetryPacket> {
    private val contextualAnySerializer = ContextualSerializer(Any::class, null, emptyArray())
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.telemetry.Packet") {
        element("distribution", DistributionType.serializer().descriptor)
        element("product", String.serializer().descriptor)
        element("version", String.serializer().descriptor)
        element("vendor", String.serializer().descriptor)
        element("arch", String.serializer().descriptor)
        element("data", contextualAnySerializer.descriptor)
        element("os", String.serializer().descriptor)
    }

    override fun deserialize(decoder: Decoder): TelemetryPacket {
        throw SerializationException("Deserialization is not supported in TelemetryPacket")
    }

    @OptIn(InternalSerializationApi::class)
    @Suppress("UNCHECKED_CAST") // it works either way
    override fun serialize(encoder: Encoder, value: TelemetryPacket) = encoder.encodeStructure(descriptor) {
        val actualSerializer = encoder.serializersModule.getContextual(value.data::class)
            ?: value.data::class.serializer()

        encodeSerializableElement(descriptor, 0, DistributionType.serializer(), value.distribution)
        encodeStringElement(descriptor, 1, value.product)
        encodeStringElement(descriptor, 2, value.version)
        encodeStringElement(descriptor, 3, value.vendor)
        encodeStringElement(descriptor, 4, value.arch)
        encodeSerializableElement(descriptor, 5, actualSerializer as KSerializer<Any>, value.data)
        encodeStringElement(descriptor, 6, value.os)
    }
}
