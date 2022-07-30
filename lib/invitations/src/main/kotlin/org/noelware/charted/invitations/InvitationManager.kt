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

package org.noelware.charted.invitations

import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.models.Organization
import org.noelware.charted.database.models.Repository
import java.io.Closeable

/**
 * Represents the manager to handle member invitations.
 */
interface InvitationManager: Closeable {
    /**
     * Sends out an invitation to join a repository if the email service is registered and returns the URL
     * to accept the invitation.
     */
    suspend fun sendRepositoryInvitation(repository: Repository, user: UserEntity): String

    /**
     * Sends out an invitation to join a repository if the email service is registered and returns the URL
     * to accept the invitation.
     */
    suspend fun sendOrganizationInvitation(organization: Organization, user: UserEntity): String

    /**
     * Validates the invitation to check if it's still available as a boolean value.
     * @param id The repository or organization ID.
     * @param userID The user's ID that the invitation belongs to.
     * @param code The invitation's unique identifier that the invitation was attached to.
     * @return If the invitation is still value, `true` otherwise `false`.
     */
    suspend fun validateInvitation(id: Long, userID: Long, code: String): Boolean
}
