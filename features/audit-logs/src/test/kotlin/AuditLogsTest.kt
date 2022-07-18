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

// import kotlinx.coroutines.runBlocking
// import kotlinx.datetime.Clock
// import kotlinx.datetime.TimeZone
// import kotlinx.datetime.toLocalDateTime
// import kotlinx.serialization.json.buildJsonObject
// import org.junit.Test
// import org.noelware.charted.common.data.ClickHouseConfig
// import org.noelware.charted.database.clickhouse.ClickHouseConnection
// import org.noelware.charted.features.audits.DefaultAuditLogsFeature
// import org.noelware.charted.features.audits.data.AuditLog
// import org.noelware.charted.features.audits.data.AuditLogAction
// import org.noelware.charted.features.audits.data.OriginType
// import org.noelware.charted.testing.containers.AbstractClickHouseContainerTest
// import kotlin.test.assertTrue
//
// class AuditLogsTest: AbstractClickHouseContainerTest() {
//    private val clickhouse by lazy {
//        val config = ClickHouseConfig(
//            true,
//            host = getContainer().host,
//            port = getContainer().getMappedPort(8123)
//        )
//
//        val ch = ClickHouseConnection(config)
//        runBlocking {
//            ch.connect()
//        }
//
//        // this looks bad but i hope this works :woeme:
//        ch.sql(buildString {
//            append("SET allow_experimental_object_type = 1; ")
//            append("CREATE TABLE IF NOT EXISTS audit_logs(FiredAt DateTime64, ID UInt64, Action enum(")
//
//            val enumMembers = listOf("repo.modify", "repo.starred", "repo.unstarred", "repo.pull", "repo.push", "org.modify", "org.new_member", "org.kicked_member", "org.updated_member")
//            for ((index, item) in enumMembers.withIndex()) {
//                append("'$item'${if (index + 1 != enumMembers.size) "," else ""}")
//            }
//
//            append("), Data JSON, OriginID UInt64, OriginType enum('repo', 'org')) ")
//            append("ENGINE = MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (Action, FiredAt, ID, OriginID);")
//        })
//
//        ch
//    }
//
//    @Test
//    fun `can we query audit logs`() = runBlocking {
//        val feature = DefaultAuditLogsFeature(clickhouse)
//        val auditLogs = feature.getAuditLogs(OriginType.ORGANIZATION, 123)
//        assertTrue(auditLogs.isEmpty(), "audit logs was not empty(?)")
//    }
//
//    @Test
//    fun `insert audit log`() = runBlocking {
//        val stub = AuditLog(
//            OriginType.ORGANIZATION,
//            123,
//            Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault()),
//            AuditLogAction.ORGANIZATION_MODIFY,
//            buildJsonObject {},
//            444
//        )
//    }
// }
