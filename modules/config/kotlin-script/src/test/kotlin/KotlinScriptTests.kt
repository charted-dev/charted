/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.configuration.kotlin.host.tests

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertDoesNotThrow
import org.noelware.charted.configuration.kotlin.dsl.DatabaseConfig
import org.noelware.charted.configuration.kotlin.dsl.KtorSSLConfig
import org.noelware.charted.configuration.kotlin.host.KotlinConfigHandle
import org.noelware.charted.configuration.kotlin.host.KotlinScriptHost
import kotlin.script.experimental.api.ResultValue
import kotlin.script.experimental.api.ScriptDiagnostic
import kotlin.script.experimental.api.valueOr
import kotlin.script.experimental.host.toScriptSource
import kotlin.script.experimental.jvm.dependenciesFromClassContext
import kotlin.script.experimental.jvm.jvm
import kotlin.script.experimental.jvmhost.BasicJvmScriptingHost
import kotlin.script.experimental.jvmhost.createJvmCompilationConfigurationFromTemplate
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals
import kotlin.test.assertNotNull

class KotlinScriptTests {
    private inline fun <reified T> runScript(code: String): T? {
        val compilationConfig = createJvmCompilationConfigurationFromTemplate<KotlinConfigHandle> {
            jvm {
                // We shouldn't probably be expanding the whole server's classpath, but it's safe
                // because only the configuration DSL is exposed as a Maven package on Noelware's
                // Maven Repository (maven.noelware.org), so most functions won't exist.
                dependenciesFromClassContext(KotlinScriptHost::class, wholeClasspath = true)
            }
        }

        val result = BasicJvmScriptingHost().eval(code.toScriptSource("test${System.nanoTime()}.charted.kts"), compilationConfig, null)
        val handle = result.valueOr {
            val message = buildString {
                appendLine("Unable to run Kotlin Script:")
                for (report in it.reports.filter { r -> r.severity == ScriptDiagnostic.Severity.FATAL || r.severity == ScriptDiagnostic.Severity.ERROR }) {
                    appendLine("        * ${report.exception?.toString() ?: report.message}")
                }
            }

            throw RuntimeException(message)
        }

        assert(handle.returnValue !is ResultValue.Unit) { "Must not return `Unit`, received ${handle.returnValue}" }
        return handle.returnValue.scriptInstance as? T
    }

    @Test
    fun `will it run`() {
        val result = assertDoesNotThrow {
            runScript<KotlinConfigHandle>(
                """
            |jwtSecretKey = "woah"
            |debug = true
            |server {
            |  host = "0.0.0.0"
            |  port = 7272
            |
            |  ssl {
            |    keystore = "awa.jks"
            |    password = "iceiscutieuwu"
            |  }
            |}
            """.trimMargin("|")
            )
        }

        assertNotNull(result)

        val config = result.build()
        assertNotEquals(KtorSSLConfig(), config.server.ssl)
        assertEquals(DatabaseConfig(), config.database)
    }
}
