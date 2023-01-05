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

package org.noelware.charted.configuration.kotlin.dsl.storage.azure

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.Serializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import org.noelware.remi.support.azure.authentication.AzureAuthType

@OptIn(ExperimentalSerializationApi::class)
@Serializer(forClass = AzureAuthType::class)
public object AzureAuthTypeSerializer: KSerializer<AzureAuthType> {
    override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.AzureAuthType", PrimitiveKind.STRING)

    override fun deserialize(decoder: Decoder): AzureAuthType = when (val value = decoder.decodeString()) {
        "connection-string", "connection string" -> AzureAuthType.CONNECTION_STRING
        "sas-token", "sas token" -> AzureAuthType.SAS_TOKEN
        else -> throw SerializationException("Unknown auth type [$value]")
    }

    override fun serialize(encoder: Encoder, value: AzureAuthType) {
        encoder.encodeString(
            when (value) {
                AzureAuthType.CONNECTION_STRING -> "connection-string"
                AzureAuthType.SAS_TOKEN -> "sas-token"
            },
        )
    }
}
