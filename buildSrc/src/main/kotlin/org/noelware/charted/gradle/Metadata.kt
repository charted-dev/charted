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

package org.noelware.charted.gradle

import dev.floofy.utils.gradle.ReleaseType
import dev.floofy.utils.gradle.Version
import org.gradle.api.JavaVersion
import java.io.File
import java.io.IOException
import java.util.concurrent.TimeUnit

val VERSION = Version(1, 1, 1, 0, ReleaseType("nightly"))
val JAVA_VERSION = JavaVersion.VERSION_17
val COMMIT_HASH by lazy {
    try {
        val cmd = "git rev-parse --short HEAD".split("\\s".toRegex())
        val proc = ProcessBuilder(cmd)
            .directory(File("."))
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .start()

        proc.waitFor(1, TimeUnit.MINUTES)
        proc.inputStream.bufferedReader().readText().trim()
    } catch (_: IOException) {
        "<unknown>"
    }
}
