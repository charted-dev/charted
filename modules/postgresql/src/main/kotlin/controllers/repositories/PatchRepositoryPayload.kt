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

package org.noelware.charted.modules.postgresql.controllers.repositories

import kotlinx.serialization.Serializable
import org.noelware.charted.StringOverflowException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.extensions.regexp.matchesRepoNameRegex
import org.noelware.charted.common.types.helm.RepoType

@Serializable
data class PatchRepositoryPayload(
    val description: String? = null,
    val deprecated: Boolean? = null,
    val private: Boolean? = null,
    val name: String? = null,
    val type: RepoType? = null
) {
    init {
        if (description != null && description.length > 64) {
            throw StringOverflowException("body.description", 64, description.length)
        }

        if (name != null) {
            if (!name.matchesRepoNameRegex()) {
                throw ValidationException("body.name", "Repository name can only contain letters, digits, dashes, or underscores.")
            }
        }
    }
}
