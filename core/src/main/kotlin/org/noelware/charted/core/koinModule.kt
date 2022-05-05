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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core

import com.charleskorn.kaml.Yaml
import kotlinx.serialization.json.Json
import org.koin.dsl.module
import org.noelware.charted.core.config.Config
import java.io.File
import java.nio.charset.Charset

val chartedModule = module {
    single {
        val configPath = System.getenv("CHARTED_CONFIG_PATH") ?: "./config.yml"
        val configFile = File(configPath)

        if (!configFile.exists())
            throw IllegalStateException("File $configPath must exist on the disk")

        Yaml.default.decodeFromString(Config.serializer(), configFile.readText(Charset.defaultCharset()))
    }

    single {
        Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }
    }
}
