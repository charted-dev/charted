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

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.configuration.ConfigurationHost
import org.noelware.charted.configuration.kotlin.dsl.Config
import java.io.File
import java.nio.file.Files
import kotlin.script.experimental.api.ScriptDiagnostic
import kotlin.script.experimental.api.valueOr
import kotlin.script.experimental.host.toScriptSource
import kotlin.script.experimental.jvm.dependenciesFromClassContext
import kotlin.script.experimental.jvm.jvm
import kotlin.script.experimental.jvmhost.BasicJvmScriptingHost
import kotlin.script.experimental.jvmhost.createJvmCompilationConfigurationFromTemplate

private val log by logging<KotlinScriptConfigurationHost>()

private fun compileScript(file: File): Config {
    log.info("Compiling Kotlin script in path [$file]")
    val config = createJvmCompilationConfigurationFromTemplate<KotlinScriptHandle> {
        jvm {
            // You might be able to access all the members, but you wouldn't be able to
            // mess with anything when the server is bootstrapping.
            dependenciesFromClassContext(KotlinScriptHandle::class, wholeClasspath = true)
        }
    }

    val result = BasicJvmScriptingHost().eval(file.toScriptSource(), config, null)
    val handle = result.valueOr {
        val message = buildString {
            appendLine("Unable to compile Kotlin Script in path [$file]:")
            for (report in it.reports.filter { r -> r.severity == ScriptDiagnostic.Severity.FATAL || r.severity == ScriptDiagnostic.Severity.ERROR }) {
                appendLine("        * ${report.exception?.toString() ?: report.message} [$file:${report.location?.start?.line ?: 0}]")
            }
        }

        throw RuntimeException(message)
    }

    return (handle.returnValue.scriptInstance as KotlinScriptHandle).build()
}

object KotlinScriptConfigurationHost: ConfigurationHost {
    override fun load(config: File): Config {
        var realPath = config
        if (!config.exists()) throw IllegalStateException("Configuration file at path [$config] doesn't exist")
        if (!config.isFile) throw IllegalStateException("Configuration in path [$config] is not a file")

        if (Files.isSymbolicLink(config.toPath())) {
            realPath = Files.readSymbolicLink(config.toPath()).toFile()
            log.info("Path '$config' was a symbolic link that resolved to [$realPath]")
        }

        return compileScript(realPath)
    }
}
