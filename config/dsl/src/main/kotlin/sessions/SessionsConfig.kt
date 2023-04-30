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

package org.noelware.charted.configuration.kotlin.dsl.sessions

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.descriptors.element
import kotlinx.serialization.encoding.*
import org.noelware.charted.common.Buildable
import kotlin.properties.Delegates

/**
 * Configuration for configuring the session manager, which is used to authenticate
 * sessions when executing privileged REST calls.
 */
@Serializable(with = SessionsConfig.Serializer::class)
public open class SessionsConfig(public val type: SessionType) {
    // used to trick the serialization compiler
    @Suppress("unused")
    public constructor(): this(SessionType.Local)

    /**
     * Configuration for configuring the local session manager, which manages a pool
     * of sessions that were inserted into Redis and uses the local user management system
     * to authenticate.
     *
     * When used in `PUT /users` (when [`registrations`][org.noelware.charted.configuration.kotlin.dsl.Config.registrations]
     * and [`invite_only`][org.noelware.charted.configuration.kotlin.dsl.Config.inviteOnly] is disabled), the local
     * session manager will create users with a given password. This is also disabled since in some cases, you would
     * want to create and authenticate users with other stuff (like LDAP), so when `PUT /users` is used, it will only
     * create a user object for **charted-server** to use to identify itself, but not for authentication.
     */
    @Serializable
    public object Local: SessionsConfig(SessionType.Local)

    /**
     * Configuration to use an LDAP server to provide authentication. **charted-server** still
     * has some management over pooling sessions and expiring the sessions, but it doesn't
     * control authentication.
     *
     * The `PUT /users` REST controller is disabled, and users are automatically pulled from
     * the specified group it resides in, and will create users based off that.
     *
     * @param abandonOnTimeout If a connection timed out, abandon the connection in the pool
     * @param maxConnections Maximum amount of connections to pool connections over.
     * @param objectClass The object category to filter when searching
     * @param organization Organization name
     * @param keepAlive If `SO_KEEPALIVE` should be enabled on the connection pool
     * @param role The binding role to use to pull new users into the API server. (i.e, `CN=Something,CN=Else`)
     * @param host The host to connect to
     * @param port The port to connect to
     */
    @Serializable
    public data class LDAP(
        @SerialName("abandon-on-timeout")
        val abandonOnTimeout: Boolean = true,

        @SerialName("max_connections")
        val maxConnections: Int = 10,

        @SerialName("object_class")
        val objectClass: String,
        val organization: String,

        @SerialName("keep-alive")
        val keepAlive: Boolean = false,
        val role: String,
        val host: String,
        val port: Int = 389
    ): SessionsConfig(SessionType.LDAP) {
        @Suppress("MemberVisibilityCanBePrivate")
        public class Builder: Buildable<LDAP> {
            /** If a connection timed out, abandon the connection in the pool */
            public var abandonOnTimeout: Boolean = true

            /** Maximum amount of connections to pool connections over. */
            public var maxConnections: Int = 10

            /** The object category to filter when searching */
            public var objectClass: String by Delegates.notNull()

            /** Organization name */
            public var organization: String by Delegates.notNull()

            /** If `SO_KEEPALIVE` should be enabled on the connection pool */
            public var keepAlive: Boolean = false

            /** The binding group to use to pull new users into the API server. */
            public var role: String by Delegates.notNull()

            /** The host to connect to */
            public var host: String by Delegates.notNull()

            /** The port to connect to */
            public var port: Int = 389

            override fun build(): LDAP = LDAP(abandonOnTimeout, maxConnections, objectClass, organization, keepAlive, role, host, port)
        }
    }

    /**
     * Builder for building a [sessions configuration][SessionsConfig] object
     */
    public class Builder: Buildable<SessionsConfig> {
        private var ldapConfig: LDAP? = null

        /**
         * [SessionType] to use when configuring the session manager
         */
        public var type: SessionType = SessionType.Local

        /**
         * Configures the LDAP session manager
         * @param block DSL object for creating the LDAP config
         */
        public fun ldap(block: LDAP.Builder.() -> Unit = {}): Builder {
            type = SessionType.LDAP
            ldapConfig = LDAP.Builder().apply(block).build()

            return this
        }

        override fun build(): SessionsConfig = when (type) {
            SessionType.LDAP -> ldapConfig ?: error("Missing LDAP configuration, did you forget to use the 'ldap {}' block?")
            SessionType.Local -> Local
        }
    }

    internal class Serializer: KSerializer<SessionsConfig> {
        override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.config.SessionsConfig") {
            element<SessionType>("type")
            element<LDAP>("ldap", isOptional = true)
        }

        override fun deserialize(decoder: Decoder): SessionsConfig = decoder.decodeStructure(descriptor) {
            var type: SessionType? = null
            var config: SessionsConfig? = null

            loop@ while (true) {
                when (val index = decodeElementIndex(descriptor)) {
                    CompositeDecoder.DECODE_DONE -> break@loop
                    0 -> {
                        type = decodeSerializableElement(descriptor, index, SessionType.serializer())
                        if (type == SessionType.Local) {
                            config = Local
                            break@loop
                        }
                    }

                    1 -> {
                        assert(type == SessionType.LDAP) {
                            "expected [type: ldap], received [type: $type]"
                        }

                        config = decodeSerializableElement(descriptor, index, LDAP.serializer())
                    }

                    else -> error("Unknown index: $index")
                }
            }

            requireNotNull(type)
            config!!
        }

        override fun serialize(encoder: Encoder, value: SessionsConfig): Unit = encoder.encodeStructure(descriptor) {
            encodeSerializableElement(descriptor, 0, SessionType.serializer(), value.type)
            when (value) {
                is LDAP -> encodeSerializableElement(descriptor, 1, LDAP.serializer(), value)
                else -> {}
            }
        }
    }
}
