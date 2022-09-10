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

package org.noelware.charted.search.elasticsearch.tests

// import kotlinx.coroutines.runBlocking
// import kotlinx.serialization.json.Json
// import okhttp3.internal.closeQuietly
// import org.junit.Test
// import org.junit.jupiter.api.Disabled
// import org.junit.jupiter.api.assertThrows
// import org.junit.jupiter.api.condition.DisabledOnOs
// import org.junit.jupiter.api.condition.OS
// import org.noelware.charted.configuration.dsl.Config
// import org.noelware.charted.configuration.dsl.search.ElasticsearchConfig
// import org.noelware.charted.configuration.dsl.search.SearchConfig
// import org.noelware.charted.elasticsearch.DefaultElasticsearchService
// import org.noelware.charted.elasticsearch.ElasticsearchService
// import org.slf4j.LoggerFactory
// import org.testcontainers.containers.output.Slf4jLogConsumer
// import org.testcontainers.elasticsearch.ElasticsearchContainer
// import org.testcontainers.junit.jupiter.Testcontainers
// import org.testcontainers.utility.DockerImageName
// import kotlin.test.assertEquals
// import kotlin.test.assertFalse
//
// @DisabledOnOs(value = [OS.MAC, OS.WINDOWS])
// @Testcontainers(disabledWithoutDocker = true)
// @Disabled("Can't run the Docker image (for now), will need to fix.")
// class ElasticsearchTests {
//    private val container: ElasticsearchContainer = ElasticsearchContainer(DockerImageName.parse("docker.elastic.co/elasticsearch/elasticsearch").withTag("8.4.1"))
//        .apply {
//            withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("co.elastic.elasticsearch.docker")))
//
//            // let elasticsearch consume ~4GB of ram since tests
//            // don't index a lot of data.
//            withEnv("ES_JAVA_OPTS", "-Xms1024m -Xmx4096m")
//        }
//
//    private val client: ElasticsearchService
//        get() = DefaultElasticsearchService(config, Json)
//
//    private val config: Config by lazy {
//        val search = SearchConfig(
//            elastic = ElasticsearchConfig(
//                nodes = listOf("${container.httpHostAddress}:9200")
//            )
//        )
//
//        Config().copy(search = search)
//    }
//
//    private suspend fun runElasticsearchTest(block: suspend ElasticsearchService.() -> Unit) {
//        if (!container.isRunning) container.start()
//
//        try {
//            assertFalse(client.closed)
//            assertThrows<IllegalStateException> { client.serverVersion }
//
//            client.connect()
//            client.block()
//        } finally {
//            client.closeQuietly()
//        }
//    }
//
//    @Test
//    fun `connect to elasticsearch`() = runBlocking {
//        runElasticsearchTest {
//            assertFalse(this.closed)
//            assertEquals("8.4.1", this.serverVersion)
//        }
//    }
// }
