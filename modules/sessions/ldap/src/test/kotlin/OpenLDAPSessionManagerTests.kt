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

import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionType
import org.noelware.charted.extensions.reflection.setField
import org.noelware.charted.testing.containers.RedisContainer
import org.testcontainers.containers.GenericContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.utility.DockerImageName
import org.testcontainers.utility.MountableFile

@Testcontainers(disabledWithoutDocker = true)
class OpenLDAPSessionManagerTests {
    private val config = Config {
        jwtSecretKey = RandomStringGenerator.generate(16)
        sessions {
            type = SessionType.LDAP
        }

        setField("_redis", redisContainer.configuration)
    }

    companion object {
        @JvmStatic
        @Container
        private val redisContainer = RedisContainer()

        @JvmStatic
        @Container
        private val openldapContainer = GenericContainer(DockerImageName.parse("osixia/openldap").withTag("1.5.0")).apply {
            withExposedPorts(389, 636)
            withEnv(
                mapOf(
                    "LDAP_ORGANISATION" to "Noelware, LLC.",
                    "LDAP_DOMAIN" to "ad.charts.noelware.org",
                    "LDAP_ADMIN_PASSWORD" to "BoelIsAMenacePleaseHelpUsWeNeedHelp",
                ),
            )

            withCopyFileToContainer(MountableFile.forClasspathResource("/users.ldif"), "/container/service/slapd/assets/config/bootstrap/ldif/50-users.ldif")
        }
    }
}
