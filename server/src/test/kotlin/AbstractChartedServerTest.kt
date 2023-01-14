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

package org.noelware.charted.server.testing

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthenticationStrategy
import org.noelware.charted.extensions.reflection.setField
import org.noelware.charted.testing.containers.ClickHouseContainer
import org.noelware.charted.testing.containers.ElasticsearchContainer
import org.noelware.charted.testing.containers.MeilisearchContainer
import org.noelware.charted.testing.containers.PostgreSQLContainer
import org.noelware.charted.testing.containers.RedisContainer

/**
 * Represents an abstract test that delegates over [TestChartedServer] and runs the server
 * with the containers that it might require. Since Testcontainers will clean up the containers
 * after the test is done.
 *
 * @param elasticsearch If the [ElasticsearchContainer] should be initialized while this test is running
 * @param meilisearch   If the [MeilisearchContainer] should be initialized while this test is running
 * @param clickhouse    If the [ClickHouseContainer] should be initialized while this test is running
 * @param features      List of enabled server features to use and initialize
 */
open class AbstractChartedServerTest(
    elasticsearch: Boolean = false,
    meilisearch: Boolean = false,
    clickhouse: Boolean = false,

    private val features: List<ServerFeature> = listOf()
) {
    private val _elasticsearchContainer: ElasticsearchContainer? = if (elasticsearch) ElasticsearchContainer() else null
    private val _meilisearchContainer: MeilisearchContainer? = if (meilisearch) MeilisearchContainer() else null
    private val _clickhouseContainer: ClickHouseContainer? = if (clickhouse) ClickHouseContainer() else null
    private val _postgresContainer: PostgreSQLContainer = PostgreSQLContainer()
    private val _redisContainer: RedisContainer = RedisContainer()
    private val log by logging<AbstractChartedServerTest>()

    /**
     * Represents the configuration that is used for the test server.
     */
    private val config: Config by lazy {
        Config {
            jwtSecretKey = RandomStringGenerator.generate(16)

            for (feature in features) {
                this.feature(feature)
            }

            storage {
                filesystem("./.data")
            }

            if (_elasticsearchContainer != null) {
                search {
                    elasticsearch {
                        node(_elasticsearchContainer.host, _elasticsearchContainer.getMappedPort(9200))
                        auth(AuthenticationStrategy.None)
                    }
                }
            }

            if (_meilisearchContainer != null) {
                search {
                    meilisearch {
                        endpoint = "http://${_meilisearchContainer.host}:${_meilisearchContainer.getMappedPort(7700)}"
                        masterKey = _meilisearchContainer.masterKey
                    }
                }
            }

            if (_clickhouseContainer != null) {
                setField("_clickhouse", _clickhouseContainer.configuration)
            }

            setField("_database", _postgresContainer.configuration)
            setField("_redis", _redisContainer.configuration)
        }
    }

    init {
        log.info("Starting all containers...")

        _redisContainer.start()
        _postgresContainer.start()
        _elasticsearchContainer?.start()
        _clickhouseContainer?.start()
        _meilisearchContainer?.start()
    }

    fun withChartedServer(func: ServerTestFunction = {}) {
        val server = TestChartedServer(config, func)
        server.start()
    }
}
