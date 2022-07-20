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

package org.noelware.charted.features.webhooks

import com.google.common.hash.Hashing
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import kotlinx.datetime.Clock
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.common.Snowflake
import org.noelware.charted.database.cassandra.CassandraConnection
import org.noelware.charted.database.controllers.WebhookSettingsController
import org.noelware.charted.database.tables.WebhookEvent
import org.noelware.charted.features.webhooks.data.OriginType
import org.noelware.charted.features.webhooks.data.WebhookEventData
import java.net.URI

class DefaultWebhookFeature(
    private val cassandra: CassandraConnection,
    private val httpClient: HttpClient,
    private val json: Json
): WebhooksFeature {
    private val log by logging<DefaultWebhookFeature>()

    override suspend fun get(id: Long): WebhookEventData? {
        TODO("Not yet implemented")
    }

    override suspend fun all(origin: Long): List<WebhookEventData> {
        TODO("Not yet implemented")
    }

    override suspend fun getAllByEvent(origin: Long, event: WebhookEvent): List<WebhookEventData> {
        TODO("Not yet implemented")
    }

    override suspend fun create(
        event: WebhookEvent,
        id: Long,
        origin: Long,
        originType: OriginType,
        data: JsonObject
    ): WebhookEventData {
        // We need the token so we can create the sha256 hash
        val webhook = WebhookSettingsController.get(id, true) ?: throw IllegalArgumentException("Unknown webhook [$id]")

        // Create the sha256 hash with the intended payload
        val payload = buildJsonObject {
            put("format_version", 1)
            put("event", event.key)
            put("data", data)
        }

        val sha256 = Hashing.hmacSha256(webhook.token!!.toByteArray()).hashBytes(json.encodeToString(payload).toByteArray())
        val now = Clock.System.now()

        // Send out the request
        val res = httpClient.post(URI.create(webhook.url).toURL()) {
            header("X-Webhook-Timestamp", now)
            header("X-Webhook-Signature", sha256)
            header("Authorization", webhook.token!!)
            setBody(
                buildJsonObject {
                    put("format_version", 1)
                    put("event", event.key)
                    put("data", data)
                }
            )
        }

        val code = res.status.value
        val isSuccess = res.status.isSuccess()
        val eventID = Snowflake.generate()
        val responseBack = res.body<ByteArray>()

        cassandra.sql(
            buildString {
                append("INSERT INTO webhooks(id, origin_id, origin_type, data, fired_at, response_code, successful, failed, response_payload, event) VALUES")
                append("VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            },
            eventID, origin, originType.key, payload, now.toString(), code, isSuccess, !isSuccess, String(responseBack), event.key
        )

        return WebhookEventData(
            code,
            originType,
            isSuccess,
            json.decodeFromString(responseBack.toString()),
            now,
            !isSuccess,
            origin,
            event,
            payload,
            eventID
        )
    }
}
