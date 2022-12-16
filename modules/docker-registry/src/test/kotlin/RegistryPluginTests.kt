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

package org.noelware.charted.modules.docker.registry.tests

import dev.floofy.utils.koin.inject
import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.request.*
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.Test
import org.koin.core.context.startKoin
import org.koin.dsl.module
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.slf4j.LoggerFactory
import org.testcontainers.containers.BindMode
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.containers.wait.strategy.HttpWaitStrategy
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals

@Testcontainers(disabledWithoutDocker = true)
class RegistryPluginTests {
    @Test
    fun `can we connect to the container`() = runBlocking {
        val httpClient: HttpClient by inject()
        val res = httpClient.get("http://${container.host}:${container.getMappedPort(5000)}/v2")

        assertEquals(200, res.status.value)
        assertNotEquals(503, res.status.value)
    }

    companion object {
        @Container
        @JvmStatic
        val container: GenericContainer<*> = GenericContainer(DockerImageName.parse("registry").withTag("2.8")).apply {
            withExposedPorts(5000)
            withAccessToHost(true)
            withClasspathResourceMapping(
                "/registry.yml",
                "/etc/docker/registry/config.yml",
                BindMode.READ_ONLY
            )

            setWaitStrategy(HttpWaitStrategy().forPort(5000).forPath("/v2/_catalog"))
            withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("com.docker.registry")))
        }

        @JvmStatic
        @BeforeAll
        fun launchKoin() {
            startKoin {
                modules(
                    module {
                        single {
                            Config.Builder().apply {
                                jwtSecretKey = "heck da uwu"
                                dockerRegistry {
                                    host = container.host
                                    port = container.getMappedPort(5000)
                                }
                            }.build()
                        }

                        single {
                            HttpClient(OkHttp) {
                                engine {
                                    config {
                                        followSslRedirects(true)
                                        followRedirects(true)
                                    }
                                }
                            }
                        }
                    }
                )
            }
        }
    }
}
