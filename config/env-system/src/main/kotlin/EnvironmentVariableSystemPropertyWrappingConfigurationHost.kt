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

package org.noelware.charted.configuration.envSystem

import org.noelware.charted.configuration.ConfigurationHost
import org.noelware.charted.configuration.kotlin.dsl.Config
import java.io.File
import java.util.ServiceLoader
import kotlin.reflect.KClass

/**
 * [ConfigurationHost] to expand a [Config] DSL object with environment variable and system property overwriting. How is it done? Even
 * though I can probably use kotlinx.serialization to do this, I rather not; it collects all the configuration property keys
 * and turns it in to a [ConfigTypeRef] structure, transforms:
 *
 * - environment variables: `CHARTED_CONFIG_<key>` (<key> is SCREAMING_SNAKE_CASE, i.e, `CHARTED_CONFIG_SWAGGER`)
 * - system property: `config.<key>` (<key> is dot.notation, i.e, `config.swagger`)
 *
 * it into the actual property mapped from a [Config] DSL, in which, a
 */
class EnvironmentVariableSystemPropertyWrappingConfigurationHost<Host: ConfigurationHost>(private val host: Host): ConfigurationHost {
    private val transformers: List<Transformer<*>> by lazy {
        ServiceLoader.load(Transformer::class.java).toList()
    }

    override fun load(config: File): Config {
        val refs = loadInRefs()

        return host.load(config)
    }

    private fun loadInRefs(): List<ConfigTypeRef> {
        return emptyList()
    }

    data class ConfigTypeRef(
        val isEnumRef: Boolean = false,
        val isListRef: Boolean = false,
        val isMapRef: Boolean = false,
        val optional: Boolean = true,
        val parent: ConfigTypeRef? = null,
        val type: KClass<*>,
        val name: String
    )
}
