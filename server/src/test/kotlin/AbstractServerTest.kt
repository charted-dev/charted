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

package org.noelware.charted.server.tests

import io.ktor.server.application.*
import io.ktor.server.testing.*
import org.junit.jupiter.api.condition.DisabledOnOs
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthenticationStrategy
import org.slf4j.LoggerFactory
import org.testcontainers.containers.DockerComposeContainer
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import java.io.File

@Testcontainers(disabledWithoutDocker = true)
@DisabledOnOs(architectures = ["aarch64", "arm64"], disabledReason = "docker/compose Docker image doesn't support ARM")
open class AbstractServerTest {
    fun withChartedServer(module: Application.() -> Unit = {}, testFunction: suspend ApplicationTestBuilder.() -> Unit) = withChartedServer(
        Config {
            jwtSecretKey = RandomStringGenerator.generate(32)
            metrics {
                enabled = false
            }

            if (includeElasticsearch) {
                search {
                    elasticsearch {
                        node(dockerComposeContainer.getServiceHost("elasticsearch_1", 9200), dockerComposeContainer.getServicePort("elasticsearch_1", 9200))
                        auth(AuthenticationStrategy.None)
                    }
                }
            }

            if (includeClickHouse) {
                clickhouse {
                    database = "default"
                    username = "charted"
                    password = "charted"
                    host = dockerComposeContainer.getServiceHost("clickhouse_1", 8123)
                    port = dockerComposeContainer.getServicePort("clickhouse_1", 8123)
                }
            }

            database {
                database = "charted"
                username = "charted"
                password = "charted"
                host = dockerComposeContainer.getServiceHost("postgres_1", 5432)
                port = dockerComposeContainer.getServicePort("postgres_1", 5432)
            }

            redis {
                index = 2
                host = dockerComposeContainer.getServiceHost("redis_1", 6379)
                port = dockerComposeContainer.getServicePort("redis_1", 6379)
            }

            storage {
                filesystem("./.charted/data")
            }
        },
        module, testFunction
    )

    companion object {
        internal var includeElasticsearch: Boolean = false
        internal var includeMeilisearch: Boolean = false
        internal var includeClickHouse: Boolean = false
        internal var logging: Boolean = true

        @JvmStatic
        @Container
        internal val dockerComposeContainer: DockerComposeContainer<*> = DockerComposeContainer(File("src/test/resources/docker-compose.yml")).apply {
            val modules = mutableListOf(
                "postgres_1" to 5432,
                "redis_1" to 6379
            )

            if (includeElasticsearch) {
                modules.add("elasticsearch_1" to 9200)
            }

            if (includeMeilisearch) {
                modules.add("meilisearch_1" to 7700)
            }

            if (includeClickHouse) {
                modules.add("clickhouse_1" to 8123)
            }

            for ((module, port) in modules) {
                withExposedService(module, port)
                if (logging) {
                    withLogConsumer(module, Slf4jLogConsumer(LoggerFactory.getLogger("org.noelware.charted.server.tests.${module.replace("_1", "")}")))
                }
            }
        }
    }
}
