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

package org.noelware.charted.configuration.dsl.tracing.apm

import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

/**
 * [read more here](https://www.elastic.co/guide/en/apm/agent/java/current/config-core.html#config-enable-instrumentations)
 */
@kotlinx.serialization.Serializable
enum class Instrumentation(val key: String) {
    ANNOTATIONS("annotations"),
    ANNOTATIONS_CAPTURE_SPAN("annotations-capture-span"),
    ANNOTATIONS_CAPTURE_TRANSACTION("annotations-capture-transaction"),
    ANNOTATIONS_TRACED("annotations-traced"),
    APACHE_HTTP_CLIENT("apache-httpclient"),
    AWS_SDK("aws-sdk"),
    CASSANDRA("cassandra"),
    ELASTICSEARCH_REST("elasticsearch-restclient"),
    EXCEPTION_HANDLER("exception-handler"),
    JAVA_EXECUTOR("executor"),
    EXECUTOR_COLLECTION("executor-collection"),
    GRPC("grpc"),
    LETTUCE("lettuce"),
    JDBC("jdbc"),
    OKHTTP("okhttp"),
    PROCESS("process"),
    SLF4J_ERROR("slf4j-error");

    companion object: KSerializer<Instrumentation> {
        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.apm.Instrumentation", PrimitiveKind.STRING)
        override fun deserialize(decoder: Decoder): Instrumentation {
            val key = decoder.decodeString()
            return values().find { it.key == key } ?: error("Unknown key [$key]")
        }

        override fun serialize(encoder: Encoder, value: Instrumentation) {
            encoder.encodeString(value.key)
        }
    }
}
