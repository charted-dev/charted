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

package org.noelware.charted.server.plugins.sessions

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.util.*
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.server.extensions.currentUser

object IsAdminGuard: BaseRouteScopedPlugin<Unit, IsAdminGuard> {
    override val key: AttributeKey<IsAdminGuard> = AttributeKey("IsAdmin Guard")
    override fun install(pipeline: ApplicationCallPipeline, configure: Unit.() -> Unit): IsAdminGuard {
        pipeline.intercept(ApplicationCallPipeline.Plugins) {
            if (call.currentUser == null || call.currentUser?.admin == false) {
                return@intercept call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "UNAUTHORIZED",
                        "You must have administrator privileges to access this route",
                    ),
                )
            }

            proceed()
        }

        return this
    }
}
