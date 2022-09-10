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

package org.noelware.charted.configuration.yaml

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.decodeFromStream
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule
import org.noelware.charted.configuration.ConfigurationHost
import org.noelware.charted.configuration.dsl.Config
import java.io.File

class YamlConfigurationHost(private val yaml: Yaml = Yaml(DefaultSerializersModule)): ConfigurationHost {
    override fun loadConfig(path: File): Config {
        if (!listOf("yaml", "yml").contains(path.extension)) {
            throw IllegalStateException("YAML file must have the .yml or .yaml extensions.")
        }

        return path.inputStream().use { yaml.decodeFromStream(it) }
    }

    companion object {
        /**
         * Default serializers module for the YAML configuration host. This can be used
         * to merge serializer modules (if needed).
         */
        val DefaultSerializersModule: SerializersModule = EmptySerializersModule()
    }
}
