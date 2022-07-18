/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

package org.noelware.charted.server.endpoints.api

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.ktor.endpoints.*

class OrganizationEndpoints: AbstractEndpoint("/organizations") {
    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("message", "Welcome to the Organizations API!")
                        put("docs", "https://charts.noelware.org/docs/api/organizations")
                    }
                )
            }
        )
    }

    @Put
    suspend fun create(call: ApplicationCall) {}

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {}

    @Patch("/{id}")
    suspend fun patch(call: ApplicationCall) {}

    @Delete("/{id}")
    suspend fun delete(call: ApplicationCall) {}

    @Get("/{id}/members")
    suspend fun members(call: ApplicationCall) {}

    @Put("/{id}/members")
    suspend fun inviteMember(call: ApplicationCall) {}

    @Patch("/{id}/members")
    suspend fun patchMember(call: ApplicationCall) {}

    @Delete("/{id}/members")
    suspend fun kickMember(call: ApplicationCall) {}

    @Get("/{id}/members/{memberId}")
    suspend fun memberById(call: ApplicationCall) {}
}
