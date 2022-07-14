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

package org.noelware.charted.gradle.plugins

import org.gradle.api.file.RegularFile
import org.gradle.api.plugins.ExtensionContainer
import org.gradle.kotlin.dsl.getByName
import org.noelware.charted.gradle.plugins.docker.ChartedDockerExtension
import org.noelware.charted.gradle.plugins.helm.HelmPluginExtension
import java.io.File

val ExtensionContainer.docker: ChartedDockerExtension?
    get() {
        try {
            return getByName<ChartedDockerExtension>("docker")
        } catch (e: Exception) {
            return null
        }
    }

val ExtensionContainer.helm: HelmPluginExtension?
    get() {
        try {
            return getByName<HelmPluginExtension>("helm")
        } catch (e: Exception) {
            return null
        }
    }

//val ExtensionContainer.go: ChartedGoExtension?
//    get() {
//        try {
//            return getByName<ChartedGoExtension>("go")
//        } catch (e: Exception) {
//            return null
//        }
//    }

fun File.toRegularFile(): RegularFile = RegularFile { this }
