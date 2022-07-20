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

package org.noelware.charted.database.tables

import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import net.perfectdreams.exposedpowerutils.sql.PGEnum
import org.jetbrains.exposed.sql.StringColumnType
import org.noelware.charted.database.SnowflakeTable
import org.noelware.charted.database.columns.array
import kotlin.reflect.full.isSubclassOf

@kotlinx.serialization.Serializable(with = WebhookEvent.Companion::class)
enum class WebhookEvent(val key: String) {
    REPO_MODIFY("repo:modify"),
    REPO_STARRED("repo:starred"),
    REPO_UNSTARRED("repo:unstarred"),
    REPO_NEW_RELEASE("repo:new:release"),
    REPO_UPDATE_RELEASE("repo:update:release"),
    REPO_DELETE_RELEASE("repo:delete:release"),
    REPO_PULL("repo:pull"),
    REPO_PUSH("repo:push"),
    ORG_MODIFY("org:modify"),
    ORG_NEW_MEMBER("org:members:new"),
    ORG_UPDATED_MEMBER("org:members:updated"),
    ORG_REMOVED_MEMBER("org:members:removed");

    companion object: KSerializer<WebhookEvent> {
        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.WebhookEvent", PrimitiveKind.STRING)
        override fun deserialize(decoder: Decoder): WebhookEvent {
            val key = decoder.decodeString()
            return WebhookEvent.values().find { it.key == key } ?: error("Unknown webhook event [$key]")
        }

        override fun serialize(encoder: Encoder, value: WebhookEvent) {
            encoder.encodeString(value.key)
        }
    }
}

object WebhookSettingsTable: SnowflakeTable("webhook_settings") {
    val authorization = text("authorization")
    val events = array<WebhookEvent>(
        "events",
        object: StringColumnType() {
            override fun sqlType(): String = WebhookEvent::class.simpleName!!.lowercase()
            override fun valueFromDB(value: Any): Any =
                if (value::class.isSubclassOf(WebhookEvent::class)) {
                    value as WebhookEvent
                } else {
                    enumValueOf<WebhookEvent>(value as String)
                }

            override fun notNullValueToDB(value: Any): Any = PGEnum(WebhookEvent::class.simpleName!!.lowercase(), value as WebhookEvent)
            override fun nonNullValueToString(value: Any): String = super.nonNullValueToString(notNullValueToDB(value))
        }
    )

    val origin = long("origin")
    val url = text("url")
}
