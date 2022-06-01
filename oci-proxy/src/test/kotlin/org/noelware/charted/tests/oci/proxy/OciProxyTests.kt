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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.tests.oci.proxy

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.server.testing.*
import io.ktor.utils.io.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import org.junit.Test
import org.noelware.charted.common.config.NoAuthStrategy
import org.noelware.charted.common.config.OciProxyConfig
import org.noelware.charted.oci.proxy.OciProxyPlugin
import org.slf4j.LoggerFactory
import org.testcontainers.containers.BindMode
import org.testcontainers.containers.GenericContainer
import org.testcontainers.utility.DockerImageName
import java.io.File
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.assertEquals

private val registry by lazy {
    val image = DockerImageName.parse("registry:2.8.1")
    val container = GenericContainer(image)
        .withExposedPorts(5000)
        .withClasspathResourceMapping("registry.yml", "/etc/docker/registry/config.yml", BindMode.READ_ONLY)
        .withFileSystemBind("./.registry/data", "/var/lib/registry", BindMode.READ_WRITE)
        .withLogConsumer {
            val log = LoggerFactory.getLogger("org.noelware.charted.tests.oci.proxy.DockerRegistryLogs")
            log.info(it.utf8String)
        }

    container.start()
    container
}

@kotlinx.serialization.Serializable
data class RepositoriesResult(
    val repositories: List<String>
)

class OciProxyTests {
    private val log = LoggerFactory.getLogger(OciProxyTests::class.java)
    private lateinit var client: HttpClient

    @BeforeTest
    fun initClient() {
        client = HttpClient(OkHttp) {
            engine {
                config {
                    followRedirects(true)
                }
            }

            install(ContentNegotiation) {
                json(Json { ignoreUnknownKeys = true })
            }
        }

        if (!File("./.registry/data").exists())
            File("./.registry/data").mkdirs()
    }

    @AfterTest
    fun deleteRegistryData() {
        File("./.registry/data").deleteRecursively()
    }

    @Test
    fun `is connection ok`() {
        val resp = runBlocking { client.head("http://${registry.host}:${registry.firstMappedPort}") }
        assertEquals(resp.status.value, 200)
        assertEquals(resp.status.description, "OK")
    }

    @Test
    fun `is the repositories list empty`() {
        val resp = runBlocking { client.get("http://${registry.host}:${registry.firstMappedPort}/v2/_catalog") }
        assertEquals(resp.status.value, 200)
        assertEquals(resp.status.description, "OK")

        val data = runBlocking { resp.body<RepositoriesResult>() }
        assertEquals(data.repositories, listOf())
    }

    @Test
    fun `can we proxy this correctly`() = testApplication {
        install(OciProxyPlugin) {
            withAuthStrategy(NoAuthStrategy())
            fromConfig(
                OciProxyConfig(
                    port = registry.firstMappedPort,
                    host = registry.host,
                    ssl = false
                )
            )

            httpClient = this@OciProxyTests.client
        }

        // Since `docker push`/`docker pull` require the registry URI
        // to contain '/v2', let's just do that!
        val resp = client.get("/v2/_catalog") {
            header("Accept", "application/json")
        }

        assertEquals(200, resp.status.value)
        assertEquals("OK", resp.status.description)

        // It is safe to assume, if `assertEquals` doesn't throw an error, it means
        // we have done something correctly, I hope.
        //
        // ALSO - It is recommended to set the server to have the ContentNegotiation
        // plugin, or it will not consume the body as `application/json` if needed.
        val data = runBlocking {
            val d = resp.body<ByteReadChannel>()
            val content = ByteArray(20)
            d.readFully(content)

            val json = Json { ignoreUnknownKeys = true }
            json.decodeFromString(RepositoriesResult.serializer(), String(content))
        }

        assertEquals(data.repositories, listOf())
    }
}
