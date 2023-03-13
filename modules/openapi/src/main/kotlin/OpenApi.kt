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

package org.noelware.charted.modules.openapi

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.kotlinModule
import io.swagger.v3.core.converter.AnnotatedType
import io.swagger.v3.core.converter.ModelConverterContextImpl
import io.swagger.v3.core.converter.ModelConverters
import io.swagger.v3.core.util.Json
import io.swagger.v3.core.util.Yaml
import io.swagger.v3.oas.models.OpenAPI
import io.swagger.v3.oas.models.media.Schema
import org.noelware.charted.modules.openapi.jackson.OpenAPIJacksonModule
import org.noelware.charted.modules.openapi.kotlin.dsl.OpenAPIDsl
import org.noelware.charted.modules.openapi.kotlin.dsl.OpenAPIDslBuilder
import kotlin.reflect.KType
import kotlin.reflect.javaType

val modelConverterContext = ModelConverterContextImpl(ModelConverters.getInstance().converters)
private val jsonMapper: ObjectMapper = Json.mapper().apply {
    registerModules(kotlinModule(), OpenAPIJacksonModule)
}

private val yamlMapper: ObjectMapper = Yaml.mapper().apply {
    registerModules(kotlinModule(), OpenAPIJacksonModule)
}

/**
 * Builds and constructs a [OpenAPI] document.
 * @param builder DSL object to construct the [OpenAPI] builder.
 */
fun openApi(builder: OpenAPIDsl.() -> Unit): OpenAPI = OpenAPIDslBuilder().apply(builder).build()

/**
 * Transforms this [OpenAPI] document into a JSON object
 */
fun OpenAPI.toJson(pretty: Boolean = false): String = if (pretty) {
    jsonMapper.writerWithDefaultPrettyPrinter().writeValueAsString(this)
} else {
    jsonMapper.writeValueAsString(this)
}

/**
 * Transforms this [OpenAPI] document into YAML
 */
fun OpenAPI.toYaml(): String = yamlMapper.writeValueAsString(this)

@OptIn(ExperimentalStdlibApi::class)
internal fun computeSchema(type: KType): Schema<*> = modelConverterContext.resolve(
    AnnotatedType().type(jsonMapper.constructType(type.javaType)).resolveAsRef(true),
) ?: error("Unable to compute schema for type [$type]")
