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

package org.noelware.charted.configuration.kotlin

import org.noelware.charted.configuration.ConfigurationHost
import org.noelware.charted.configuration.dsl.Config
import java.io.File
import kotlin.script.experimental.api.*
import kotlin.script.experimental.host.toScriptSource
import kotlin.script.experimental.jvm.dependenciesFromClassContext
import kotlin.script.experimental.jvm.jvm
import kotlin.script.experimental.jvmhost.BasicJvmScriptingHost
import kotlin.script.experimental.jvmhost.createJvmCompilationConfigurationFromTemplate

object KotlinScriptConfigurationHost: ConfigurationHost {
    /**
     * Loads the configuration in the [path] that is given.
     */
    override fun loadConfig(path: File): Config = load0(path) ?: throw RuntimeException("Unable to define configuration.")
    private fun load0(path: File): Config? {
        val compilationConfig = createJvmCompilationConfigurationFromTemplate<KotlinScriptHandle> {
            jvm {
                // SAFETY: Yes, I know, I used `wholeClasspath` to load the config-{VERSION}.jar file.
                //         But, it's in good reason: the config DSL is only exposed and the other
                //         parts of charted-server isn't published to Noelware's Maven Repository.
                dependenciesFromClassContext(KotlinScriptConfigurationHost::class, wholeClasspath = true)
            }
        }

        val result = BasicJvmScriptingHost().eval(path.toScriptSource(), compilationConfig, null)
        val handle = result.valueOr {
            val message = buildString {
                appendLine("Unable to run script in path [$path]:")

                for (report in it.reports.filter { r -> r.severity == ScriptDiagnostic.Severity.FATAL || r.severity == ScriptDiagnostic.Severity.ERROR }) {
                    appendLine("        * ${report.exception?.toString() ?: report.message} [$path:${report.location?.start?.line ?: 0}]")
                }
            }

            throw RuntimeException(message)
        }

        if (handle.returnValue !is ResultValue.Unit) {
            throw IllegalStateException("Return value must be `Unit`, not anything.")
        }

        return (handle.returnValue.scriptInstance as? KotlinScriptHandle)?.config
    }
}
