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

import kotlinx.serialization.json.JsonObject
import org.noelware.charted.features.audits.data.AuditLog
import org.noelware.charted.features.audits.data.AuditLogAction
import org.noelware.charted.features.audits.data.OriginType

interface AuditLogsFeature {
    suspend fun getAuditLogs(origin: OriginType, id: Long): List<AuditLog>
    suspend fun getAuditLog(id: Long): AuditLog?
    suspend fun createAuditLog(
        origin: Long,
        originType: OriginType,
        action: AuditLogAction,
        data: JsonObject
    ): AuditLog
}
