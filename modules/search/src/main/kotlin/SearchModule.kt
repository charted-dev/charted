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

package org.noelware.charted.modules.search

import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.models.users.User
import java.io.Closeable

interface SearchModule: Closeable {
    /**
     * Version of the server that this [SearchModule] is using.
     */
    val serverVersion: String

    /**
     * If this [SearchModule] is closed and can't do any other operations
     * at this time.
     */
    val closed: Boolean

    /**
     * Indexes a user into this [SearchModule].
     * @param user The user to insert
     */
    suspend fun indexUser(user: User)

    /**
     * Indexes a repository into this [SearchModule].
     * @param repository The repository to insert
     */
    suspend fun indexRepository(repository: Repository)

    /**
     * Indexes an organization into this [SearchModule]
     * @param org The organization to insert
     */
    suspend fun indexOrganization(org: Organization)

    /**
     * Un-indexes a user from this [SearchModule].
     * @param user The user to insert
     */
    suspend fun unindexUser(user: User)

    /**
     * Un-indexes a repository from this [SearchModule].
     * @param repository The repository to insert
     */
    suspend fun unindexRepository(repository: Repository)

    /**
     * Un-indexes an organization from this [SearchModule]
     * @param org The organization to insert
     */
    suspend fun unindexOrganization(org: Organization)

    /**
     * Indexes all the data available into all indexes. This is a very expensive
     * function, so this will be called at the start of the server's
     * lifespan.
     */
    suspend fun indexAllData()

    /**
     * Initializes this [SearchModule], if necessary
     */
    suspend fun init() {}
}
