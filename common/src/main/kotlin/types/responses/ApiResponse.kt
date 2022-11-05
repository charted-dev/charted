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

package org.noelware.charted.types.responses

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.encoding.encodeStructure
import kotlinx.serialization.json.JsonEncoder

/**
 * Represents a generic API response object.
 */
@Serializable(with = KResponseSerializer::class)
sealed class ApiResponse<out T>(val success: Boolean) {
    /**
     * Represents a successful response, with any data attached if any.
     * @param data The data to use to send out the response. The [T] generic
     *             must be marked with [Serializable][kotlinx.serialization.Serializable] or
     *             the server will not know how to serialize it to JSON.
     */
    data class Ok<out T>(val data: T? = null): ApiResponse<T>(true)

    /**
     * Represents an unsuccessful response, with any errors that might've occurred during
     * the invocation of the request.
     *
     * @param errors A list of API errors that might've occurred when invoking the request.
     */
    data class Err(val errors: List<ApiError>): ApiResponse<Unit>(false)

    companion object {
        /**
         * Sends out an empty response payload with only the success marker.
         */
        @JvmStatic
        fun ok(): ApiResponse<Nothing> = Ok(null)

        /**
         * Sends out a response that is represented as [T].
         * @param data The data payload to send.
         */
        fun <T> ok(data: T): ApiResponse<T> = Ok(data)

        /**
         * Sends out a response that represents multiple errors that might've happened during
         * a REST request invocation.
         *
         * @param errors A list of errors to prepend to the payload itself.
         */
        fun err(errors: List<ApiError>): ApiResponse<Unit> = Err(errors)

        /**
         * Sends out a response that represents a single error that might've happened during
         * a REST request invocation.
         *
         * @param error the [APIError] object to use.
         */
        fun err(error: ApiError): ApiResponse<Unit> = err(listOf(error))

        /**
         * Sends out a response that still represents a single error, but the [code] and [message]
         * will construct a [APIError] object for you to send.
         *
         * @param code The error code that gives a human-readable message in the documentation.
         * @param message The message of what happened.
         */
        fun err(code: String, message: String): ApiResponse<Unit> = err(ApiError(code, message))

        fun err(
            code: String,
            message: String,
            detail: Any
        ): ApiResponse<Unit> = err(ApiError(code, message, detail))

        /**
         * Sends out a response from a generic [Throwable] object. It'll transform the
         * exception into an [APIError] that the serializer can serialize.
         */
        fun <T: Throwable> err(throwable: T): ApiResponse<Unit> = err("INTERNAL_SERVER_ERROR", throwable.message ?: "(empty message)")
    }
}

private class KResponseSerializer<T>(private val kSerializer: KSerializer<T>): KSerializer<ApiResponse<T>> {
    private val API_ERROR_SERIALIZER = ListSerializer(ApiError.serializer())
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.ApiResponse") {
        element("success", Boolean.serializer().descriptor)
        element("data", kSerializer.descriptor, isOptional = true)
        element("errors", API_ERROR_SERIALIZER.descriptor, isOptional = true)
    }

    override fun deserialize(decoder: Decoder): ApiResponse<T> {
        throw IllegalAccessException("Deserialization is not supported in KResponseSerializer.")
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
                    encodeSerializableElement(descriptor, 2, API_ERROR_SERIALIZER, value.errors)
                }
            }
        }
    }
}
