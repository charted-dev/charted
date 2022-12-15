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

package org.noelware.charted.modules.sessions.local.tests

import kotlinx.coroutines.future.await
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import okhttp3.internal.closeQuietly
import org.junit.jupiter.api.Test
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.RedisConfig
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.local.LocalSessionManager
import org.slf4j.LoggerFactory
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName
import kotlin.test.assertEquals

@Testcontainers(disabledWithoutDocker = true)
class LocalSessionManagerTests {
    private fun <T> withRedisConnection(block: suspend (redis: RedisClient, sessionsManager: LocalSessionManager) -> T): T = try {
        val redisClient = DefaultRedisClient(
            RedisConfig(
                host = redisContainer.host,
                port = redisContainer.firstMappedPort
            )
        )

        val result = runBlocking {
            redisClient.connect()
            block(
                redisClient,
                LocalSessionManager(
                    Argon2PasswordEncoder.defaultsForSpringSecurity_v5_8(),
                    redisClient,
                    Json,
                    Config {
                        jwtSecretKey = "blablabla"
                        redis {
                            host = redisContainer.host
                            port = redisContainer.firstMappedPort
                        }
                    }
                )
            )
        }

        redisClient.closeQuietly()
        result
    } catch (e: Exception) {
        throw e
    }

    @Test
    fun `can we connect to Redis container`() = withRedisConnection { redisClient, _ ->
        assertEquals("PONG", redisClient.commands.ping().await())
    }

    companion object {
        // We need a Redis container for the session manager
        @JvmStatic
        @Container
        private val redisContainer: GenericContainer<*> = GenericContainer(DockerImageName.parse("redis:7.0.5-alpine")).apply {
            withExposedPorts(6379)
            withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("com.redis.docker")))
        }
    }
}
