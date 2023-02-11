/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

@file:Suppress("unused")

package org.noelware.charted.server.endpoints.v1.api.organizations

import guru.zoroark.tegral.openapi.dsl.RootDsl
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.server.endpoints.v1.api.MainOrganizationsResponse
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.Get

class OrganizationEndpoints: AbstractOrganizationEndpoints("/organizations") {
    @Get
    suspend fun main(call: ApplicationCall): Unit = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainOrganizationsResponse()))

    @Get("/{idOrName}")
    suspend fun getByIdOrName(call: ApplicationCall) {}

    companion object {
        fun RootDsl.toOpenAPI() {
        }
    }
}
