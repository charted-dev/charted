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

package org.noelware.charted.server.routing.openapi

import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.modules.openapi.kotlin.dsl.PathDslBuilder
import org.noelware.charted.modules.openapi.toPaths

/**
 * Represents a description about a [RestController][org.noelware.charted.server.routing.RestController].
 */
interface ResourceDescription {
    /**
     * HTTP path that belongs to this resource description
     */
    val path: String

    /**
     * Method to describe this resource that in turn, returns a [PathItem]
     * to be used by people who use our OpenAPI resources.
     */
    fun describe(): PathItem
}

fun describeResource(path: String, dsl: PathDslBuilder.() -> Unit): ResourceDescription = object: ResourceDescription {
    override val path: String = path
    override fun describe(): PathItem = toPaths(path, dsl)
}
