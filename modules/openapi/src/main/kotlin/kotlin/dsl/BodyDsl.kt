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

import io.ktor.http.*
import io.swagger.v3.oas.models.media.MediaType
import org.noelware.charted.annotations.ChartedDsl

@ChartedDsl
interface BodyDsl {
    /**
     * Adds a media type to this content
     */
    fun contentType(type: ContentType, block: MediaTypeDsl.() -> Unit = {})
}

/**
 * Alias for [contentType(ContentType.Application.Json) {}][BodyDsl.contentType] for easy consumption.
 * @param block [MediaTypeDsl] object to attach to this [BodyDsl].
 */
fun BodyDsl.json(block: MediaTypeDsl.() -> Unit = {}) {
    contentType(ContentType.Application.Json, block)
}

/**
 * Alias for [contentType(ContentType.Text.Plain) {}][BodyDsl.contentType] for easy consumption.
 * @param block [MediaTypeDsl] object to attach to this [BodyDsl].
 */
fun BodyDsl.text(block: MediaTypeDsl.() -> Unit = {}) {
    contentType(ContentType.Text.Plain, block)
}

fun BodyDsl.yaml(block: MediaTypeDsl.() -> Unit = {}) {
    contentType(ContentType.parse("text/yaml; charset=utf-8"), block)
}

open class BodyBuilder: BodyDsl {
    @Suppress("PropertyName")
    internal val _contentTypes: MutableMap<String, MediaType> = mutableMapOf()

    override fun contentType(type: ContentType, block: MediaTypeDsl.() -> Unit) {
        _contentTypes[type.withoutParameters().toString()] = MediaTypeDslBuilder().apply(block).build()
    }
}
