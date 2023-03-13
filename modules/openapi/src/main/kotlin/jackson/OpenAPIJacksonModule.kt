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

package org.noelware.charted.modules.openapi.jackson

import com.fasterxml.jackson.core.JsonGenerator
import com.fasterxml.jackson.databind.JsonSerializer
import com.fasterxml.jackson.databind.SerializerProvider
import com.fasterxml.jackson.databind.module.SimpleModule
import com.fasterxml.jackson.module.kotlin.addSerializer
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDateTime

object OpenAPIJacksonModule: SimpleModule() {
    init {
        addSerializer(
            Instant::class,
            object: JsonSerializer<Instant>() {
                override fun serialize(value: Instant, gen: JsonGenerator, serializers: SerializerProvider) {
                    gen.writeString(value.toString())
                }
            },
        )

        addSerializer(
            LocalDateTime::class.java,
            object: JsonSerializer<LocalDateTime>() {
                override fun serialize(value: LocalDateTime, gen: JsonGenerator, serializers: SerializerProvider) {
                    gen.writeString(value.toString())
                }
            },
        )
    }
}
