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

package org.noelware.charted.common.types.responses

import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.*
import kotlinx.serialization.encoding.CompositeDecoder.Companion.DECODE_DONE
import kotlinx.serialization.json.JsonEncoder

/**
 * Represents a generic API response object.
 */
@Serializable(with = KResponseSerializer::class)
public sealed class ApiResponse<out T>(public val success: Boolean) {
    /**
     * Represents a successful response, with any data attached if any.
     * @param data The data to use to send out the response. The [T] generic
     *             must be marked with [Serializable][kotlinx.serialization.Serializable] or
     *             the server will not know how to serialize it to JSON.
     */
    @Schema(description = "Represents a successful response, with any data attached if any")
    public data class Ok<out T>(val data: T? = null): ApiResponse<T>(true)

    /**
     * Represents an unsuccessful response, with any errors that might've occurred during
     * the invocation of the request.
     *
     * @param errors A list of API errors that might've occurred when invoking the request.
     */
    @Schema(description = "Represents an unsuccessful response, with any errors that might've occurred during the invocation of the request")
    public data class Err(val errors: List<ApiError>): ApiResponse<Unit>(false)

    public companion object {
        /**
         * Sends out an empty response payload with only the success marker.
         */
        @JvmStatic
        public fun ok(): ApiResponse<Unit> = Ok(null)

        /**
         * Sends out a response that is represented as [T].
         * @param data The data payload to send.
         */
        @JvmStatic
        public fun <T> ok(data: T): ApiResponse<T> = Ok(data)

        /**
         * Sends out a response that represents multiple errors that might've happened during
         * a REST request invocation.
         *
         * @param errors A list of errors to prepend to the payload itself.
         */
        @JvmStatic
        public fun err(errors: List<ApiError>): ApiResponse<Unit> = Err(errors)

        /**
         * Sends out a response that represents a single error that might've happened during
         * a REST request invocation.
         *
         * @param error the [ApiError] object to use.
         */
        @JvmStatic
        public fun err(error: ApiError): ApiResponse<Unit> = err(listOf(error))

        /**
         * Sends out a response that still represents a single error, but the [code] and [message]
         * will construct a [ApiError] object for you to send.
         *
         * @param code The error code that gives a human-readable message in the documentation.
         * @param message The message of what happened.
         */
        @JvmStatic
        public fun err(code: String, message: String): ApiResponse<Unit> = err(ApiError(code, message))

        /**
         * Sends out a response that represents a single error, but a detailed blob is added
         * in the payload.
         *
         * @param code The error code that gives a human-readable message in the documentation.
         * @param message The message of what happened.
         * @param detail Detailed blob of what happened
         */
        @JvmStatic
        public fun err(
            code: String,
            message: String,
            detail: Any
        ): ApiResponse<Unit> = err(ApiError(code, message, detail))

        /**
         * Sends out a response from a generic [Throwable] object. It'll transform the
         * exception into an [ApiError] that the serializer can serialize.
         */
        @JvmStatic
        public fun <T: Throwable> err(throwable: T): ApiResponse<Unit> = err("INTERNAL_SERVER_ERROR", throwable.message ?: "(empty message)")
    }
}

private class KResponseSerializer<T>(private val kSerializer: KSerializer<T>) : KSerializer<ApiResponse<T>> {
    private val apiErrorSerializer = ListSerializer(ApiError.serializer())
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.ApiResponse") {
        element("success", Boolean.serializer().descriptor)
        element("data", kSerializer.descriptor, isOptional = true)
        element("errors", apiErrorSerializer.descriptor, isOptional = true)
    }

    @OptIn(ExperimentalSerializationApi::class)
    @Suppress("UNCHECKED_CAST")
    override fun deserialize(decoder: Decoder): ApiResponse<T> = decoder.decodeStructure(descriptor) {
        var res: ApiResponse<T>? = null
        loop@ while (true) {
            when (val index = decodeElementIndex(descriptor)) {
                DECODE_DONE -> break
                0 -> {
                    val success = decodeBooleanElement(descriptor, index)
                    res = if (success) {
                        ApiResponse.Ok(null)
                    } else {
                        ApiResponse.err(listOf()) as ApiResponse<T>
                    }
                }

                // index 1 represents "data", so we need to assert that success is true.
                1 -> {
                    check(res != null) { "Reached to index 1 without reaching to index 0" }

                    // Modify the state
                    if (res is ApiResponse.Ok) {
                        val data = decodeNullableSerializableElement(descriptor, index, kSerializer as KSerializer<T?>)
                        if (data != null) {
                            res = ApiResponse.ok(data)
                        }
                    }
                }

                // index 2 is the errors, so we need to assert if res is ApiResponse.Err
                2 -> {
                    check(res != null) { "Reached to index 2 without reaching to index 0" }

                    // Modify the state
                    if (res is ApiResponse.Err) {
                        res = ApiResponse.err(decodeSerializableElement(descriptor, index, apiErrorSerializer)) as ApiResponse<T>
                    }
                }

                else -> throw SerializationException("Unexpected index [$index]")
            }
        }

        check(res != null) { "Couldn't deserialize result due to `res` being null!" }
        res
    }

    override fun serialize(encoder: Encoder, value: ApiResponse<T>) {
        require(encoder is JsonEncoder) { "Json serialization is only supported in ApiResponse, not ${encoder::class}" }
        encoder.encodeStructure(descriptor) {
            encodeBooleanElement(descriptor, 0, value.success)

            when (value) {
                is ApiResponse.Ok -> {
                    if (value.data != null) {
                        encodeSerializableElement(descriptor, 1, kSerializer, value.data)
                    }
                }

                is ApiResponse.Err -> {
                    encodeSerializableElement(descriptor, 2, apiErrorSerializer, value.errors)
                }
            }
        }
    }
}
