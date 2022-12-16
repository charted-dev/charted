/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.modules.webhooks.types

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
enum class WebhookActionKind {
    @SerialName("repo.modify")
    REPO_MODIFY,

    @SerialName("repo.starred")
    REPO_STARRED,

    @SerialName("repo.unstarred")
    REPO_UNSTARRED,

    @SerialName("repo.push")
    REPO_PUSH,

    @SerialName("repo.pull")
    REPO_PULL,

    @SerialName("repo.member_perm_update")
    REPO_MEMBER_PERMISSIONS_UPDATE,

    @SerialName("org.modify")
    ORG_MODIFY,

    @SerialName("org.new_member")
    ORG_NEW_MEMBER,

    @SerialName("org.kicked_member")
    ORG_KICKED_MEMBER,

    @SerialName("org.updated_member")
    ORG_UPDATED_MEMBER,

    @SerialName("org.repo_member_update")
    ORG_MEMBER_PERMISSIONS_UPDATE
}
