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

import org.koin.dsl.bind
import org.koin.dsl.module
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.v1.users.avatars.usersV1AvatarModule
import org.noelware.charted.server.routing.v1.users.crud.usersV1CrudModule
import org.noelware.charted.server.routing.v1.users.sessions.usersV1SessionsModule

val usersV1Module = usersV1CrudModule + usersV1AvatarModule + usersV1SessionsModule + module {
    single { MainUserRestController() } bind RestController::class
}
