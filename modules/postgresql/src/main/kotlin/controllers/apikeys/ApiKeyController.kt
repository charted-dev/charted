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

package org.noelware.charted.modules.postgresql.controllers.apikeys

import io.ktor.http.*
import io.ktor.server.application.*
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.and
import org.jetbrains.exposed.sql.deleteWhere
import org.jetbrains.exposed.sql.update
import org.noelware.charted.KtorHttpRespondException
import org.noelware.charted.common.Bitfield
import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.common.types.responses.ApiError
import org.noelware.charted.models.ApiKeys
import org.noelware.charted.models.flags.ApiKeyScopes
import org.noelware.charted.models.flags.SCOPES
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.controllers.AbstractController
import org.noelware.charted.modules.postgresql.controllers.BooleanOrError
import org.noelware.charted.modules.postgresql.entities.ApiKeyEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.postgresql.ktor.UserEntityAttributeKey
import org.noelware.charted.modules.postgresql.tables.ApiKeyTable
import org.noelware.charted.snowflake.Snowflake
import org.noelware.charted.utils.randomString
import kotlin.time.DurationUnit

class ApiKeyController(private val snowflake: Snowflake): AbstractController<ApiKeys, CreateApiKeyBody, PatchApiKeyBody>() {
    override suspend fun getOrNull(id: Long): ApiKeys? = asyncTransaction {
        ApiKeyEntity.findById(id)?.let { entity -> ApiKeys.fromEntity(entity, false) }
    }

    override suspend fun delete(id: Long): Unit = asyncTransaction {
        ApiKeyTable.deleteWhere { ApiKeyTable.id eq id }
    }

    override suspend fun update(call: ApplicationCall, id: Long, patched: PatchApiKeyBody): Map<String, BooleanOrError> {
        val currentUserEntity = call.attributes.getOrNull(UserEntityAttributeKey) ?: throw IllegalStateException("Unable to fetch user")
        if (patched.name != null) {
            val found = asyncTransaction {
                ApiKeyEntity.find {
                    (ApiKeyTable.name eq patched.name) and (ApiKeyTable.owner eq currentUserEntity.id)
                }.firstOrNull()
            }

            if (found != null) {
                return mapOf(
                    "body.name" to BooleanOrError(
                        ApiError(
                            "EXISTING_API_KEY",
                            "API key with new name [${patched.name}] already exists on your account.",
                        ),
                    ),
                )
            }
        }

        val bitfield = getApiKeyScopes(patched.scopes)
        return asyncTransaction {
            ApiKeyTable.update({ ApiKeyTable.id eq id }) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())

                if (patched.description != null) {
                    it[description] = patched.description
                }

                if (patched.expiresIn != null) {
                    it[expiresIn] = Clock.System.now().plus(patched.expiresIn.toDuration(DurationUnit.MILLISECONDS)).toLocalDateTime(TimeZone.currentSystemDefault())
                }

                if (patched.scopes.isNotEmpty()) {
                    it[scopes] = bitfield.bits()
                }

                if (patched.name != null) {
                    it[name] = patched.name
                }
            }

            emptyMap()
        }
    }

    override suspend fun create(call: ApplicationCall, data: CreateApiKeyBody): ApiKeys {
        val expiresIn = if (data.expiresIn != null) data.expiresIn.toDuration(DurationUnit.MILLISECONDS) else null
        val bitfield = getApiKeyScopes(data.scopes)
        val token = randomString(32)
        val id = snowflake.generate()

        // Since we can't get the extension for ApplicationCall.currentUserEntity (since
        // :server depends on :modules:postgresql, and we can't have circular dependencies),
        //
        // We have a `UserEntityAttributeKey` in modules/postgresql/src/main/kotlin/ktor/AttributeKeys.kt that
        // we can use. This is filled in if a session is available to us.
        val currentUserEntity = call.attributes.getOrNull(UserEntityAttributeKey) ?: throw IllegalStateException("Unable to fetch user")
        return asyncTransaction {
            ApiKeyEntity.new(id.value) {
                this.expiresIn = if (expiresIn != null) {
                    Clock.System.now().plus(expiresIn).toLocalDateTime(TimeZone.currentSystemDefault())
                } else {
                    null
                }

                this.token = CryptographyUtils.sha256Hex(token)
                scopes = bitfield.bits()
                owner = currentUserEntity
                name = data.name
            }.let { entity -> ApiKeys.fromEntity(entity, true, token) }
        }
    }

    private fun getApiKeyScopes(scopes: List<String>): Bitfield {
        val bitfield = ApiKeyScopes()

        // Check if we have a wildcard, which enables all scopes
        val isWildcard = scopes.isNotEmpty() && scopes.size == 1 && scopes.first() == "*"
        if (isWildcard) {
            bitfield.addAll()
        } else {
            val unknownKeys = mutableListOf<String>()
            for (key in scopes) {
                if (!SCOPES.containsKey(key)) unknownKeys.add(key) else bitfield.add(key)
            }

            if (unknownKeys.isNotEmpty()) {
                throw KtorHttpRespondException(
                    HttpStatusCode.NotAcceptable,
                    unknownKeys.map {
                        ApiError("UNKNOWN_API_KEY_SCOPE", "API key scope [$it] doesn't exist")
                    },
                )
            }
        }

        return bitfield
    }
}

suspend fun ApiKeyController.getOrNullByName(name: String): ApiKeys? = asyncTransaction {
    ApiKeyEntity.find { ApiKeyTable.name eq name }.firstOrNull()?.let { entity -> ApiKeys.fromEntity(entity, false) }
}
