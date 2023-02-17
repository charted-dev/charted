/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

import org.gradle.api.tasks.testing.logging.TestExceptionFormat
import org.gradle.api.tasks.testing.logging.TestLogEvent
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.noelware.infra.gradle.Licenses
import org.noelware.charted.gradle.*
import dev.floofy.utils.gradle.*

plugins {
    id("org.noelware.gradle.kotlin")
    id("org.noelware.gradle.java")
    id("kotlinx-atomicfu")
}

// https://github.com/Kotlin/kotlinx-atomicfu/issues/210
atomicfu {
    val libs = extensions.getByType<VersionCatalogsExtension>().named("libs")
    dependenciesVersion = libs.findVersion("kotlinx-atomicfu").get().requiredVersion
}

group = "org.noelware.charted"
version = "$VERSION"
description = "\uD83D\uDCE6 You know, for Helm Charts? [Subproject $name]"

noelware {
    minimumJavaVersion by JAVA_VERSION
    projectDescription by "Free, open source, and reliable Helm Chart registry made in Kotlin."
    projectEmoji by "\uD83D\uDCE6"
    projectName by "charted-server"
    currentYear by "2022-2023"
    unitTests by true
    license by Licenses.APACHE
}

repositories {
    maven("https://repo.perfectdreams.net/")
    mavenCentral()
    mavenLocal()
    noelware()
    noel()
}

dependencies {
    // Kotlin libraries
    implementation(kotlin("reflect"))
    implementation(kotlin("stdlib"))

    // Java Annotations (only for Java usage)
    implementation("org.jetbrains:annotations:24.0.0")

    // Test Dependencies
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.9.2")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.9.2")
    testImplementation("org.testcontainers:testcontainers:1.17.6")
    testImplementation("org.testcontainers:junit-jupiter:1.17.6")
    testImplementation("org.slf4j:slf4j-simple:2.0.6")
    testImplementation(project(":test:containers"))
    testImplementation(kotlin("test"))

    if (name != "common") {
        implementation(project(":common"))
        if (path != ":modules:config:dsl") {
            implementation(project(":modules:config:dsl"))
        }
    }
}

// This will transform the project path:
//
//    - :sessions -> sessions
//    - :modules:elasticsearch -> elasticsearch
//    - :modules:tracing:apm -> tracing-apm
//    - :sessions:integrations:noelware -> sessions-integrations-noelware
val projectName = path
    .substring(1)
    .replace(':', '-')
    .replace("modules-", "")

tasks {
    withType<Jar> {
        archiveFileName by "charted-$projectName-$VERSION.jar"
        manifest {
            attributes(
                mapOf(
                    "Implementation-Version" to "$VERSION",
                    "Implementation-Vendor" to "Noelware, LLC. [team@noelware.org]",
                    "Implementation-Title" to "charted-server"
                )
            )
        }
    }
}
