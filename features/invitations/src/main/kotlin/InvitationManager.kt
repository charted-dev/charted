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

package org.noelware.charted.features.invitations

import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.models.repositories.RepositoryMember
import org.noelware.charted.models.users.User
import java.io.Closeable
import java.util.UUID

/**
 * Represents a manager for handling, accepting, and declining invitations.
 */
interface InvitationManager: Closeable {
    suspend fun createRepositoryMemberInvite(repo: Repository, user: User, to: String): RepoMemberInvite
    suspend fun acceptRepositoryMemberInvite(id: UUID): RepositoryMember
    suspend fun declineRepositoryMemberInvite(id: UUID)

    suspend fun createOrganizationMemberInvite(repoID: Long, userID: Long, to: String): OrgMemberInvite
    suspend fun acceptOrganizationMemberInvite(id: UUID): RepositoryMember
    suspend fun declineOrganizationMemberInvite(id: UUID)

    suspend fun isInviteExpired(id: UUID): Boolean
}
