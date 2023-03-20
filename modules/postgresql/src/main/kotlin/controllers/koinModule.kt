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

package org.noelware.charted.modules.postgresql.controllers

import org.koin.dsl.module
import org.noelware.charted.modules.postgresql.controllers.apikeys.ApiKeysDatabaseController
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.controllers.users.connections.UserConnectionsDatabaseController

val controllersModule = module {
    single { UserDatabaseController(get(), get(), get()) }
    single { RepositoryDatabaseController(get(), get()) }
    single { OrganizationDatabaseController(get()) }
    single { UserConnectionsDatabaseController() }
    single { ApiKeysDatabaseController(get()) }
}
