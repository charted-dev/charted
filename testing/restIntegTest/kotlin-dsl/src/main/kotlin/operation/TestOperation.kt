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

package org.noelware.charted.testing.restIntegTest.kotlin.dsl.operation

import io.ktor.http.*

public data class Operation<T: Any>(
    private val headersMap: MutableMap<String, String> = mutableMapOf(),
    var body: T? = null
) {
    public val headers: Map<String, String>
        get() = headersMap

    public fun header(name: String, value: String): Operation<T> {
        headersMap[name] = value
        return this
    }
}

public data class TestOperation internal constructor(
    private val delete: MutableMap<String, (Operation<*>) -> Unit> = mutableMapOf(),
    private val patch: MutableMap<String, (Operation<*>) -> Unit> = mutableMapOf(),
    private val post: MutableMap<String, (Operation<*>) -> Unit> = mutableMapOf(),
    private val head: MutableMap<String, (Operation<*>) -> Unit> = mutableMapOf(),
    private val `get`: MutableMap<String, (Operation<*>) -> Unit> = mutableMapOf(),
    private val put: MutableMap<String, (Operation<*>) -> Unit> = mutableMapOf(),
    val path: String
) {
    public constructor(path: String): this(
        mutableMapOf(),
        mutableMapOf(),
        mutableMapOf(),
        mutableMapOf(),
        mutableMapOf(),
        mutableMapOf(),
        path,
    )

    public fun addOperation(path: String, method: HttpMethod, block: (Operation<*>) -> Unit): Unit = when (method) {
        HttpMethod.Delete -> delete[path] = block
        HttpMethod.Patch -> patch[path] = block
        HttpMethod.Post -> post[path] = block
        HttpMethod.Head -> head[path] = block
        HttpMethod.Put -> put[path] = block
        HttpMethod.Get -> `get`[path] = block
        else -> {}
    }
}
