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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.webhooks

import kotlinx.serialization.SerialName

@kotlinx.serialization.Serializable
enum class WebhookEvents {
    @SerialName("repo.release.new")
    REPO_NEW_RELEASE,

    @SerialName("repo.release.update")
    REPO_UPDATE_RELEASE,

    @SerialName("repo.release.delete")
    REPO_DELETE_RELEASE,

    @SerialName("repo.members.joined")
    REPO_MEMBER_JOINED,

    @SerialName("repo.members.left")
    REPO_MEMBER_LEFT,

    @SerialName("repo.updated")
    REPO_METADATA_UPDATED,

    @SerialName("org.member.joined")
    ORG_MEMBER_JOINED,

    @SerialName("org.member.left")
    ORG_MEMBER_LEFT;
}

val WebhookEvents.key: String
    get() = when (this) {
        WebhookEvents.REPO_METADATA_UPDATED -> "repo.updated"
        WebhookEvents.REPO_NEW_RELEASE -> "repo.release.new"
        WebhookEvents.REPO_DELETE_RELEASE -> "repo.release.delete"
        WebhookEvents.REPO_UPDATE_RELEASE -> "repo.release.update"
        WebhookEvents.REPO_MEMBER_JOINED -> "repo.members.joined"
        WebhookEvents.REPO_MEMBER_LEFT -> "repo.members.left"
        WebhookEvents.ORG_MEMBER_JOINED -> "org.member.joined"
        WebhookEvents.ORG_MEMBER_LEFT -> "org.member.left"
    }
