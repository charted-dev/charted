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

import dev.floofy.utils.slf4j.logging
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import org.noelware.charted.database.clickhouse.TransformResultSetInto
import java.sql.ResultSet

@kotlinx.serialization.Serializable
data class AuditLog(
    @SerialName("origin_type")
    val originType: OriginType,

    @SerialName("origin_id")
    val originID: Long,

    @SerialName("fired_at")
    val firedAt: LocalDateTime,
    val action: AuditLogAction,
    val data: JsonObject,
    val id: Long
) {
    companion object: TransformResultSetInto<AuditLog> {
        private val log by logging<Companion>()
        override fun transform(rs: ResultSet): AuditLog {
            val originType = rs.getInt("OriginType")
            val originID = rs.getLong("OriginID")
            val firedAt = rs.getObject("FiredAt")
            val action = rs.getInt("Action")
            val data = rs.getObject("Data")
            val id = rs.getLong("ID")

            log.debug(
                buildString {
                    appendLine("origin_type = $originType")
                    appendLine("origin_id = $originID")
                    appendLine("fired_at = $firedAt")
                    appendLine("action = $action")
                    appendLine("data = $data")
                    appendLine("id = $id")
                }
            )

            return AuditLog(
                OriginType.ORGANIZATION,
                69420L,
                LocalDateTime.parse("2022-07-18T02:23:47.326Z"),
                AuditLogAction.ORGANIZATION_MODIFY,
                buildJsonObject {},
                4444
            )
        }
    }
}
