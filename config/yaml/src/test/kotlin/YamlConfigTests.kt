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

package org.noelware.charted.configuration.yaml.tests

import org.junit.jupiter.api.Test
import org.noelware.charted.configuration.yaml.YamlConfigurationHost
import java.io.File
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class YamlConfigTests {
    @Test
    fun `load yaml file`() {
        val yamlFile = this.javaClass.getResource("/testConfig.yaml")
        assertNotNull(yamlFile)

        val file = File(yamlFile.file)
        assertTrue(file.exists())

        val config = YamlConfigurationHost().loadConfig(file)
        assertTrue(config.debug)
    }
}
