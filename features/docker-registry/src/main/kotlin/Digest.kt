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

package org.noelware.charted.features.oci.registry

import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import java.util.Objects

private val digestRegex: Regex = """([A-Za-z0-9_+.-]+):([A-Fa-f0-9]+)""".toRegex()

// https://docs.docker.com/registry/spec/api/#content-digests
/**
 * A digest is a serialized hash result, consisting of an algorithm and hex portion. The algorithm identifies
 * the methodology used to calculate the digest. The hex portion is the hex-encoded
 * result of the hash.
 */
@Schema(
    description = "A digest is a serialized hash result, consisting of an algorithm and hex portion. The algorithm identifies\n" +
        "the methodology used to calculate the digest. The hex portion is the hex-encoded\n" +
        "result of the hash.",

    implementation = String::class,
    pattern = "([A-Za-z0-9_+.-]+):([A-Fa-f0-9]+)",
)
@Serializable(with = Digest.Serializer::class)
data class Digest(private val data: String) {
    /**
     * The algorithm used for this [Digest].
     */
    val algorithm: String

    /**
     * The hex value itself of this [Digest].
     */
    val hex: String

    init {
        if (!(data matches digestRegex)) {
            throw DockerRegistryException(DockerRegistryException.Code.InvalidDigest)
        }

        val (alg, h) = data.split(':')
        algorithm = alg
        hex = h
    }

    override fun toString(): String = data
    override fun hashCode(): Int = Objects.hash(algorithm, hex)
    override fun equals(other: Any?): Boolean {
        if (other == null) return false
        if (other !is Digest) return false

        return algorithm == other.algorithm && hex == other.hex
    }

    internal class Serializer: KSerializer<Digest> {
        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.oci.Digest", PrimitiveKind.STRING)
        override fun deserialize(decoder: Decoder): Digest = Digest(decoder.decodeString())
        override fun serialize(encoder: Encoder, value: Digest) {
            encoder.encodeString(value.toString())
        }
    }
}
