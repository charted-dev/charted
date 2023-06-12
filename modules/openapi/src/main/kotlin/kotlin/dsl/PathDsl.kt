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
import io.ktor.http.*
import io.swagger.v3.oas.models.ExternalDocumentation
import io.swagger.v3.oas.models.Operation
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.ChartedInfo
import org.noelware.charted.annotations.ChartedDsl
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.extensions.setonce.getValue
import org.noelware.charted.common.extensions.setonce.setValue

@ChartedDsl
interface PathDsl {
    /**
     * Description about this path
     */
    var description: String?

    /**
     * Adds a DELETE [OperationDsl] object into this [PathDsl].
     */
    fun delete(block: OperationDsl.() -> Unit = {})

    /**
     * Adds a PATCH [OperationDsl] object into this [PathDsl].
     */
    fun patch(block: OperationDsl.() -> Unit = {})

    /**
     * Adds a POST [OperationDsl] object into this [PathDsl].
     */
    fun post(block: OperationDsl.() -> Unit = {})

    /**
     * Adds a HEAD [OperationDsl] object into this [PathDsl].
     */
    fun head(block: OperationDsl.() -> Unit = {})

    /**
     * Adds a PUT [OperationDsl] object into this [PathDsl].
     */
    fun put(block: OperationDsl.() -> Unit = {})

    /**
     * Adds a GET [OperationDsl] object into this [PathDsl]. This will also
     * register a HEAD [OperationDsl] if this is used.
     */
    fun get(block: OperationDsl.() -> Unit = {})
}

class PathDslBuilder(private val path: String): PathDsl, Buildable<PathItem> {
    private val _description: SetOnce<String> = SetOnce()
    private val _methods: MutableMap<HttpMethod, Operation> = mutableMapOf()

    override var description: String? by _description

    private fun registerMethod(method: HttpMethod, block: OperationDsl.() -> Unit) {
        if (_methods.containsKey(method)) throw IllegalStateException("Path with HTTP method [$path '${method.value}'] is already registered")

        _methods[method] = OperationDslBuilder().apply(block).build().apply {
            externalDocs(
                ExternalDocumentation().apply {
                    url("https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/${method.value.uppercase()}-$path")
                },
            )
        }
    }

    override fun delete(block: OperationDsl.() -> Unit) = registerMethod(HttpMethod.Delete, block)
    override fun patch(block: OperationDsl.() -> Unit) = registerMethod(HttpMethod.Patch, block)
    override fun post(block: OperationDsl.() -> Unit) = registerMethod(HttpMethod.Post, block)
    override fun head(block: OperationDsl.() -> Unit) = registerMethod(HttpMethod.Head, block)
    override fun put(block: OperationDsl.() -> Unit) = registerMethod(HttpMethod.Put, block)
    override fun get(block: OperationDsl.() -> Unit) = registerMethod(HttpMethod.Get, block)
    override fun build(): PathItem = PathItem().apply {
        _description.valueOrNull?.let { description(it) }
        for ((method, operation) in _methods) {
            when (method) {
                HttpMethod.Delete -> delete(operation)
                HttpMethod.Patch -> patch(operation)
                HttpMethod.Post -> post(operation)
                HttpMethod.Head -> head(operation)
                HttpMethod.Put -> put(operation)
                HttpMethod.Get -> get(operation)
            }
        }
    }
}
