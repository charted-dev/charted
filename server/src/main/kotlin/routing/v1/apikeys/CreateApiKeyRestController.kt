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

package org.noelware.charted.server.routing.v1.apikeys

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.routing.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.modules.postgresql.controllers.apikeys.ApiKeysDatabaseController
import org.noelware.charted.server.routing.RestController

class CreateApiKeyRestController(private val controller: ApiKeysDatabaseController): RestController("/apikeys", HttpMethod.Put) {
    override fun Route.init() {
        TODO("Not yet implemented")
    }

    override suspend fun call(call: ApplicationCall) {
        TODO("Not yet implemented")
    }

    override fun toPathDsl(): PathItem {
        TODO("Not yet implemented")
    }
}
