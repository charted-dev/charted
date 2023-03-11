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

@file:JvmName("FromEntityExtensionsKt")

package org.noelware.charted.modules.postgresql.extensions

import org.noelware.charted.models.ApiKeys
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.organizations.OrganizationMember
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.models.repositories.RepositoryMember
import org.noelware.charted.models.repositories.RepositoryRelease
import org.noelware.charted.models.users.User
import org.noelware.charted.models.users.UserConnections
import org.noelware.charted.modules.postgresql.entities.*

fun ApiKeys.Companion.fromEntity(entity: ApiKeyEntity, showToken: Boolean = false, token: String? = null): ApiKeys = ApiKeys(
    entity.description,
    entity.expiresIn,
    entity.scopes,
    if (showToken) token else null,
    User.fromEntity(entity.owner),
    entity.name,
    entity.id.value,
)

fun Organization.Companion.fromEntity(entity: OrganizationEntity): Organization = Organization(
    entity.verifiedPublisher,
    entity.twitterHandle,
    entity.gravatarEmail,
    entity.displayName,
    entity.createdAt,
    entity.updatedAt,
    entity.iconHash,
    entity.private,
    User.fromEntity(entity.owner),
    entity.name,
    entity.id.value,
)

fun OrganizationMember.Companion.fromEntity(entity: OrganizationMemberEntity): OrganizationMember = OrganizationMember(
    entity.displayName,
    entity.permissions,
    entity.updatedAt,
    entity.joinedAt,
    User.fromEntity(entity.account),
    entity.id.value,
)

fun Repository.Companion.fromEntity(entity: RepositoryEntity): Repository = Repository(
    entity.description,
    entity.deprecated,
    entity.createdAt,
    entity.updatedAt,
    entity.iconHash,
    entity.private,
    entity.owner,
    entity.name,
    entity.type,
    entity.id.value,
)

fun RepositoryMember.Companion.fromEntity(entity: RepositoryMemberEntity): RepositoryMember = RepositoryMember(
    entity.displayName,
    entity.permissions,
    entity.updatedAt,
    entity.joinedAt,
    User.fromEntity(entity.account),
    entity.id.value,
)

fun RepositoryRelease.Companion.fromEntity(entity: RepositoryReleaseEntity): RepositoryRelease = RepositoryRelease(
    entity.updateText,
    entity.createdAt,
    entity.updatedAt,
    entity.tag,
    entity.id.value,
)

fun User.Companion.fromEntity(entity: UserEntity): User = User(
    entity.verifiedPublisher,
    entity.gravatarEmail,
    entity.description,
    entity.avatarHash,
    entity.createdAt,
    entity.updatedAt,
    entity.username,
    entity.admin,
    entity.name,
    entity.id.value,
)

fun UserConnections.Companion.fromEntity(entity: UserConnectionsEntity): UserConnections = UserConnections(
    entity.noelwareAccountID,
    entity.googleAccountID,
    entity.githubAccountID,
    entity.appleAccountID,
    entity.createdAt,
    entity.updatedAt,
    entity.id.value,
)
