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

import dev.floofy.utils.java.SetOnce
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import org.junit.jupiter.api.Test
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.extensions.reflection.setField
import org.noelware.charted.modules.elasticsearch.DefaultElasticsearchModule
import org.noelware.charted.modules.elasticsearch.ElasticsearchModule
import org.noelware.charted.testing.containers.ElasticsearchContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.time.Duration.Companion.seconds

@Testcontainers(disabledWithoutDocker = true)
class ElasticsearchSSLTests {
    private val elasticsearchModule: SetOnce<ElasticsearchModule> = SetOnce()
    private suspend fun withElasticsearch(block: suspend ElasticsearchModule.() -> Unit = {}) {
        if (elasticsearchModule.wasSet()) {
            elasticsearchModule.value.block()
            return
        }

        val module = DefaultElasticsearchModule(
            Config {
                jwtSecretKey = RandomStringGenerator.generate(16)
                search {
                    setField("_elasticsearch", elasticsearchContainer.configuration)
                }
            },
            Json,
        )

        // wait 5 seconds if we need to initialize all indexes
        withContext(Dispatchers.IO) {
            module.connect()
            delay(5.seconds)
        }

        module.block()
    }

    @Test
    fun `can we connect to Elasticsearch`(): Unit = runBlocking {
        withElasticsearch {
            assertEquals("8.6.0", serverVersion)
            assertFalse(closed)
        }
    }

    companion object {
        @Container
        private val elasticsearchContainer: ElasticsearchContainer = ElasticsearchContainer()
    }
}
