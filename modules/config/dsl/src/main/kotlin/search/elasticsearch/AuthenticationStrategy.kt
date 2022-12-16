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

package org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.CompositeDecoder.Companion.DECODE_DONE
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.encoding.decodeStructure
import kotlinx.serialization.encoding.encodeStructure
import org.noelware.charted.serializers.SecretStringSerializer

/**
 * Represents the authentication strategy type to use.
 *
 * - **None** refers to no authentication is needed to connect.
 * - **Cloud** refers to using the [Elasticsearch Service Cloud](https://www.elastic.co/cloud) credentials to connect to an
 *   Elasticsearch Service Cloud cluster.
 * - **Basic** refers to using basic username and password authentication, which is not *ideally* recommended, but
 *   it is something.
 * - **ApiKey** refers to using Elasticsearch [API Keys](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html)
 *   for authentication. This is ideally recommended since it can restrict features that **charted-server** doesn't need.
 *
 * You can read up on how to use [Elasticsearch with charted-server](https://charts.noelware.org/docs/server/self-hosting/search/elasticsearch) on how
 * to set up an ideal environment for **charted-server** and Elasticsearch.
 */
@Serializable
enum class AuthStrategyType {
    /** No authentication is required on the server level. */
    @SerialName("none")
    None,

    /**
     * Uses the [Elasticsearch Service Cloud](https://www.elastic.co/cloud) credentials to connect to an
     * Elasticsearch Service Cloud cluster.
     */
    @SerialName("cloud")
    Cloud,

    /**
     * Uses basic username and password authentication, which is not *ideally* recommended, but
     * it is something.
     */
    @SerialName("basic")
    Basic,

    /**
     * Uses Elasticsearch [API Keys](https://www.elastic.co/guide/en/elasticsearch/reference/current/security-api-create-api-key.html)
     * for authentication. This is ideally recommended since it can restrict features that **charted-server** doesn't need.
     */
    @SerialName("api_key")
    ApiKey,

    /**
     * Refers to an unknown authentication strategy that charted-server can't use.
     */
    Unknown
}

@Serializable(with = AuthenticationStrategy.Companion::class)
sealed class AuthenticationStrategy(val type: AuthStrategyType) {
    // tricks the serialization compiler (for now), but we are using our own serializer for this
    @Suppress("unused")
    constructor(): this(AuthStrategyType.Unknown)

    @Serializable
    object None: AuthenticationStrategy(AuthStrategyType.None)

    @Serializable
    class Cloud(
        @Serializable(with = SecretStringSerializer::class)
        @SerialName("cloud_id")
        val id: String
    ): AuthenticationStrategy(AuthStrategyType.Cloud)

    @Serializable
    class Basic(
        @Serializable(with = SecretStringSerializer::class)
        val username: String,

        @Serializable(with = SecretStringSerializer::class)
        val password: String
    ): AuthenticationStrategy(AuthStrategyType.Basic)

    @Serializable
    class ApiKey(
        @Serializable(with = SecretStringSerializer::class)
        @SerialName("api_key")
        val key: String
    ): AuthenticationStrategy(AuthStrategyType.ApiKey)

    companion object: KSerializer<AuthenticationStrategy> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.elasticsearch.AuthStrategy") {
            element("type", AuthStrategyType.serializer().descriptor)
            element("cloud_id", SecretStringSerializer.descriptor, isOptional = true)
            element("username", SecretStringSerializer.descriptor, isOptional = true)
            element("password", SecretStringSerializer.descriptor, isOptional = true)
            element("api_key", SecretStringSerializer.descriptor, isOptional = true)
        }

        override fun deserialize(decoder: Decoder): AuthenticationStrategy = decoder.decodeStructure(descriptor) {
            var authStrategy: AuthenticationStrategy? = null
            var username: String? = null
            lateinit var type: AuthStrategyType

            loop@ while (true) {
                when (val index = decodeElementIndex(descriptor)) {
                    DECODE_DONE -> break@loop
                    0 -> type = decodeSerializableElement(descriptor, 0, AuthStrategyType.serializer())
                    1 -> {
                        if (type == AuthStrategyType.Cloud) {
                            authStrategy = Cloud(decodeSerializableElement(descriptor, index, SecretStringSerializer))
                        }
                    }

                    2 -> {
                        assert(type != AuthStrategyType.Basic) {
                            "Received authentication type '$type' when expecting 'basic'"
                        }

                        username = decodeSerializableElement(descriptor, index, SecretStringSerializer)
                    }

                    3 -> {
                        if (authStrategy == null) {
                            authStrategy = Basic(
                                username!!,
                                decodeSerializableElement(descriptor, index, SecretStringSerializer)
                            )
                        }
                    }

                    4 -> {
                        assert(type != AuthStrategyType.ApiKey) {
                            "Received authentication type '$type' when expecting 'apikey'"
                        }

                        if (authStrategy == null) {
                            authStrategy = ApiKey(decodeSerializableElement(descriptor, index, SecretStringSerializer))
                        }
                    }
                }
            }

            assert(authStrategy != null) { "Unable to determine which authentication strategy to use" }
            authStrategy!!
        }

        /*
        decoder.decodeStructure(descriptor) {
            when (decodeSerializableElement(descriptor, 0, AuthStrategyType.serializer())) {
                AuthStrategyType.Unknown -> throw SerializationException("Reached unknown authentication strategy type")
                AuthStrategyType.None -> None
                AuthStrategyType.Cloud -> {
                    val id = decodeSerializableElement(descriptor, 1, SecretStringSerializer)
                    Cloud(id)
                }

                AuthStrategyType.Basic -> {
                    val username = decodeSerializableElement(descriptor, 2, SecretStringSerializer)
                    val password = decodeSerializableElement(descriptor, 3, SecretStringSerializer)

                    Basic(username, password)
                }

                AuthStrategyType.ApiKey -> {
                    val key = decodeSerializableElement(descriptor, 4, SecretStringSerializer)
                    ApiKey(key)
                }
            }
        }
         */

        override fun serialize(encoder: Encoder, value: AuthenticationStrategy) = encoder.encodeStructure(descriptor) {
            encodeSerializableElement(descriptor, 0, AuthStrategyType.serializer(), value.type)
            when (value) {
                is None -> {}
                is Cloud -> {
                    encodeSerializableElement(descriptor, 1, SecretStringSerializer, value.id)
                }

                is Basic -> {
                    encodeSerializableElement(descriptor, 2, SecretStringSerializer, value.username)
                    encodeSerializableElement(descriptor, 3, SecretStringSerializer, value.password)
                }

                is ApiKey -> {
                    encodeSerializableElement(descriptor, 4, SecretStringSerializer, value.key)
                }
            }
        }
    }
}
