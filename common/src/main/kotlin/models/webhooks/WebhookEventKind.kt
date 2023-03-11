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

package org.noelware.charted.models.webhooks

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Represents a kind of event that occurred and was recorded.
 */
@Serializable
public enum class WebhookEventKind {
    @SerialName("repo:modify")
    REPO_MODIFY,

    @SerialName("repo:starred")
    REPO_STARRED,

    @SerialName("repo:unstarred")
    REPO_UNSTARRED,

    @SerialName("repo:release:new")
    REPO_NEW_RELEASE,

    @SerialName("repo:release:update")
    REPO_UPDATE_RELEASE,

    @SerialName("repo:release:delete")
    REPO_DELETE_RELEASE,

    @SerialName("repo:members:new")
    REPO_NEW_MEMBER,

    @SerialName("repo:members:kick")
    REPO_KICK_MEMBER,

    @SerialName("repo:member:permissions:update")
    REPO_UPDATE_MEMBER_PERMISSIONS,

    @SerialName("org:modify")
    ORGANIZATION_MODIFY,

    @SerialName("org:members:new")
    ORGANIZATION_NEW_MEMBER,

    @SerialName("org:members:kick")
    ORGANIZATION_KICK_MEMBER,

    @SerialName("org:member:permissions:update")
    ORGANIZATION_UPDATE_MEMBER_PERMISSIONS
}
