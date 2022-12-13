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

package org.noelware.charted.modules.webhooks.events

import com.fasterxml.jackson.databind.ser.std.MapSerializer
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.builtins.MapSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.descriptors.element
import kotlinx.serialization.encoding.*
import kotlinx.serialization.encoding.CompositeDecoder.Companion.DECODE_DONE
import org.noelware.charted.serializers.AnySerializer

/**
 * Collection of {new => <any>, old => <any>} pairs that were updated.
 */
@Serializable(with = UpdatedMetadata.Serializer::class)
class UpdatedMetadata(val new: New, val old: Old) {
    // to trick the serialization compiler
    @Suppress("UNUSED")
    constructor(): this(New(mapOf()), Old(mapOf()))

    /**
     * Collection of new data that was updated on the server.
     *
     * ## Example
     * ```json
     * {
     *    "data": {
     *       "repository": [...],
     *       "updated": {
     *          "new": {
     *              "description": "Hello, world!"
     *          },
     *          "old": {
     *              "description": null
     *          }
     *       }
     *    }
     * }
     * ```
     */
    @Serializable(with = New.Companion::class)
    class New(val data: Map<String, @Serializable(with = AnySerializer::class) Any>) {
        companion object: KSerializer<New> {
            private val realSerializer = MapSerializer(String.serializer(), AnySerializer)

            override val descriptor: SerialDescriptor = realSerializer.descriptor
            override fun deserialize(decoder: Decoder): New = New(decoder.decodeSerializableValue(realSerializer))
            override fun serialize(encoder: Encoder, value: New) {
                encoder.encodeSerializableValue(realSerializer, value.data)
            }
        }
    }

    /**
     * Collection of the old data that was later updated on the server.
     *
     * ## Example
     * ```json
     * {
     *    "data": {
     *       "repository": [...],
     *       "updated": {
     *          "new": {
     *              "description": "Hello, world!"
     *          },
     *          "old": {
     *              "description": null
     *          }
     *       }
     *    }
     * }
     * ```
     */
    @Serializable(with = Old.Companion::class)
    class Old(val data: Map<String, @Serializable(with = AnySerializer::class) Any>) {
        companion object: KSerializer<Old> {
            private val realSerializer = MapSerializer(String.serializer(), AnySerializer)

            override val descriptor: SerialDescriptor = realSerializer.descriptor
            override fun deserialize(decoder: Decoder): Old = Old(realSerializer.deserialize(decoder))
            override fun serialize(encoder: Encoder, value: Old) {
                encoder.encodeSerializableValue(realSerializer, value.data)
            }
        }
    }

    /**
     * Serializer for [UpdatedMetadata].
     */
    class Serializer: KSerializer<UpdatedMetadata> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.webhooks.UpdatedMetadata") {
            element<New>("new")
            element<Old>("old")
        }

        override fun deserialize(decoder: Decoder): UpdatedMetadata = decoder.decodeStructure(descriptor) {
            var new: New? = null
            var old: Old? = null

            loop@ while (true) {
                when (val index = decodeElementIndex(descriptor)) {
                    DECODE_DONE -> break@loop
                    0 -> new = decodeSerializableElement(descriptor, index, New.serializer())
                    1 -> old = decodeSerializableElement(descriptor, index, Old.serializer())
                    else -> throw SerializationException("Unexpected index @ $index")
                }
            }

            if (new == null) new = New(mapOf())
            if (old == null) old = Old(mapOf())

            UpdatedMetadata(new, old)
        }

        override fun serialize(encoder: Encoder, value: UpdatedMetadata) = encoder.encodeStructure(descriptor) {
            encodeSerializableElement(descriptor, 0, New.serializer(), value.new)
            encodeSerializableElement(descriptor, 1, Old.serializer(), value.old)
        }
    }
}
