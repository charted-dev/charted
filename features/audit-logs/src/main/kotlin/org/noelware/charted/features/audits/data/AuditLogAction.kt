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

package org.noelware.charted.features.audits.data

/**
 * Represents the action of the audit log.
 * @param key The database key for the enum object.
 */
enum class AuditLogAction(val key: String) {
    /**
     * The owner or a member who has the `repo:update` permissions modified the repository.
     * The payload will contain the old and new information.
     */
    REPO_MODIFY("repo.modify"),

    /**
     * A user has starred the repository! The payload will contain the user who starred it
     * and the repository.
     */
    REPO_STARRED("repo.starred"),

    /**
     * A user has unstarred this repository. The payload will contain the user who unstarred it
     * and the repository.
     */
    REPO_UNSTARRED("repo.unstarred"),

    /**
     * A user has pulled the repository. The payload will contain the repository and the user payload
     * if the repository is private.
     */
    REPO_PULL("repo.pull"),

    /**
     * A user has pushed a new repository changed to the main server. The payload will contain the repository
     * and the user who pushed the repository.
     */
    REPO_PUSH("repo.push"),
    ORGANIZATION_MODIFY("org.modify"),
    ORGANIZATION_NEW_MEMBER("org.new_member"),
    ORGANIZATION_UPDATED_MEMBER("org.updated_member"),
    ORGANIZATION_KICKED_MEMBER("org.kicked_member");
}
