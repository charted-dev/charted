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

import dev.floofy.utils.gradle.by

plugins {
    // this will have `java` installed :D
    `charted-java-module`
}

dependencies {
    implementation("guru.zoroark.tegral:tegral-openapi-cli:0.0.3")
    implementation(project(":database"))
    implementation(libs.logback.classic)
    implementation(project(":server"))
    implementation(libs.slf4j.api)
}

val serverProject = project(":server")
val cleanAndCreateDirectories by tasks.registering {
    val openapiFolder = File("${serverProject.projectDir}/build/openapi")
    if (openapiFolder.exists()) openapiFolder.delete()

    openapiFolder.mkdirs()
}

tasks {
    create<JavaExec>("generateOpenAPIJson") {
        dependsOn(cleanAndCreateDirectories)
        classpath = sourceSets["main"].runtimeClasspath

        mainClass by "guru.zoroark.tegral.openapi.cli.MainKt"
        args("$projectDir/server.openapi.kts", "-o", "${serverProject.projectDir}/build/openapi/openapi.json")
    }

    create<JavaExec>("generateOpenAPIYaml") {
        dependsOn(cleanAndCreateDirectories)
        classpath = sourceSets["main"].runtimeClasspath

        mainClass by "guru.zoroark.tegral.openapi.cli.MainKt"
        args("$projectDir/server.openapi.kts", "-f", "yaml", "-o", "${serverProject.projectDir}/build/openapi/openapi.yaml")
    }
}
