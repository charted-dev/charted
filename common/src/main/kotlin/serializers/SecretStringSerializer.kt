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

package org.noelware.charted.serializers

import kotlinx.serialization.KSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

private val environmentVariableRegex: Regex = """[$]\{([\w.]+)(:-\w+)?}""".toRegex()

public object SecretStringSerializer: KSerializer<String> {
    override val descriptor: SerialDescriptor = String.serializer().descriptor
    override fun deserialize(decoder: Decoder): String {
        val decoded = decoder.decodeString()
        if (decoded.matches(environmentVariableRegex)) {
            return environmentVariableRegex.replace(decoded, transform = {
                // Removes the `:-` from the matched result.
                System.getenv(it.groups[1]!!.value) ?: (it.groups[2]?.value?.substring(2) ?: "")
            })
        }

        return decoded
    }

    override fun serialize(encoder: Encoder, value: String): Unit = String.serializer().serialize(encoder, value)
}
