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

import dev.floofy.utils.gradle.noel
import dev.floofy.utils.gradle.noelware
import org.noelware.charted.gradle.*

plugins {
    id("com.diffplug.spotless")
    java
}

group = "org.noelware.charted"
version = "$VERSION"

repositories {
    mavenCentral()
    mavenLocal()
    noelware()
    noel()
}

dependencies {
//    // SLF4J
//    api("org.slf4j:slf4j-api:1.7.36")
}

spotless {
    java {
        trimTrailingWhitespace()
        licenseHeaderFile("${rootProject.projectDir}/assets/HEADING")
        removeUnusedImports()
        endWithNewline()
        googleJavaFormat()
    }
}

java {
    sourceCompatibility = JAVA_VERSION
    targetCompatibility = JAVA_VERSION
}
