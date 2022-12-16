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

package org.noelware.charted.modules.webhooks

import dev.floofy.utils.slf4j.logging
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toJavaLocalDateTime
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.*
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import org.noelware.charted.Snowflake
import org.noelware.charted.databases.clickhouse.ClickHouseConnection
import org.noelware.charted.modules.webhooks.events.WebhookEvent
import org.noelware.charted.modules.webhooks.types.WebhookOriginKind
import org.noelware.charted.modules.webhooks.types.toDbType
import kotlin.reflect.KClass

class DefaultWebhookEventManager(private val clickhouse: ClickHouseConnection, private val json: Json): WebhookEventManager {
    private val log by logging<DefaultWebhookEventManager>()

    override suspend fun getAll(origin: Pair<WebhookOriginKind, Long>): List<WebhookEvent<JsonObject>> {
        log.info("Collecting all webhook events for ${origin.first} [${origin.second}]")
        return clickhouse.create {
            val events = mutableListOf<WebhookEvent<JsonObject>>()
            val stmt = prepareStatement("SELECT * FROM charted.webhook_events WHERE origin = ? AND origin_id = ?")
            stmt.setString(0, origin.first.toDbType)
            stmt.setLong(1, origin.second)

            val rs = stmt.executeQuery()
            if (!rs.next()) return@create emptyList<WebhookEvent<JsonObject>>()

            while (rs.next()) {
                val id = rs.getLong("ID")
                val firedAt = rs.getObject("FiredAt")
                val data = rs.getObject("Data")
                val originId = rs.getLong("Origin")
                val originType = rs.getString("OriginType")
                val action = rs.getString("Action")

                log.info("id=$id;firedAt=$firedAt;data=$data;origin=$originType;origin_id=$originId;action=$action")
            }

            events
        }
    }

    override suspend fun <T: Any> get(inner: KClass<T>, origin: Pair<WebhookOriginKind, Long>): WebhookEvent<T>? {
        TODO("Not yet implemented")
    }

    @OptIn(InternalSerializationApi::class)
    @Suppress("UNCHECKED_CAST")
    override suspend fun <T> create(origin: Pair<WebhookOriginKind, Long>, data: T?): WebhookEvent<T> = clickhouse.create {
        log.info("Creating webhook event [${origin.first} ${origin.second}]")

        val id = Snowflake.generate()
        val firedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
        val stmt = prepareStatement("INSERT INTO charted.webhook_events(*) VALUES(?, ?, ?, ?, ?, ?, ?);")
        stmt.setLong(0, id)
        stmt.setObject(1, firedAt.toJavaLocalDateTime())
        if (data != null) {
            stmt.setString(2, json.encodeToString(data!!::class.serializer() as KSerializer<T>, data))
        } else {
            stmt.setString(2, "{}")
        }

        stmt.execute()

        WebhookEvent(
            origin.first,
            firedAt,
            origin.second,
            data,
            id
        )
    }
}
