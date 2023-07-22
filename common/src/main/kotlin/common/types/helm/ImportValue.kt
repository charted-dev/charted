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

package org.noelware.charted.common.types.helm

import com.fasterxml.jackson.annotation.JsonIgnore
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

/**
 * ImportValues holds the mapping of source values to parent key to be imported.
 * Each item can be a string or pair of child/parent sublist items.
 */
@Serializable
public data class ImportValue(
    /** The source key of the values to be imported */
    val child: String,

    /** The destination path in the parent chart's values */
    val parent: String
)

@Serializable(with = StringOrImportValue.Companion::class)
@Schema(oneOf = [String::class, ImportValue::class])
public class StringOrImportValue(@JsonIgnore private val value: Any) {
    init {
        require(value is String || value is ImportValue) { "Can't resolve a `import-value` from anything other than a String or ImportValue" }
    }

    @get:JsonIgnore
    public val stringOrNull: String?
        get() = value as? String

    @get:JsonIgnore
    public val importValueOrNull: ImportValue?
        get() = value as? ImportValue

    internal companion object: KSerializer<StringOrImportValue> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.StringOrImportValue")
        override fun deserialize(decoder: Decoder): StringOrImportValue = try {
            val string = decoder.decodeString()
            StringOrImportValue(string)
        } catch (e: SerializationException) {
            val importValue = decoder.decodeSerializableValue(ImportValue.serializer())
            StringOrImportValue(importValue)
        } catch (e: Exception) {
            throw e
        }

        override fun serialize(encoder: Encoder, value: StringOrImportValue) {
            if (value.stringOrNull != null) {
                encoder.encodeString(value.stringOrNull!!)
            } else {
                encoder.encodeSerializableValue(ImportValue.serializer(), value.importValueOrNull!!)
            }
        }
    }
}