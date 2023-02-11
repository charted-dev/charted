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

package org.noelware.charted.configuration.kotlin.dsl.storage.azure

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.descriptors.element
import kotlinx.serialization.encoding.*
import kotlinx.serialization.encoding.CompositeDecoder.Companion.DECODE_DONE
import org.noelware.charted.serializers.SecretStringSerializer
import org.noelware.remi.support.azure.authentication.AzureAuthType
import org.noelware.remi.support.azure.authentication.AzureConnectionAuth
import org.noelware.remi.support.azure.authentication.AzureConnectionStringAuth
import org.noelware.remi.support.azure.authentication.AzureSasTokenAuth

public fun AzureAuthenticationHost.toAzureConnectionAuth(): AzureConnectionAuth = when (this) {
    is AzureAuthenticationHost.ConnectionStringAuthenticationHost -> AzureConnectionStringAuth(connectionString)
    is AzureAuthenticationHost.SasTokenAuthenticationHost -> AzureSasTokenAuth(sasToken)
}

/**
 * Represents the authentication host to use when dealing with Azure Blob Storage authentication
 */
@Serializable(with = AzureAuthenticationHost.Serializer::class)
public sealed class AzureAuthenticationHost(public val authType: AzureAuthType) {
    @Suppress("unused") // magically force the compiler to use `SAS_TOKEN` as the default, but this is never used for (de)serializing
    private constructor() : this(AzureAuthType.SAS_TOKEN)

    @Serializable
    public class ConnectionStringAuthenticationHost(
        @Serializable(with = SecretStringSerializer::class)
        @SerialName("connection_string")
        public val connectionString: String
    ) : AzureAuthenticationHost(AzureAuthType.CONNECTION_STRING)

    @Serializable
    public class SasTokenAuthenticationHost(
        @Serializable(with = SecretStringSerializer::class)
        @SerialName("connection_string")
        public val sasToken: String
    ) : AzureAuthenticationHost(AzureAuthType.SAS_TOKEN)

    internal object Serializer : KSerializer<AzureAuthenticationHost> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.AzureAuthenticationHost") {
            element("type", AzureAuthTypeSerializer.descriptor)
            element<String>("connection_string", isOptional = true)
            element<String>("sas_token", isOptional = true)
        }

        override fun serialize(encoder: Encoder, value: AzureAuthenticationHost) = encoder.encodeStructure(descriptor) {
            encodeSerializableElement(descriptor, 0, AzureAuthTypeSerializer, value.authType)
            when (value) {
                is ConnectionStringAuthenticationHost -> {
                    encodeStringElement(descriptor, 1, value.connectionString)
                }

                is SasTokenAuthenticationHost -> {
                    encodeStringElement(descriptor, 2, value.sasToken)
                }
            }
        }

        override fun deserialize(decoder: Decoder): AzureAuthenticationHost = decoder.decodeStructure(descriptor) {
            var authType: AzureAuthType? = null
            var authHost: AzureAuthenticationHost? = null

            loop@ while (true) {
                when (val index = decodeElementIndex(descriptor)) {
                    DECODE_DONE -> break@loop
                    0 -> authType = decodeSerializableElement(descriptor, index, AzureAuthTypeSerializer)
                    1 -> {
                        assert(authType != null && authType == AzureAuthType.CONNECTION_STRING) { "Authentication type was not CONNECTION_STRING" }
                        authHost = ConnectionStringAuthenticationHost(decodeStringElement(descriptor, index))
                    }

                    2 -> {
                        assert(authType != null && authType == AzureAuthType.SAS_TOKEN) { "Authentication type was not SAS_TOKEN" }
                        authHost = SasTokenAuthenticationHost(decodeStringElement(descriptor, index))
                    }

                    else -> throw SerializationException("Unexpected index $index")
                }
            }

            assert(authHost != null) { "Was unable to determine what Azure Auth Host to use" }
            authHost!!
        }
    }
}
