/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.configuration.kotlin.dsl.metrics.keysets

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.enumSets.EnumSet

@Serializable
public enum class PostgresKeysets {
    @SerialName("charted_postgres_total_organizations")
    TotalOrganizationsAvailable,

    @SerialName("charted_postgres_total_repositories")
    TotalRepositoriesAvailable,

    @SerialName("charted_postgres_total_users")
    TotalUsersAvailable,

    @SerialName("charted_postgres_database_size")
    DatabaseSize,

    @SerialName("charted_postgres_server_uptime")
    ServerUptime,

    @SerialName("*")
    Wildcard,

    @SerialName("charted_postgres_version")
    Version;

    public companion object {
        // I don't know why this is kept but whatever
        @Suppress("unused")
        internal object Serializer: KSerializer<PostgresKeysets> by PostgresKeysets.serializer()
    }
}

public val PostgresKeysets.Companion.enumSet: EnumSet<PostgresKeysets>
    get() = PostgresKeysetEnumSet

public object PostgresKeysetEnumSet: EnumSet<PostgresKeysets>(PostgresKeysets::class) {
    override val wildcard: PostgresKeysets
        get() = PostgresKeysets.Wildcard
}
