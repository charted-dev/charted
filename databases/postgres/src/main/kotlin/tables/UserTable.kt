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

package org.noelware.charted.databases.postgres.tables

import kotlinx.datetime.toKotlinLocalDateTime
import net.perfectdreams.exposedpowerutils.sql.PGEnum
import org.jetbrains.exposed.sql.StringColumnType
import org.jetbrains.exposed.sql.kotlin.datetime.datetime
import org.noelware.charted.databases.postgres.SnowflakeTable
import org.noelware.charted.databases.postgres.columns.array
import org.noelware.charted.databases.postgres.flags.UserRole
import java.time.LocalDateTime
import kotlin.reflect.full.isSubclassOf

object UserTable: SnowflakeTable("users") {
    val gravatarEmail = text("gravatar_email").nullable().default(null)
    val description = varchar("description", 240).nullable().default(null)
    val avatarHash = text("avatar_hash").nullable().default(null)
    val createdAt = datetime("created_at").default(LocalDateTime.now().toKotlinLocalDateTime())
    val updatedAt = datetime("updated_at").default(LocalDateTime.now().toKotlinLocalDateTime())
    val username = varchar("username", 64).uniqueIndex()
    val password = text("password").nullable() // It is only null if other sources (unlike local) is being used
    val roles = array<UserRole>(
        "roles",
        object: StringColumnType() {
            override fun sqlType(): String = UserRole::class.simpleName!!.lowercase()
            override fun valueFromDB(value: Any): Any = if (value::class.isSubclassOf(Enum::class)) {
                value as UserRole
            } else {
                enumValueOf<UserRole>(value as String)
            }

            override fun notNullValueToDB(value: Any): Any = PGEnum(UserRole::class.simpleName!!.lowercase(), value as UserRole)
            override fun nonNullValueToString(value: Any): String = super.nonNullValueToString(notNullValueToDB(value))
        }
    ).default(arrayOf())

    val email = text("email").uniqueIndex()
    val name = varchar("name", 64).nullable().default(null)
}
