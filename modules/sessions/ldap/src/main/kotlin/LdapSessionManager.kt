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

import com.unboundid.ldap.sdk.*
import dev.floofy.utils.slf4j.logging
import kotlinx.serialization.json.Json
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionsConfig
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.snowflake.Snowflake
import java.net.InetAddress

class LdapSessionManager(
    private val snowflake: Snowflake,
    config: Config,
    redis: RedisClient,
    json: Json
): AbstractSessionManager("ldap", config, json, redis) {
    // SAFETY: it is validated in the configure.modules bootstrap phase.
    private val ldapConfig: SessionsConfig.LDAP = config.sessions as SessionsConfig.LDAP
    private val log by logging<LdapSessionManager>()

    private val connectionPool: LDAPConnectionPool
    init {
        val connection = LDAPConnection(
            LDAPConnectionOptions().apply {
                setAbandonOnTimeout(ldapConfig.abandonOnTimeout)
                setUseKeepAlive(ldapConfig.keepAlive)
                setUseSynchronousMode(false)

                connectionLogger = Logger
            },
        )

        connectionPool = LDAPConnectionPool(connection, ldapConfig.maxConnections)
        log.info("Established connection pool!")
    }

    override suspend fun doAuthenticate(user: UserEntity, password: String): Session {
        TODO("Not yet implemented")
    }

    override suspend fun isPasswordValid(user: UserEntity, password: String): Boolean {
        TODO("Not yet implemented")
    }

    override fun close() {
        connectionPool.close(true, 1)
        super.close()
    }

    internal object Logger: LDAPConnectionLogger() {
        private val log by logging<LDAPConnectionLogger>()
        override fun logConnect(
            connectionInfo: LDAPConnectionInfo,
            host: String,
            inetAddress: InetAddress,
            port: Int
        ) {
            log.info("Connected to LDAP server [${connectionInfo.hostPort} (${connectionInfo.connectionID})]")
        }

        override fun logConnectFailure(
            connectionInfo: LDAPConnectionInfo,
            host: String,
            port: Int,
            connectException: LDAPException
        ) {
            log.error("Unable to connect to LDAP server [${connectionInfo.hostPort} (${connectionInfo.connectionID})]", connectException)
        }
    }
}
