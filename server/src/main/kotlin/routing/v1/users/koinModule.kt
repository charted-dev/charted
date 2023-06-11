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

package org.noelware.charted.server.routing.v1.users

import org.koin.core.module.dsl.singleOf
import org.koin.dsl.bind
import org.koin.dsl.module
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.v1.users.avatars.usersV1AvatarModule
import org.noelware.charted.server.routing.v1.users.crud.usersV1CrudModule
import org.noelware.charted.server.routing.v1.users.repositories.usersV1RepositoriesModule
import org.noelware.charted.server.routing.v1.users.sessions.usersV1SessionsModule
import org.noelware.charted.server.util.composeKoinModules

val usersV1Module = composeKoinModules(
    usersV1AvatarModule, usersV1CrudModule, usersV1RepositoriesModule, usersV1SessionsModule,
    module {
        singleOf(::MainUserRestController) bind RestController::class
    },
)
