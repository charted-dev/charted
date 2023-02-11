/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.databases.postgres.metrics

import com.google.protobuf.Value
import kotlinx.serialization.Serializable
import org.noelware.charted.modules.analytics.kotlin.dsl.*

@Serializable
data class PostgresServerStats(
    val organizations: Long,
    val repositories: Long,
    val version: String,
    val dbSize: Long,
    val uptime: Long,
    val users: Long
) : org.noelware.analytics.jvm.server.serialization.Serializable {
    override fun toGrpcValue(): Value = Struct {
        put(this, PostgresServerStats::organizations)
        put(this, PostgresServerStats::repositories)
        put(this, PostgresServerStats::version)
        put(this, PostgresServerStats::dbSize)
        put(this, PostgresServerStats::uptime)
        put(this, PostgresServerStats::users)
    }.toGrpcValue()
}
