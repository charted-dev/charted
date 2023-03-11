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

package org.noelware.charted.gradle

import com.diffplug.gradle.spotless.SpotlessExtension
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure

fun Project.applyRootProjectSpotless() {
    if (parent != null) throw IllegalStateException("This can only be applied to the root project")

    configure<SpotlessExtension> {
        predeclareDeps()
        encoding("UTF-8")
        format("prettier") {
            target(
                "**/*.json",
                "**/*.yaml",
                "**/*.yml",
                "**/*.xml",
                "**/*.md",
            )

            // Exclude all Helm templates, .idea files, and submodule
            // in modules/emails/vendor
            targetExclude(
                "modules/emails/vendor/protos/.github/workflows/*.yaml",
                "modules/emails/vendor/protos/.devcontainer/*.json",
                "modules/emails/vendor/protos/.vscode/*.json",
                "modules/emails/vendor/protos/.github/*.md",
                "modules/emails/vendor/protos/*.json",
                "modules/emails/vendor/protos/*.yaml",
                "modules/emails/vendor/protos/*.yml",
                "modules/emails/vendor/protos/*.xml",
                "modules/emails/vendor/protos/*.md",

                "distribution/chart/templates/configmap/*.yml",
                "distribution/chart/templates/server/*.yml",
                "distribution/chart/templates/*.yaml",
                "distribution/chart/templates/*.yml",

                ".idea/inspectionProfiles/*.xml",
                ".idea/*.xml",
            )

            prettier(mapOf("prettier" to "2.8.4", "@prettier/plugin-xml" to "2.2.0")).apply {
                configFile(file("$projectDir/.prettierrc.json"))
            }
        }

        kotlin {
            endWithNewline()
            encoding("UTF-8")
            target("**/*.main.kts")
            ktlint().apply {
                setUseExperimental(true)
                setEditorConfigPath(file("${rootProject.projectDir}/.editorconfig"))
            }

            licenseHeaderFile(file("${rootProject.projectDir}/assets/HEADING"))
        }

        kotlinGradle {
            targetExclude(
                "buildSrc/build/kotlin-dsl/plugins-blocks/extracted/charted-module.gradle.kts",
            )

            endWithNewline()
            encoding("UTF-8")
            target("**/*.gradle.kts")
            ktlint().apply {
                setUseExperimental(true)
                setEditorConfigPath(file("${rootProject.projectDir}/.editorconfig"))
            }

            licenseHeaderFile(file("${rootProject.projectDir}/assets/HEADING"), "(package |@file|plugins|pluginManagement|import)")
        }
    }
}

fun Project.applySpotless() {
    configure<SpotlessExtension> {
        // For Kotlin and Kotlin (Gradle), we will need to move the license header
        // as the last step due to https://github.com/diffplug/spotless/issues/1599
        kotlin {
            targetExclude("**/*.charted.kts")
            endWithNewline()
            encoding("UTF-8")
            target("**/*.kt")
            ktlint().apply {
                setUseExperimental(true)
                setEditorConfigPath(file("${rootProject.projectDir}/.editorconfig"))
            }

            licenseHeaderFile(file("${rootProject.projectDir}/assets/HEADING"))
        }

        java {
            licenseHeaderFile(file("${rootProject.projectDir}/assets/HEADING"))
            trimTrailingWhitespace()
            removeUnusedImports()
            palantirJavaFormat()
            endWithNewline()
            encoding("UTF-8")
        }
    }
}
