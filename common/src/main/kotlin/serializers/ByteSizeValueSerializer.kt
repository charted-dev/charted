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

package org.noelware.charted.serializers

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.Serializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import org.noelware.charted.common.ByteSizeValue

@OptIn(ExperimentalSerializationApi::class)
@Serializer(forClass = ByteSizeValue::class)
object ByteSizeValueSerializer: KSerializer<Long> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.ByteSizeValue", PrimitiveKind.LONG)
    override fun deserialize(decoder: Decoder): Long = try {
        val str = decoder.decodeString()
        ByteSizeValue.fromString(str)
    } catch (_: SerializationException) {
        decoder.decodeLong()
    }

    override fun serialize(encoder: Encoder, value: Long) {
        encoder.encodeString(ByteSizeValue.fromLong(value))
    }
}
