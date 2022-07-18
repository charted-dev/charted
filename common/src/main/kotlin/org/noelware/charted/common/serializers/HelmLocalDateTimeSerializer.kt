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

package org.noelware.charted.common.serializers

import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import java.util.*

/**
 * This is a custom serializer to output it as Go's `RFC3339` instead of Java's
 * interpretation. This is required due to an error with Helm:
 *
 * ```sh
 * $ helm repo add http://localhost:<port>/users/<userid>
 * # Error: looks like "http://127.0.0.1:8989/users/24932163002568704" is not a valid chart repository or cannot be reached:
 * # error unmarshaling JSON: while decoding JSON:
 * #      parsing time "\"2022-07-07T12:11:31.078753507\"" as "\"2006-01-02T15:04:05Z07:00\"": cannot parse "\"" as "Z07:00"
 * ```
 */
object HelmLocalDateTimeSerializer: KSerializer<LocalDateTime> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.HelmLocalDateTime", PrimitiveKind.STRING)
    override fun serialize(encoder: Encoder, value: LocalDateTime) {
        encoder.encodeString(value.toString() + "Z")
    }

    override fun deserialize(decoder: Decoder): LocalDateTime {
        val decoded = decoder.decodeString().replace("Z", "")
        return LocalDateTime.parse(decoded)
    }
}
