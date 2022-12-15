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

package org.noelware.charted.modules.elasticsearch.tests

import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import org.junit.jupiter.api.Disabled
import org.junit.jupiter.api.Test
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.elasticsearch.DefaultElasticsearchModule
import org.slf4j.LoggerFactory
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.elasticsearch.ElasticsearchContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName
import org.testcontainers.utility.MountableFile
import kotlin.io.path.Path
import kotlin.test.assertEquals
import kotlin.test.assertFalse

@Testcontainers(disabledWithoutDocker = true)
@Disabled("SSL works with the right certificates, not in CI")
class ElasticsearchSSLTests {
    @Test
    fun `can the module be initialized`(): Unit = runBlocking {
        val config = Config {
            jwtSecretKey = RandomStringGenerator.generate(16)
            search {
                elasticsearch {
                    node("https://${elasticsearchContainer.host}:${elasticsearchContainer.getMappedPort(9200)}")
                    ssl {
                        caPath = Path("src/test/resources/certs/ca.crt").toRealPath().toString()
                        validateHostnames = false
                    }
                }
            }
        }

        val elasticsearch = DefaultElasticsearchModule(config, Json)
        elasticsearch.connect()

        assertFalse(elasticsearch.closed)
        assertEquals(elasticsearch.serverVersion, "8.5.2")
    }

    companion object {
        @JvmStatic
        @Container
        internal val elasticsearchContainer: ElasticsearchContainer = ElasticsearchContainer(DockerImageName.parse("docker.elastic.co/elasticsearch/elasticsearch:8.5.2")).apply {
            withCertPath("/usr/share/elasticsearch/config/certs")
            withCopyFileToContainer(MountableFile.forClasspathResource("/elasticsearch.ssl.yml"), "/usr/share/elasticsearch/config/elasticsearch.yml")
            withFileSystemBind(Path("src/test/resources/certs").toRealPath().toString(), "/usr/share/elasticsearch/config/certs")
            withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("co.elastic.elasticsearch.docker")))
        }
    }
}
