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

package org.noelware.charted.modules.sessions.ldap

import com.unboundid.ldap.sdk.LDAPConnectionPool
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.runTest
import kotlinx.serialization.json.Json
import org.junit.jupiter.api.Test
import org.noelware.charted.common.extensions.reflection.setField
import org.slf4j.LoggerFactory
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.testing.containers.*
import org.testcontainers.containers.BindMode
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.assertj.core.api.Assertions.*
import org.junit.jupiter.api.Disabled
import org.noelware.charted.common.extensions.reflection.getAndUseField

// i have no idea why it does this lol
@Disabled("Caused by: java.lang.UnsatisfiedLinkError: 'int io.netty.channel.unix.ErrorsStaticallyReferencedJniMethods.errorEHOSTUNREACH()'")
@Testcontainers(disabledWithoutDocker = true)
class LdapSessionManagerTest {
    private val config = Config {
        sessions {
            ldap {
                maxConnections = 1
                objectClass = "users"
                organization = ""
                role = ""
                host = ldapContainer.host
                port = ldapContainer.getMappedPort(1389)
            }
        }

        setField("redis", redisContainer.configuration)
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    @Test
    fun `can we connect`(): Unit = runTest {
        val redisClient = DefaultRedisClient(config.redis)
        redisClient.connect()

        val ldapSessionManager = LdapSessionManager(config, redisClient, Json)
        val connectionPool: LDAPConnectionPool = ldapSessionManager.getAndUseField("connectionPool")!!
        assertThat(connectionPool)
            .isNotNull
            .extracting { it.connection }
            .isNotNull
            .extracting { it.isConnected }
            .isEqualTo(true)
    }

    companion object {
        @Container
        private val redisContainer = RedisContainer()

        @Container
        private val ldapContainer: GenericContainer<*> = GenericContainer("osixia/openldap:1.5.0".toImageName()) {
            withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("com.docker.osixia.openldap")))
            withExposedPorts(1389, 1636)
            withEnv(
                mapOf(
                    "LDAP_ADMIN_PASSWORD" to "admin",
                    "LDAP_DOMAIN" to "ldap.charts.noelware.org",
                    "LDAP_BASE_DN" to "DC=ldap,DC=charts,DC=noelware,DC=org",
                    "LDAP_LOG_LEVEL" to "256",
                    "LDAP_ORGANISATION" to "Noelware, LLC.",
                    "LDAP_PORT" to "1389",
                    "LDAPS_PORT" to "1636",
                ),
            )

            withCommand("--copy-service --loglevel debug")
            withClasspathResourceMapping(
                "/10-setup.ldif",
                "/container/service/slapd/assets/config/bootstrap/ldif/custom/10-setup.ldif",
                BindMode.READ_ONLY,
            )
        }
    }
}
