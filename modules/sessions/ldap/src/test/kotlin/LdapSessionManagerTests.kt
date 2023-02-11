/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.testing.sessions.ldap

import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import org.jetbrains.exposed.dao.id.EntityID
import org.junit.jupiter.api.Disabled
import org.junit.jupiter.api.Test
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.RedisConfig
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.ldap.LDAPSessionManager
import org.testcontainers.containers.GenericContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName

@Testcontainers(disabledWithoutDocker = true)
@Disabled
class LdapSessionManagerTests {
    private val redisClient: RedisClient by lazy {
        DefaultRedisClient(
            RedisConfig {
                host = redisContainer.host
                port = redisContainer.getMappedPort(6379)
            },
        )
    }

    private val ldapSessionManager: LDAPSessionManager by lazy {
        LDAPSessionManager(
            redisClient, Json,
            Config {
                jwtSecretKey = RandomStringGenerator.generate(16)

                sessions {
                    ldap {
                        organizationUnit = ""
                        domainComponents += listOf("dc=noelware,dc=dev")
                        host = ldapContainer.host
                        port = ldapContainer.getMappedPort(1389)
                    }
                }
            },
        )
    }

    @Test
    fun `can we launch openldap`(): Unit = runBlocking {
        ldapSessionManager.isPasswordValid(
            UserEntity(EntityID(0L, UserTable)).apply {
                username = "boel"
            },
            "1234",
        )
    }

    companion object {
        @Container
        @JvmStatic
        private val ldapContainer: GenericContainer<*> = GenericContainer(DockerImageName.parse("bitnami/openldap:2.6")).apply {
            withExposedPorts(1389, 1636)
            withEnv(
                mapOf(
                    "LDAP_ADMIN_USERNAME" to "admin",
                    "LDAP_ADMIN_PASSWORD" to "admin",
                    "LDAP_USERS" to "noel,test2",
                    "LDAP_PASSWORDS" to "noeliscutieuwu,owodauwu",
                    "LDAP_ROOT" to "dc=noelware,dc=dev",
                ),
            )
        } // LDAP_ROOT

        @Container
        @JvmStatic
        private val redisContainer: GenericContainer<*> = GenericContainer(DockerImageName.parse("bitnami/redis:7.0.7")).apply {
            withExposedPorts(6379)
            withEnv(
                mapOf(
                    "REDIS_PASSWORD" to "bestpasswordever",
                ),
            )
        }
    }
}
