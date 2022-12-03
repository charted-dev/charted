/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.serializers.SecretStringSerializer
import kotlin.properties.Delegates

@Serializable
data class LDAPSessionConfig(
    @SerialName("organization_unit")
    val organizationUnit: String,

    @SerialName("domain_components")
    val domainComponents: List<String> = listOf(),

    @Serializable(with = SecretStringSerializer::class)
    val credentials: String,

    @Serializable(with = SecretStringSerializer::class)
    val host: String,
    val port: Int
) {
    class Builder: org.noelware.charted.common.Builder<LDAPSessionConfig> {
        val domainComponents: MutableList<String> = mutableListOf()
        var organizationUnit: String by Delegates.notNull()
        var credentials: String by Delegates.notNull()
        var port: Int by Delegates.notNull()
        var host: String by Delegates.notNull()

        override fun build(): LDAPSessionConfig = LDAPSessionConfig(organizationUnit, domainComponents, credentials, host, port)
    }
}
