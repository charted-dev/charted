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

package org.noelware.charted.modules.docker.registry.types

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

val REGISTERED_ALGORITHMS = listOf("sha512", "sha256")

@Serializable(with = DigestSerializer::class)
data class Digest(
    val algorithm: String,
    val encoded: String
) {
    override fun toString(): String = "$algorithm:$encoded"
    companion object {
        fun fromString(raw: String): Digest {
            if (!(raw matches "[a-z0-9]+:[a-zA-Z0-9=_-]*".toRegex())) {
                throw IllegalStateException("Invalid digest '$raw'")
            }

            val (algo, enc) = raw.split(':')
            if (!REGISTERED_ALGORITHMS.contains(algo)) {
                throw IllegalStateException("Algorithm '$algo' is not a registered algorithm (${REGISTERED_ALGORITHMS.joinToString(", ")}]")
            }

            return Digest(algo, enc)
        }
    }
}

object DigestSerializer: KSerializer<Digest> {
    override val descriptor: SerialDescriptor = String.serializer().descriptor

    override fun deserialize(decoder: Decoder): Digest = Digest.fromString(decoder.decodeString())
    override fun serialize(encoder: Encoder, value: Digest) {
        encoder.encodeString("${value.algorithm}:${value.encoded}")
    }
}
