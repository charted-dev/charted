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

package org.noelware.charted.modules.openapi.kotlin.dsl

import dev.floofy.utils.java.SetOnce
import io.swagger.v3.oas.models.media.MediaType
import io.swagger.v3.oas.models.media.Schema
import org.noelware.charted.annotations.ChartedDsl
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.extensions.setonce.getValue
import org.noelware.charted.common.extensions.setonce.setValue
import org.noelware.charted.modules.openapi.computeSchema
import kotlin.reflect.KType
import kotlin.reflect.typeOf

@ChartedDsl
interface MediaTypeDsl: SchemaSupport {
    /**
     * An example to give for the schema.
     */
    var example: Any?
}

/**
 * Uses the [typeOf] function to get the type of schema it is
 * represented as.
 *
 * @param example The example to give.
 */
inline fun <reified T> MediaTypeDsl.schema(example: T) {
    this.example = example
    schema(typeOf<T>())
}

class MediaTypeDslBuilder: MediaTypeDsl, Buildable<MediaType> {
    private val _example: SetOnce<Any> = SetOnce()
    private val _schema: SetOnce<Schema<*>> = SetOnce()

    override var example: Any? by _example
    override fun schema(type: KType) {
        _schema.value = computeSchema(type)
    }

    override fun build(): MediaType = MediaType().apply {
        _example.valueOrNull?.let { example(it) }
        _schema.valueOrNull?.let { schema(it) }
    }
}
