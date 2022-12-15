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

package org.noelware.charted.configuration.kotlin.host

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.configuration.host.ConfigurationHost
import org.noelware.charted.configuration.kotlin.dsl.Config
import java.io.File
import java.nio.file.Files
import kotlin.script.experimental.api.ResultValue
import kotlin.script.experimental.api.ScriptDiagnostic
import kotlin.script.experimental.api.valueOr
import kotlin.script.experimental.host.toScriptSource
import kotlin.script.experimental.jvm.dependenciesFromClassContext
import kotlin.script.experimental.jvm.jvm
import kotlin.script.experimental.jvmhost.BasicJvmScriptingHost
import kotlin.script.experimental.jvmhost.createJvmCompilationConfigurationFromTemplate

object KotlinScriptHost: ConfigurationHost {
    private val log by logging<KotlinScriptHost>()

    override fun load(path: String): Config? = load0(File(path))
    private fun load0(path: File): Config? {
        var realPath = path
        if (!path.exists()) throw IllegalStateException("File '$path' doesn't exist")
        if (!path.isFile) throw IllegalStateException("File '$path' was not a file")

        if (Files.isSymbolicLink(path.toPath())) {
            realPath = Files.readSymbolicLink(path.toPath()).toFile()
            log.info("Path '$path' was a symbolic link that resolved to [$realPath]")
        }

        log.info("Loading Kotlin Script in path [$path]")
        val compilationConfig = createJvmCompilationConfigurationFromTemplate<KotlinConfigHandle> {
            jvm {
                // We shouldn't probably be expanding the whole server's classpath, but it's safe
                // because only the configuration DSL is exposed as a Maven package on Noelware's
                // Maven Repository (maven.noelware.org), so most functions won't exist.
                dependenciesFromClassContext(KotlinScriptHost::class, wholeClasspath = true)
            }
        }

        val result = BasicJvmScriptingHost().eval(realPath.toScriptSource(), compilationConfig, null)
        val handle = result.valueOr {
            val message = buildString {
                appendLine("Unable to run Kotlin Script in path [$realPath]:")
                for (report in it.reports.filter { r -> r.severity == ScriptDiagnostic.Severity.FATAL || r.severity == ScriptDiagnostic.Severity.ERROR }) {
                    appendLine("        * ${report.exception?.toString() ?: report.message} [$path:${report.location?.start?.line ?: 0}]")
                }
            }

            throw RuntimeException(message)
        }

        if (handle.returnValue !is ResultValue.Unit) throw IllegalStateException("Return value must be `Unit`, not anything returned")
        return (handle.returnValue.scriptInstance as? KotlinConfigHandle)?.build()
    }
}
