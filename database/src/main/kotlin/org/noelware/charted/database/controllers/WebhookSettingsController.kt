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

package org.noelware.charted.database.controllers

import dev.floofy.utils.exposed.asyncTransaction
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.RandomGenerator
import org.noelware.charted.common.Snowflake
import org.noelware.charted.database.entities.WebhookSettingsEntity
import org.noelware.charted.database.models.WebhookSettings
import org.noelware.charted.database.tables.WebhookEvent
import java.net.URI
import java.net.URISyntaxException

object WebhookSettingsController {
    suspend fun get(id: Long, showToken: Boolean = false): WebhookSettings? = asyncTransaction(ChartedScope) {
        WebhookSettingsEntity.findById(id)?.let { entity -> WebhookSettings.fromEntity(entity, showToken) }
    }

    suspend fun create(
        id: Long,
        url: String,
        events: List<WebhookEvent>
    ): WebhookSettings {
        val auth = RandomGenerator.generate(64)
        try {
            URI.create(url)
        } catch (e: URISyntaxException) {
            throw e
        }

        val webhookId = Snowflake.generate()
        return asyncTransaction(ChartedScope) {
            WebhookSettingsEntity.new(webhookId) {
                this.authorization = auth
                this.origin = id
                this.events = events.toTypedArray()
                this.url = url
            }.let { entity -> WebhookSettings.fromEntity(entity, true) }
        }
    }
}
