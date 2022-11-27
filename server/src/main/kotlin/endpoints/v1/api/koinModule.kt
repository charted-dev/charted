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

package org.noelware.charted.server.endpoints.v1.api

import org.koin.dsl.bind
import org.koin.dsl.module
import org.noelware.ktor.endpoints.AbstractEndpoint

val apiV1Endpoints = module {
    single { UsersEndpoint(get(), get(), get(), get(), get(), get(), get()) } bind AbstractEndpoint::class
    single { RepositoriesEndpoint(get(), get(), getOrNull()) } bind AbstractEndpoint::class
    single { OrganizationsEndpoint() } bind AbstractEndpoint::class
    single { ApiKeysEndpoint(get()) } bind AbstractEndpoint::class
    single { AdminEndpoint() } bind AbstractEndpoint::class
}
