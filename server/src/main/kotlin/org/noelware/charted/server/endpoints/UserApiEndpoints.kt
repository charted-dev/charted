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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.server.endpoints

import io.ktor.server.application.*
import org.noelware.ktor.endpoints.*

class UserApiEndpoints: AbstractEndpoint("/users") {
    @Get
    suspend fun main(call: ApplicationCall) {}

    @Get("/@me")
    suspend fun me(call: ApplicationCall) {}

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {}

    @Patch("/@me")
    suspend fun updateMe(call: ApplicationCall) {}

    @Delete
    suspend fun deleteCurrent(call: ApplicationCall) {}

    @Post("/login")
    suspend fun login(call: ApplicationCall) {}

    @Post("/@me/refresh_token")
    suspend fun refreshSessionToken(call: ApplicationCall) {}
}
