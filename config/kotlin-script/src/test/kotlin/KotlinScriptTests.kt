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

package org.noelware.charted.configuration.kotlin.tests

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.noelware.charted.configuration.kotlin.KotlinScriptConfigurationHost
import org.noelware.charted.configuration.kotlin.KotlinScriptHandle
import kotlin.script.experimental.api.valueOrThrow
import kotlin.script.experimental.host.toScriptSource
import kotlin.script.experimental.jvm.dependenciesFromClassContext
import kotlin.script.experimental.jvm.jvm
import kotlin.script.experimental.jvmhost.BasicJvmScriptingHost
import kotlin.script.experimental.jvmhost.createJvmCompilationConfigurationFromTemplate
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

class KotlinScriptTests {
    @Test
    fun `check if source code can be compiled`() {
        val compilationConfig = createJvmCompilationConfigurationFromTemplate<KotlinScriptHandle> {
            jvm {
                dependenciesFromClassContext(KotlinScriptConfigurationHost::class, wholeClasspath = true)
            }
        }

        val result = BasicJvmScriptingHost().eval(
            """
        |debug = true
        |metrics = true
            """.trimMargin().toScriptSource("main.charted.kts"),
            compilationConfig,
            null
        )

        val res = assertDoesNotThrow { result.valueOrThrow() }
        val handle = (res.returnValue.scriptInstance as? KotlinScriptHandle)
        assertNotNull(handle)
        assertTrue(handle.debug)
        assertFalse(handle.inviteOnly)
    }
}
