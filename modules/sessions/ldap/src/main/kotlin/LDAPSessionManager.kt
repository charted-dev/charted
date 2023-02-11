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

package org.noelware.charted.modules.sessions.ldap

import dev.floofy.utils.slf4j.logging
import kotlinx.serialization.json.Json
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.Session
import org.noelware.charted.modules.sessions.SessionManager
import java.util.Hashtable
import javax.naming.Context
import javax.naming.NamingException
import javax.naming.directory.InitialDirContext

class LDAPSessionManager(redis: RedisClient, json: Json, private val config: Config) : SessionManager(redis, json, "ldap", config) {
    private val log by logging<LDAPSessionManager>()

    /**
     * Does the actual authentication process with the given [user] and the
     * [password] itself.
     *
     * @param user        The user that was found to authenticate with
     * @param password    The password to do the authentication
     */
    override suspend fun doAuthenticate(user: UserEntity, password: String): Session {
        // Create the environment
        val env = Hashtable<String, String>()
        env[Context.INITIAL_CONTEXT_FACTORY] = "com.sun.jndi.ldap.LdapCtxFactory"
        env[Context.PROVIDER_URL] = "ldap://${config.sessions.ldap!!.host}:${config.sessions.ldap!!.port}"
        env[Context.SECURITY_AUTHENTICATION] = "simple"
        env[Context.SECURITY_PRINCIPAL] = "cn=${user.username},ou=${config.sessions.ldap!!.organizationUnit},${config.sessions.ldap!!.domainComponents.joinToString(",") {
            "dc=$it"
        }}"

        env[Context.SECURITY_CREDENTIALS] = password

        log.info("Now connecting to LDAP server...")
        val context: InitialDirContext = try {
            InitialDirContext(env)
        } catch (e: NamingException) {
            log.error("Unable to connect to LDAP server:", e)
            throw e
        }

        log.info("User with username @${user.username} has successfully been connected to the LDAP server!")
        context.close()

        return create(user.id.value)
    }

    /**
     * Checks if the given [password] is valid or not. This is mainly used for Basic
     * authentication
     *
     * @param user [UserEntity] object
     * @param password The password to check for
     */
    override suspend fun isPasswordValid(user: UserEntity, password: String): Boolean {
        // Create the environment
        val env = Hashtable<String, String>()
        env[Context.INITIAL_CONTEXT_FACTORY] = "com.sun.jndi.ldap.LdapCtxFactory"
        env[Context.PROVIDER_URL] = "ldap://${config.sessions.ldap!!.host}:${config.sessions.ldap!!.port}"
        env[Context.SECURITY_AUTHENTICATION] = "simple"
        env[Context.SECURITY_PRINCIPAL] = "cn=${user.username},ou=${config.sessions.ldap!!.organizationUnit},${config.sessions.ldap!!.domainComponents.joinToString(",") {
            "dc=$it"
        }}"

        env[Context.SECURITY_CREDENTIALS] = password

        return try {
            InitialDirContext(env)
            true
        } catch (e: NamingException) {
            log.error("Unable to connect to LDAP server:", e)
            false
        }
    }
}
