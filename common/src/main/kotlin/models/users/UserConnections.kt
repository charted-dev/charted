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

package org.noelware.charted.models.users

import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Represents a user's connections that are used to log-in from different session
 * integrations.
 *
 * @param noelwareAccountID The account ID that is sourced from [Noelware's User Accounts System](https://accounts.noelware.org).
 * @param googleAccountID   Account ID that is sourced from Google
 * @param githubAccountID   Account ID that is sourced from GitHub
 * @param appleAccountID    Account ID that is sourced from Apple
 * @param createdAt         Creation date of when this user was created. This will have a slight offset
 *                          from a user's creation time since the user is first created.
 * @param updatedAt         Date of when this entity was last updated.
 * @param id                The user's ID that this entity this belongs to.
 */
@Serializable
public data class UserConnections(
    @SerialName("noelware_account_id")
    val noelwareAccountID: Long? = null,

    @SerialName("google_account_id")
    val googleAccountID: String? = null,

    @SerialName("github_account_id")
    val githubAccountID: String? = null,

    @SerialName("apple_account_id")
    val appleAccountID: String? = null,

    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,
    val id: Long
)
