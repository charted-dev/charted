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

package org.noelware.charted.server.routing

import org.koin.dsl.module
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.toApiBaseUrl
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.openApi
import org.noelware.charted.server.routing.v1.routingV1Module

val routingModule = routingV1Module + module {
    single {
        val config: Config = get()
        openApi {
            server {
                description("Production Server/Official Instance")
                url("https://charts.noelware.org/api")
            }

            server {
                description("Main Instance")
                url(config.toApiBaseUrl().trimEnd('/'))
            }

            path("/_/openapi") {
                description = "Endpoint for the OpenAPI specification for charted-server"
                get {
                    description = "Gets the document in JSON format or YAML format"
                    queryParameter {
                        description = "Format to use"
                        name = "format"

                        schema<String>()
                    }
                }
            }
        }
    }
}
