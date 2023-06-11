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

package org.noelware.charted.server.routing.v1

import org.koin.core.module.dsl.singleOf
import org.koin.dsl.bind
import org.koin.dsl.module
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.v1.admin.adminV1RoutingModule
import org.noelware.charted.server.routing.v1.apikeys.apiKeysV1Module
import org.noelware.charted.server.routing.v1.organizations.organizationsV1Module
import org.noelware.charted.server.routing.v1.repositories.repositoriesV1Module
import org.noelware.charted.server.routing.v1.users.usersV1Module
import org.noelware.charted.server.util.composeKoinModules

val routingV1Module = composeKoinModules(
    usersV1Module,
    apiKeysV1Module,
    adminV1RoutingModule,
    *repositoriesV1Module.toTypedArray(),
    *organizationsV1Module.toTypedArray(),
    module {
        singleOf(::IndexMappingsRestController) bind RestController::class
        singleOf(::HeartbeatRestController) bind RestController::class
        singleOf(::FeaturesRestController) bind RestController::class
        singleOf(::MetricsRestController) bind RestController::class
        singleOf(::InfoRestController) bind RestController::class
        singleOf(::MainRestController) bind RestController::class
    },
)
