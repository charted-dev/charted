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

import org.gradle.api.tasks.testing.logging.TestExceptionFormat
import org.gradle.api.tasks.testing.logging.TestLogEvent
import org.noelware.charted.gradle.*
import dev.floofy.utils.gradle.*
import gradle.kotlin.dsl.accessors._657ac3b2e5072282e18494eb2b5fc9d6.testImplementation

plugins {
    id("com.diffplug.spotless")
    java
}

group = "org.noelware.charted"
version = "$VERSION"

repositories {
    maven("https://repo.perfectdreams.net/")
    mavenCentral()
    mavenLocal()
    noelware()
    noel()
}

dependencies {
    implementation("org.jetbrains:annotations:20.1.0")

    if (name != "common") {
        implementation(project(":common"))
    }
}

spotless {
    java {
        licenseHeaderFile("${rootProject.projectDir}/assets/HEADING")
        trimTrailingWhitespace()
        removeUnusedImports()
        palantirJavaFormat()
        endWithNewline()
    }
}

java {
    sourceCompatibility = JAVA_VERSION
    targetCompatibility = JAVA_VERSION
}
