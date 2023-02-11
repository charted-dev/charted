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

package org.noelware.charted.types.responses

import kotlinx.serialization.*
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.*
import kotlinx.serialization.json.JsonEncoder

/**
 * Represents an API error that might occur in a REST handler. It contains the [code]
 * and [message] elements to give a better understanding on what happened. You can read up
 * on all the codes here: https://charts.noelware.org/docs/server/api/reference#error-codes.
 *
 * @param code The error code that broaden what happened
 * @param message Human-readable message of the [code] element.
 * @param detail Extra detail to give more context on what happened.
 */
@Serializable(with = ApiError.Companion::class)
public data class ApiError(
    val code: String,
    val message: String,

    @Contextual
    val detail: Any? = null
) {
    @OptIn(ExperimentalSerializationApi::class)
    public companion object : KSerializer<ApiError> {
        public val EMPTY: ApiError = ApiError("", "")

        private val CONTEXTUAL_ANY = ContextualSerializer(Any::class, null, emptyArray())
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.ApiError") {
            element("code", String.serializer().descriptor)
            element("message", String.serializer().descriptor)
            element("detail", CONTEXTUAL_ANY.descriptor, isOptional = true)
        }

        override fun deserialize(decoder: Decoder): ApiError = decoder.decodeStructure(descriptor) {
            var code: String? = null
            var message: String? = null
            // var detail: Any? = null

            loop@ while (true) {
                when (val index = decodeElementIndex(descriptor)) {
                    CompositeDecoder.DECODE_DONE -> break
                    0 -> code = decodeStringElement(descriptor, index)
                    1 -> message = decodeStringElement(descriptor, index)
                    2 -> throw IllegalStateException("Decoding `detail` is not supported at this time")
                    else -> throw SerializationException("Unexpected index [$index]")
                }
            }

            check(code != null && message != null) { "Missing `code` and/or `message` in deserialized result" }
            ApiError(code, message, null)
        }

        @OptIn(InternalSerializationApi::class)
        @Suppress("UNCHECKED_CAST")
        override fun serialize(encoder: Encoder, value: ApiError) {
            require(encoder is JsonEncoder) { "Json encoding is only supported in ApiError, not ${encoder::class}" }
            encoder.encodeStructure(descriptor) {
                encodeStringElement(descriptor, 0, value.code)
                encodeStringElement(descriptor, 1, value.message)

                if (value.detail != null) {
                    val actualSerializer = encoder.serializersModule.getContextual(value.detail::class) ?: value.detail::class.serializer()
                    encodeSerializableElement(descriptor, 2, actualSerializer as KSerializer<Any>, value.detail)
                }
            }
        }
    }
}
