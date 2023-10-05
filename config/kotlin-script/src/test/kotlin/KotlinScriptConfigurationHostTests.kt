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

package org.noelware.charted.configuration.kotlin.host

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.junit.jupiter.api.assertThrows
import java.io.File
import java.lang.RuntimeException
import kotlin.test.assertEquals

class KotlinScriptConfigurationHostTests {
    @Test
    fun `should compile correctly`() {
        val test1 = File("src/test/resources/stubs/test1.charted.kts")
        val result = assertDoesNotThrow { KotlinScriptConfigurationHost.load(test1) }

        assertEquals("awau", result.jwtSecretKey)
        assertEquals(12345, result.server.port)
        assertEquals("1.2.3.4", result.server.host)
    }

    @Test
    fun `should error`() {
        val error1 = File("src/test/resources/stubs/error1.charted.kts")
        assertThrows<RuntimeException> { KotlinScriptConfigurationHost.load(error1) }
    }
}