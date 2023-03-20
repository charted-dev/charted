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

package org.noelware.charted.server

import io.ktor.server.application.*
import org.noelware.charted.common.extensions.reflection.setField
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.Feature
import org.noelware.charted.testing.containers.PostgreSQLContainer
import org.noelware.charted.testing.containers.RedisContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.contracts.ExperimentalContracts
import kotlin.contracts.InvocationKind
import kotlin.contracts.contract

@Testcontainers(disabledWithoutDocker = true)
abstract class AbstractServerTest(
    registrations: Boolean = true,
    isInviteOnly: Boolean = false,
    elasticearch: Boolean = false,
    meilisearch: Boolean = false,
    features: List<Feature> = listOf()
) {
    private val config: Config by lazy {
        Config {
            this.registrations = registrations
            inviteOnly = isInviteOnly

            setField("database", postgresContainer.configuration)
            setField("features", features)
            setField("redis", redisContainer.configuration)

            storage {
                filesystem("./.data")
            }
        }
    }

    @OptIn(ExperimentalContracts::class)
    fun withServer(appModule: (Application.() -> Unit)? = null, testFunction: ServerTestFunction) {
        contract { callsInPlace(testFunction, InvocationKind.EXACTLY_ONCE) }

        val server = TestServer(config, testFunction, appModule)
        server.start()
    }

    companion object {
        @Container
        private val postgresContainer: PostgreSQLContainer = PostgreSQLContainer()

        @Container
        private val redisContainer: RedisContainer = RedisContainer()
    }
}
