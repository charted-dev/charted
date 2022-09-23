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
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.condition.DisabledOnOs
import org.junit.jupiter.api.condition.OS
import org.slf4j.LoggerFactory
import org.testcontainers.containers.BindMode
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.containers.wait.strategy.HttpWaitStrategy
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals

@DisabledOnOs(value = [OS.WINDOWS, OS.MAC])
@Testcontainers(disabledWithoutDocker = true)
class DockerRegistryTests {
    private val container: GenericContainer<*> = GenericContainer(DockerImageName.parse("registry").withTag("2.8.1")).apply {
        withExposedPorts(5000)
        withAccessToHost(true)
        withClasspathResourceMapping(
            "/registry.yml",
            "/etc/docker/registry/config.yml",
            BindMode.READ_WRITE
        )

        waitingFor(HttpWaitStrategy().forPort(5000))
        withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("com.docker.registry")))
    }

    private val httpHost: String by lazy { "${container.host}:${container.getMappedPort(5000)}" }
    private val httpClient: HttpClient by lazy {
        HttpClient(OkHttp) {
            engine {
                config {
                    followSslRedirects(true)
                    followRedirects(true)
                }
            }

            install(ContentNegotiation) {
                json()
            }
        }
    }

    @Test
    fun `can we send requests to container`(): Unit = runBlocking {
        if (!container.isRunning) container.start()

        val res = httpClient.get("http://$httpHost/v2")
        assertEquals(200, res.status.value)
        assertNotEquals(500, res.status.value)
    }
}
