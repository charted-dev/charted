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

import com.diffplug.gradle.spotless.SpotlessExtensionPredeclare
import org.noelware.charted.gradle.*
import org.noelware.charted.gradle.util.FindBinaryUtil

buildscript {
    dependencies {
        classpath("org.noelware.charted.gradle:build-logic:0.0.0-devel.0")
    }
}

plugins {
    id("com.diffplug.spotless") version "6.18.0"
    application
}

group = "org.noelware.charted"
version = "$VERSION"
description = "\uD83D\uDCE6 You know, for Helm Charts?"

repositories {
    mavenCentral()
    mavenLocal()
}

spotless {
    // Use the line ending from .gitattributes
    // WINDOWS: You will need to run `git config --global core.autocrlf input` to not have conversions
    //          to take into account.
    lineEndings = com.diffplug.spotless.LineEnding.GIT_ATTRIBUTES

    predeclareDeps()
    encoding("UTF-8")
    format("prettier") {
        target(
            "**/*.json",
            "**/*.yaml",
            "**/*.yml",
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

        prettier(mapOf("prettier" to "2.8.4")).apply {
            configFile(file("$projectDir/.prettierrc.json"))
        }
    }

    kotlinGradle {
        targetExclude(
            "build-logic/build/kotlin-dsl/plugins-blocks/extracted/charted-module.gradle.kts",
        )

        endWithNewline()
        encoding("UTF-8")
        target("**/*.gradle.kts")
        ktlint().apply {
            setUseExperimental(true)
            setEditorConfigPath(file("${rootProject.projectDir}/.editorconfig"))
        }

        licenseHeaderFile(file("${rootProject.projectDir}/assets/HEADING"), "(package |@file|plugins|pluginManagement|import|rootProject)")
    }

    val terraformBinary = FindBinaryUtil.find("terraform")
    if (terraformBinary != null) {
        logger.info("Found Terraform binary in [$terraformBinary], Spotless will be enabled for Terraform files")
        format("terraform") {
            target(".noelware/deployment/**/*.tf", ".noelware/deployment/**/*.tfvars")
            nativeCmd(
                "terraform",
                terraformBinary,
                listOf("fmt", "-"),
            )
        }
    }
}

the<SpotlessExtensionPredeclare>().apply {
    kotlinGradle {
        ktlint()
    }

    kotlin {
        ktlint()
    }

    java {
        googleJavaFormat()
        palantirJavaFormat()
    }
}

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

tasks {
    wrapper {
        distributionType = Wrapper.DistributionType.ALL
    }

    test {
        dependsOn(gradle.includedBuilds.map { it.task(":test") })
        useJUnitPlatform()
    }

    create<Copy>("precommitHook") {
        from(file("${project.rootDir}/scripts/pre-commit"))
        into(file("${project.rootDir}/.git/hooks"))
    }

    named("spotlessCheck") {
        dependsOn(gradle.includedBuilds.map { it.task(":spotlessCheck") })
    }

    named("spotlessApply") {
        dependsOn(gradle.includedBuilds.map { it.task(":spotlessApply") })
    }
}
