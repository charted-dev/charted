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

package org.noelware.charted.server.routing

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.routing.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.modules.openapi.ToPaths
import org.noelware.charted.server.routing.openapi.ResourceDescription
import kotlin.reflect.full.companionObjectInstance

/**
 * Represents a REST controller that is available to be used through-out the application.
 * @param path The path to call this controller
 * @param method list of valid [http methods][HttpMethod] to use.
 */
abstract class RestController(val path: String, internal val method: HttpMethod = HttpMethod.Get): ToPaths {
    /**
     * [APIVersion] this [RestController] is using. It will use the default one.
     */
    open val apiVersion: APIVersion
        get() = APIVersion.default()

    /**
     * Initializes this [RestController] when routing is being configured.
     */
    open fun Route.init() {}
    abstract suspend fun call(call: ApplicationCall)

    override fun toPathDsl(): PathItem = TODO()

    /**
     * Returns the [ResourceDescription] that was attached to this [RestController] by its
     * companion object, or `null` if there is no companion object, or it doesn't implement
     * the [ResourceDescription] interface.
     */
    fun describeResource(): ResourceDescription? = this::class.companionObjectInstance as? ResourceDescription
}
