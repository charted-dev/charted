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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.features.audit

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.database.clickhouse.ClickHouseConnection

class AuditLogsFeature(private val clickhouse: ClickHouseConnection) {
    private val log by logging<AuditLogsFeature>()

    fun getAuditLog(origin: Long, id: Long): Any? {
        val result = clickhouse.sql(
            "SELECT Action, FiredAt, ID, Data, OriginID, OriginType FROM audit_logs WHERE ID = ? AND OriginID = ?;",
            origin,
            id
        )

        return if (result == null) null else "awa"
    }
}
