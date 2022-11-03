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
import dev.floofy.utils.slf4j.logging
import org.noelware.charted.configuration.host.ConfigurationHost
import org.noelware.charted.configuration.kotlin.dsl.Config
import java.io.File
import java.nio.file.Files

class YamlConfigurationHost(private val yaml: Yaml = Yaml.default): ConfigurationHost {
    private val log by logging<YamlConfigurationHost>()
    override fun load(path: String): Config? {
        var realPath = File(path)
        if (!realPath.exists()) throw IllegalStateException("File '$path' doesn't exist")
        if (!realPath.isFile) throw IllegalStateException("File '$path' was not a file")

        if (Files.isSymbolicLink(realPath.toPath())) {
            realPath = Files.readSymbolicLink(realPath.toPath()).toFile()
            log.info("Path '$path' was a symbolic link that resolved to [$realPath]")
        }

        log.info("Loading YAML file from path [$path]")
        return yaml.decodeFromStream(realPath.inputStream())
    }
}
