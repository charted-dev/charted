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

package org.noelware.charted.database.models

import org.noelware.charted.database.entities.WebhookSettingsEntity
import org.noelware.charted.database.tables.WebhookEvent

@kotlinx.serialization.Serializable
data class WebhookSettings(
    val events: List<WebhookEvent>,
    val origin: Long,
    val token: String? = null,
    val url: String,
    val id: Long
) {
    companion object {
        fun fromEntity(entity: WebhookSettingsEntity, showToken: Boolean = false): WebhookSettings = WebhookSettings(
            entity.events.toList(),
            entity.origin,
            if (showToken) entity.authorization else null,
            entity.url,
            entity.id.value
        )
    }
}
