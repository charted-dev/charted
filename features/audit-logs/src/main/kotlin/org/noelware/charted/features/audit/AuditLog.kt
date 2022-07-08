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

package org.noelware.charted.features.audit

import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.json.JsonObject

data class AuditLog<O>(
    val firedAt: LocalDateTime,
    val id: Long,
    val action: AuditLogAction,
    val data: JsonObject,
    val originID: Long,
    val origin: O
)

enum class AuditLogAction(val enumName: String) {
    REPOSITORY_MODIFY("repo.modify"),
    REPOSITORY_STARRED("repo.starred"),
    REPOSITORY_UNSTARRED("repo.unstarred"),
    REPOSITORY_PULL("repo.pull"),
    REPOSITORY_PUSH("repo.push"),
    ORGANIZATION_MODIFY("org.modify"),
    ORGANIZATION_NEW_MEMBER("org.new_member"),
    ORGANIZATION_UPDATED_MEMBER("org.updated_member"),
    ORGANIZATION_REMOVE_MEMBER("org.remove_member");
}
