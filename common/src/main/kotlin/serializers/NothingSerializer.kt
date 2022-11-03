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

package org.noelware.charted.serializers

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.Serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

/**
 * Represents a kotlinx.serialization serializer for the [Nothing] object, which just throws
 * a [SerializationException] when being used.
 */
@OptIn(ExperimentalSerializationApi::class)
@Serializer(forClass = Nothing::class)
object NothingSerializer: KSerializer<Nothing> {
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor("kotlin.Nothing")

    override fun deserialize(decoder: Decoder): Nothing {
        throw SerializationException("Can't deserialize this object.")
    }

    override fun serialize(encoder: Encoder, value: Nothing) {
        throw SerializationException("Can't serialize this object.")
    }
}
