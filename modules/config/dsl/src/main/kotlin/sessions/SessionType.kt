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

package org.noelware.charted.configuration.kotlin.dsl.sessions

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

@Serializable(with = SessionType.Companion.Serializer::class)
sealed class SessionType(val name: String) {
    // also used to trick the serialization compiler; this is never used
    @Suppress("unused")
    constructor(): this("why does this exist")

    @Serializable
    object Local: SessionType("local")

    @Serializable
    object LDAP: SessionType("ldap")

    companion object {
        val TYPES: List<SessionType> = listOf(Local, LDAP)

        object Serializer: KSerializer<SessionType> {
            override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.sessions.SessionType") {
                element("type", String.serializer().descriptor)
                element("ldap", LDAP.serializer().descriptor, isOptional = true)
            }

            override fun deserialize(decoder: Decoder): SessionType {
                val type = decoder.decodeString()
                return TYPES.find { it.name == type } ?: throw SerializationException("Unknown session type [$type]")
            }

            override fun serialize(encoder: Encoder, value: SessionType) {
                encoder.encodeString(value.name)
            }
        }
    }
}
