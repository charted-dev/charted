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

package org.noelware.charted.server.routing.v1

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.encodeToStream
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.util.*
import org.noelware.charted.common.extensions.regexp.matchesNameRegex
import org.noelware.charted.common.types.helm.ChartIndexYaml
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.postgresql.controllers.EntityNotFoundException
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.organizations.OrganizationDatabaseController
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.tables.OrganizationTable
import org.noelware.charted.modules.postgresql.tables.UserTable
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import org.noelware.charted.server.util.createBodyWithByteArray
import java.io.ByteArrayOutputStream

class IndexMappingsRestController(
    private val yaml: Yaml,
    private val charts: HelmChartModule? = null,
    private val userController: UserDatabaseController,
    private val organizationController: OrganizationDatabaseController
): RestController("/indexes/{idOrName}") {
    override val apiVersion: APIVersion = APIVersion.V1
    override suspend fun call(call: ApplicationCall) {
        if (charts == null) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val idOrName = call.parameters.getOrFail("idOrName")
        return when {
            idOrName.toLongOrNull() != null -> {
                val entry = charts.getIndexYaml(idOrName.toLong())
                    ?: return call.respond(HttpStatusCode.NotFound)

                val baos = ByteArrayOutputStream()
                yaml.encodeToStream(entry, baos)

                call.respond(createBodyWithByteArray(baos.toByteArray(), ContentType.parse("text/yaml; charset=utf-8")))
            }

            idOrName.matchesNameRegex() -> {
                // Is it a user index?
                val user = userController.getOrNull(UserTable::username to idOrName)
                if (user != null) {
                    val entry = charts.getIndexYaml(user.id)
                        ?: return call.respond(HttpStatusCode.NotFound)

                    val baos = ByteArrayOutputStream()
                    yaml.encodeToStream(entry, baos)

                    return call.respond(createBodyWithByteArray(baos.toByteArray(), ContentType.parse("text/yaml; charset=utf-8")))
                } else {
                    try {
                        val org = organizationController.get(OrganizationTable::name to idOrName)
                        val entry = charts.getIndexYaml(org.id)
                            ?: return call.respond(HttpStatusCode.NotFound)

                        val baos = ByteArrayOutputStream()
                        yaml.encodeToStream(entry, baos)

                        return call.respond(createBodyWithByteArray(baos.toByteArray(), ContentType.parse("text/yaml; charset=utf-8")))
                    } catch (ignored: EntityNotFoundException) {
                        return call.respond(HttpStatusCode.NotFound)
                    }
                }
            }

            else -> call.respond(HttpStatusCode.NotFound)
        }
    }

    companion object: ResourceDescription by describeResource("/indexes", {
        description = "Returns a organization or user's Helm index, which shows all the repositories to the Helm CLI."
        get {
            description = "Retrieve an organization or user's Helm index"

            idOrName()
            ok {
                description = "Helm index for the organization or user"
                yaml {
                    schema<ChartIndexYaml>()
                }
            }

            notFound {
                description = "User or organization doesn't exist"
                json {
                    schema<ApiResponse.Err>()
                }
            }
        }
    })
}
