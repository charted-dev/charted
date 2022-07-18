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

import dev.floofy.utils.slf4j.logging
import kotlinx.serialization.json.JsonObject
import org.noelware.charted.database.clickhouse.ClickHouseConnection
import org.noelware.charted.features.audits.data.AuditLog
import org.noelware.charted.features.audits.data.AuditLogAction
import org.noelware.charted.features.audits.data.OriginType

class DefaultAuditLogsFeature(private val clickhouse: ClickHouseConnection): AuditLogsFeature {
    private val log by logging<DefaultAuditLogsFeature>()

    override suspend fun createAuditLog(
        origin: Long,
        originType: OriginType,
        action: AuditLogAction,
        data: JsonObject
    ): AuditLog {
        log.info("Creating audit log for [$action in $originType ($origin)]")

        return null as AuditLog
    }

    override suspend fun getAuditLogs(origin: OriginType, id: Long): List<AuditLog> {
        val logs = clickhouse.sql(
            "SELECT Action, Data, FiredAt, ID, OriginID, OriginType FROM audit_logs WHERE ID = ? AND OriginType = ?;",
            id,
            origin.key
        )

        return emptyList()
    }

    override suspend fun getAuditLog(id: Long): AuditLog? {
        val audit = clickhouse.sql("SELECT Action, Data, FiredAt, ID, OriginID, OriginType FROM audit_logs WHERE ID = ?;", id)
            ?: return null

        return null
    }
}

/*
CREATE TABLE IF NOT EXISTS audit_logs(
    FiredAt DateTime64,
    ID UInt64,
    Action enum(
        'repo.modify',
        'repo.starred',
        'repo.unstarred',
        'repo.pull',
        'repo.push',
        'org.modify',
        'org.new_member',
        'org.updated_member',
        'org.kicked_member'
    ),
    Data JSON,
    OriginID UInt64,
    OriginType enum(
        'repo',
        'org'
    )
) ENGINE = MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (ACTION, FiredAt, ID, OriginID);
 */
