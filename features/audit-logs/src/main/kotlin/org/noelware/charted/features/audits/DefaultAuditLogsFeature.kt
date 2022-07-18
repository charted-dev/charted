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

// class DefaultAuditLogsFeature(private val clickhouse: ClickHouseConnection): AuditLogsFeature {
//    private val log by logging<DefaultAuditLogsFeature>()
//
//    override suspend fun createAuditLog(
//        origin: Long,
//        originType: OriginType,
//        action: AuditLogAction,
//        data: JsonObject
//    ): AuditLog {
//        log.info("Creating audit log for [$action in $originType ($origin)]")
//
//        val now = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
//        val id = Snowflake.generate()
//        val log = AuditLog(originType, origin, now, action, data, id)
//
//        clickhouse.grabConnection {
//            val stmt = prepareStatement("")
//        }
//
//        return null as AuditLog
//    }
//
//    override suspend fun getAuditLogs(origin: OriginType, id: Long): List<AuditLog> {
//        val logs = clickhouse.sql(
//            "SELECT Action, Data, FiredAt, ID, OriginID, OriginType FROM audit_logs WHERE ID = ? AND OriginType = ?;",
//            id,
//            origin.key
//        ) ?: return emptyList()
//
//        println(logs)
//        return emptyList()
//    }
//
//    override suspend fun getAuditLog(id: Long): AuditLog? {
//        val audit = clickhouse.sql("SELECT Action, Data, FiredAt, ID, OriginID, OriginType FROM audit_logs WHERE ID = ?;", id)
//            ?: return null
//
//        return null
//    }
// }
