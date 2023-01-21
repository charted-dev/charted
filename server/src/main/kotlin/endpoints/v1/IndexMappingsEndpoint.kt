/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

@file:Suppress("UNUSED")

package org.noelware.charted.server.endpoints.v1

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.encodeToStream
import dev.floofy.utils.exposed.asyncTransaction
import guru.zoroark.tegral.openapi.dsl.RootDsl
import guru.zoroark.tegral.openapi.dsl.schema
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.ChartedInfo
import org.noelware.charted.ChartedScope
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.databases.postgres.entities.OrganizationEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.tables.OrganizationTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.server.createKtorContentWithByteArray
import org.noelware.charted.types.helm.ChartIndexYaml
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import java.io.ByteArrayOutputStream

class IndexMappingsEndpoint(
    private val config: Config,
    private val yaml: Yaml,
    private val charts: HelmChartModule? = null
) : AbstractEndpoint("/indexes") {
    @Get("/{idOrName}")
    suspend fun getIndexYamlByIdOrName(call: ApplicationCall) {
        if (charts == null) return call.respond(HttpStatusCode.NotFound)

        val idOrName = call.parameters["idOrName"]!!
        return when {
            idOrName.toLongOrNull() != null -> {
                val entry = charts.getIndexYaml(idOrName.toLong())
                    ?: return call.respond(HttpStatusCode.NotFound)

                val baos = ByteArrayOutputStream()
                yaml.encodeToStream(entry, baos)

                call.respond(createKtorContentWithByteArray(baos.toByteArray(), ContentType.parse("text/yaml; charset=utf-8")))
            }

            idOrName.toNameRegex(false).matches() -> {
                val user = asyncTransaction(ChartedScope) {
                    UserEntity.find { UserTable.name eq idOrName }.firstOrNull()
                }

                if (user != null) {
                    val entry = charts.getIndexYaml(user.id.value)
                        ?: return call.respond(HttpStatusCode.NotFound)

                    val baos = ByteArrayOutputStream()
                    yaml.encodeToStream(entry, baos)

                    return call.respond(createKtorContentWithByteArray(baos.toByteArray(), ContentType.parse("text/yaml; charset=utf-8")))
                }

                val org = asyncTransaction(ChartedScope) {
                    OrganizationEntity.find { OrganizationTable.name eq idOrName }.firstOrNull()
                }

                if (org != null) {
                    val entry = charts.getIndexYaml(org.id.value)
                        ?: return call.respond(HttpStatusCode.NotFound)

                    val baos = ByteArrayOutputStream()
                    yaml.encodeToStream(entry, baos)

                    return call.respond(createKtorContentWithByteArray(baos.toByteArray(), ContentType.parse("text/yaml; charset=utf-8")))
                }

                call.respond(HttpStatusCode.NotFound)
            }

            else -> call.respond(HttpStatusCode.NotFound)
        }
    }

    companion object {
        fun RootDsl.toOpenAPI() {
            "/indexes" get {
                externalDocsUrl = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/indexes#GET-/:idOrName"
                summary = "Returns a user or organization's chart index"

                200 response {
                    "text/yaml" content {
                        schema<ChartIndexYaml>()
                    }
                }

                404 response {
                    "application/json" content {
                        schema<ApiResponse.Err>()
                    }
                }
            }
        }
    }
}
