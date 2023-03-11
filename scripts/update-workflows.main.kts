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

@file:DependsOn("net.pearx.kasechange:kasechange-jvm:1.3.0")

import net.pearx.kasechange.toPascalCase
import java.io.File
import java.nio.charset.Charset

val licenseHeader = """
|# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
|# Copyright 2022-2023 Noelware <team@noelware.org>
|#
|# Licensed under the Apache License, Version 2.0 (the "License");
|# you may not use this file except in compliance with the License.
|# You may obtain a copy of the License at
|#
|#    http://www.apache.org/licenses/LICENSE-2.0
|#
|# Unless required by applicable law or agreed to in writing, software
|# distributed under the License is distributed on an "AS IS" BASIS,
|# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
|# See the License for the specific language governing permissions and
|# limitations under the License.
|
""".trimMargin("|")

val actionsDir: Array<File> = File(".github/actions").listFiles { f -> f.extension == "kts" } ?: arrayOf()
for (file in actionsDir) {
    // Convert from snake-case -> PascalCase
    val workflowName = ".github/workflows/${file.name.toPascalCase().replace("MainKts", "")}.yaml"

    println("Updating workflow from file [$file] ~> $workflowName")

    // Now, we need to execute the command
    val proc = ProcessBuilder("kotlin", ".github/actions/${file.name}").start()
    val generatedContent = String(proc.inputStream.readBytes())
    val workflowFile = File(workflowName)

    workflowFile.writeText(licenseHeader + "\n" + generatedContent.trim(), Charset.defaultCharset())
    println("Updated workflow [$workflowName] successfully!")
}
