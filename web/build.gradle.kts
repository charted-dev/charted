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

import com.github.gradle.node.npm.task.NpmTask
import dev.floofy.utils.gradle.*

plugins {
    id("com.github.node-gradle.node")
}

node {
    download by (System.getenv("INSTALL_NODE_JS") ?: "yes").matches("^(yes|true|si|si*|1)$".toRegex())
    version by File(projectDir, ".node-version").readText()
}

val build by tasks.registering(NpmTask::class) {
    args.set(listOf("run", "build"))
}

val dev by tasks.registering(NpmTask::class) {
    args.set(listOf("run", "dev"))
}
