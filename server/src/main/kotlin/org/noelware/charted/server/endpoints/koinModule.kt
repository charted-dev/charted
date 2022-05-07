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

import org.koin.dsl.bind
import org.koin.dsl.module
import org.noelware.ktor.endpoints.AbstractEndpoint

val endpointsModule = module {
    single { DebugEndpoint(getOrNull(), getOrNull(), getOrNull(), getOrNull()) } bind AbstractEndpoint::class
    single { InfoEndpoint(getOrNull(), getOrNull(), getOrNull(), getOrNull()) } bind AbstractEndpoint::class
    single { OrganizationMemberApiEndpoints() } bind AbstractEndpoint::class
    single { RepositoryMemberApiEndpoints() } bind AbstractEndpoint::class
    single { RepositoryApiEndpoints() } bind AbstractEndpoint::class
    single { UserApiEndpoints() } bind AbstractEndpoint::class
    single { MetricsEndpoint() } bind AbstractEndpoint::class
    single { SearchEndpoint() } bind AbstractEndpoint::class
    single { MainEndpoint() } bind AbstractEndpoint::class
}
