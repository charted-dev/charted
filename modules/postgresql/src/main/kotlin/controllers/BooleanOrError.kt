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

package org.noelware.charted.modules.postgresql.controllers

import io.sentry.Sentry
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import org.noelware.charted.common.extensions.sentry.ifSentryEnabled
import org.noelware.charted.common.types.responses.ApiError

/**
 * Represents a discriminated union that can be represented as a [Boolean] or [ApiError], useful
 * for patching entities to determine if it was successful or not.
 */
@Serializable(with = BooleanOrError.Companion::class)
class BooleanOrError(private val value: Any) {
    init {
        check(value is Boolean || value is ApiError) { "[${value::class}] was not Boolean or ApiError" }
    }

    /**
     * Returns a [Boolean] if the [value] specified was a Boolean, otherwise, `null`
     * will be returned, which will be represented as an [ApiError].
     */
    val booleanOrNull: Boolean? get() = value as? Boolean

    /**
     * Returns an [ApiError] if the [value] specified was an [ApiError], otherwise, `null`
     * will be returned, which will always be a [Boolean].
     */
    val errorOrNull: ApiError? get() = value as? ApiError
    internal companion object: KSerializer<BooleanOrError> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.BooleanOrError")
        override fun deserialize(decoder: Decoder): BooleanOrError = try {
            val bool = decoder.decodeBoolean()
            BooleanOrError(bool)
        } catch (e: SerializationException) {
            val apiError = decoder.decodeSerializableValue(ApiError.serializer())
            BooleanOrError(apiError)
        } catch (e: Throwable) {
            ifSentryEnabled { Sentry.captureException(e) }
            throw e
        }

        override fun serialize(encoder: Encoder, value: BooleanOrError) {
            if (value.booleanOrNull != null) {
                encoder.encodeBoolean(value.booleanOrNull!!)
            } else {
                encoder.encodeSerializableValue(ApiError.serializer(), value.errorOrNull!!)
            }
        }
    }
}
