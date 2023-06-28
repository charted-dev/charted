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

import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.modules.openapi.kotlin.dsl.PathDslBuilder

/**
 * Interface to implement to register the base class with
 * a [PathItem] that is acceptable by the [openApi] function.
 */
@Deprecated("Since v0.1-beta: Moved to the new [ResourceDescription] API")
interface ToPaths {
    /**
     * Transforms into a [PathItem]. Recommended to use the [toPaths] DSL
     * object to achieve this.
     */
    fun toPathDsl(): PathItem
}

@Deprecated("Since v0.1-beta: Moved to the new [ResourceDescription] API")
fun toPaths(path: String, block: PathDslBuilder.() -> Unit): PathItem = PathDslBuilder(path).apply(block).build()
