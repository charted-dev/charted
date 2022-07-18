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

package org.noelware.charted.features.audits.tests

import kotlinx.coroutines.runBlocking
import org.junit.Test
import org.junit.jupiter.api.Disabled
import org.noelware.charted.common.data.ClickHouseConfig
import org.noelware.charted.database.clickhouse.ClickHouseConnection
import org.noelware.charted.features.audits.AuditLogsFeature
import org.noelware.charted.features.audits.DefaultAuditLogsFeature
import org.noelware.charted.features.audits.data.OriginType
import org.noelware.charted.testing.containers.AbstractClickHouseContainerTest
import kotlin.test.assertTrue

@Disabled("Test is not finished or is broken.")
class AuditLogsTest: AbstractClickHouseContainerTest() {
    private val clickhouse: ClickHouseConnection
        get() {
            val config = ClickHouseConfig(
                true,
                host = getContainer().host,
                port = getContainer().getMappedPort(8123)
            )

            val ch = ClickHouseConnection(config)
            runBlocking { ch.connect() }

            return ch
        }

    private val feature: AuditLogsFeature = DefaultAuditLogsFeature(clickhouse)

    @Test
    fun `can we query audit logs`() = runBlocking {
        val auditLogs = feature.getAuditLogs(OriginType.ORGANIZATION, 123)
        assertTrue(auditLogs.isEmpty(), "audit logs was not empty(?)")
    }
}
