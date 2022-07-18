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

package org.noelware.charted.features.docker.registry.tests

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.testing.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.jsonArray
import org.junit.Test
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.features.docker.registry.DockerRegistryPlugin
import kotlin.test.assertNotNull

class DockerRegistryTests: AbstractRegistryTest() {
    @Test
    fun `can we connect`() {
        val host = container.host
        val port = container.getMappedPort(5000)
        val client = HttpClient(OkHttp) {
            engine {
                config {
                    followRedirects(true)
                    followSslRedirects(true)
                }
            }

            install(ContentNegotiation) {
                json()
            }
        }

        runBlocking {
            val res = client.get("http://$host:$port/v2/_catalog")
            val body: JsonObject = res.body()
            val repositories = body["repositories"]?.jsonArray

            assertNotNull(repositories)
            assert(repositories.isEmpty())
        }
    }

    @Test
    fun `test ktor proxy`() {
        val host = container.host
        val port = container.getMappedPort(5000)

        startKoin {
            modules(
                module {
                    single {
                        HttpClient(OkHttp) {
                            engine {
                                config {
                                    followRedirects(true)
                                    followSslRedirects(true)
                                }
                            }

                            install(ContentNegotiation) {
                                json()
                            }
                        }
                    }
                }
            )
        }

        testApplication {
            install(DockerRegistryPlugin) {
                this.host = host
                this.port = port
            }

            createClient {
                install(ContentNegotiation) {
                    json()
                }
            }

            val res = client.get("/v2/_catalog")
            val bodyString: String = res.body()
            val body = Json.decodeFromString(JsonObject.serializer(), bodyString)
            val repositories = body["repositories"]?.jsonArray

            assertNotNull(repositories)
            assert(repositories.isEmpty())
        }
    }
}
