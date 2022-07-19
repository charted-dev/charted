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

package org.noelware.charted.features.audits

import kotlinx.datetime.Clock
import kotlinx.datetime.toKotlinInstant
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import org.noelware.charted.common.Snowflake
import org.noelware.charted.database.cassandra.CassandraConnection
import org.noelware.charted.features.audits.data.AuditLog
import org.noelware.charted.features.audits.data.AuditLogAction
import org.noelware.charted.features.audits.data.OriginType

class DefaultAuditLogsFeature(private val cassandra: CassandraConnection, private val json: Json): AuditLogsFeature {
    override suspend fun getAuditLogs(origin: OriginType, id: Long): List<AuditLog> {
        val rs = cassandra.sql(
            "SELECT action, data, fired_at, id, origin_id, origin_type FROM charted.audit_logs WHERE origin_type = ? AND origin_id = ?;",
            origin.key,
            id
        ).all()

        if (rs.isEmpty()) return emptyList()
        return rs.map { item ->
            AuditLog(
                OriginType.values().find { it.key == item.getString("origin_type") }!!,
                item.getLong("origin"),
                item.getTimestamp("fired_at").toInstant().toKotlinInstant(),
                AuditLogAction.values().find { it.key == item.getString("action") }!!,
                json.decodeFromString(item.getString("data")),
                item.getLong("id")
            )
        }
    }

    override suspend fun getAuditLog(id: Long): AuditLog? {
        val rs = cassandra.sql("SELECT action, data, fired_at, id, origin_id, origin_type FROM charted.audit_logs WHERE id = ?;", id).all()
        if (rs.isEmpty()) return null

        val item = rs.first()
        return AuditLog(
            OriginType.values().find { it.key == item.getString("origin_type") }!!,
            item.getLong("origin"),
            item.getTimestamp("fired_at").toInstant().toKotlinInstant(),
            AuditLogAction.values().find { it.key == item.getString("action") }!!,
            json.decodeFromString(item.getString("data")),
            item.getLong("id")
        )
    }

    override suspend fun createAuditLog(
        origin: Long,
        originType: OriginType,
        action: AuditLogAction,
        data: JsonObject
    ): AuditLog {
        val id = Snowflake.generate()
        val firedAt = Clock.System.now()
        val dataString = json.encodeToString(data)
        val log = AuditLog(
            originType,
            origin,
            firedAt,
            action,
            data,
            id
        )

        try {
            cassandra.sql(
                "INSERT INTO charted.audit_logs(action, data, fired_at, id, origin_id, origin_type) VALUES(?, ?, ?, ?, ?, ?);",
                action.key,
                dataString,
                "$firedAt",
                id,
                origin,
                originType.key
            )

            return log
        } catch (e: Exception) {
            throw e
        }
    }
}
