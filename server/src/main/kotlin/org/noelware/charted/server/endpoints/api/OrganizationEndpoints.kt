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

@file:Suppress("UNUSED")

package org.noelware.charted.server.endpoints.api

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.common.data.responses.Response
import org.noelware.ktor.endpoints.*

@kotlinx.serialization.Serializable
data class OrganizationsResponse(
    val message: String,
    val docs: String
)

class OrganizationEndpoints: AbstractEndpoint("/organizations") {
    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            Response.ok(
                OrganizationsResponse(
                    message = "Welcome to the Organizations API!",
                    docs = "https://charts.noelware.org/docs/server/api/organizations"
                )
            )
        )
    }

    @Put
    suspend fun create(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Patch("/{id}")
    suspend fun patch(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}")
    suspend fun delete(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}/members")
    suspend fun members(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Put("/{id}/members")
    suspend fun inviteMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Patch("/{id}/members")
    suspend fun patchMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}/members")
    suspend fun kickMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}/members/{memberId}")
    suspend fun memberById(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }
}
