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

package org.noelware.charted.configuration.kotlin.dsl.sessions

import kotlinx.serialization.Serializable

@Serializable
public data class SessionsConfig(
    val ldap: LDAPSessionConfig? = null,
    val type: SessionType = SessionType.Local
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder : org.noelware.charted.common.Builder<SessionsConfig> {
        private var _ldap: LDAPSessionConfig? = null
        public var type: SessionType = SessionType.Local

        public fun ldap(builder: LDAPSessionConfig.Builder.() -> Unit = {}): Builder {
            _ldap = LDAPSessionConfig.Builder().apply(builder).build()
            return this
        }

        override fun build(): SessionsConfig = SessionsConfig(_ldap, type)
    }
}
