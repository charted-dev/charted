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

plugins {
    id("com.diffplug.spotless") version "6.11.0"
    `java-gradle-plugin`
    java
}

group = "org.noelware.charted.build-tools"

repositories {
    gradlePluginPortal()
    mavenCentral()
    mavenLocal()
}

dependencies {
    implementation("com.netflix.nebula:gradle-ospackage-plugin:9.1.1")
    implementation("io.github.z4kn4fein:semver:1.3.3")
    implementation("com.google.code.gson:gson:2.9.1")
    implementation(gradleApi())
}

gradlePlugin {
    plugins {
        create("aur") {
            id = "org.noelware.charted.distribution.aur"
            implementationClass = "org.noelware.charted.gradle.plugins.aur.ChartedAurPlugin"
        }

        create("deb") {
            id = "org.noelware.charted.distribution.deb"
            implementationClass = "org.noelware.charted.gradle.plugins.deb.ChartedDebPlugin"
        }

        create("docker") {
            id = "org.noelware.charted.distribution.docker"
            implementationClass = "org.noelware.charted.gradle.plugins.docker.ChartedDockerPlugin"
        }

        create("rpm") {
            id = "org.noelware.charted.distribution.rpm"
            implementationClass = "org.noelware.charted.gradle.plugins.rpm.ChartedRpmPlugin"
        }

        create("homebrew") {
            id = "org.noelware.charted.distribution.homebrew"
            implementationClass = "org.noelware.charted.gradle.plugins.homebrew.ChartedHomebrewPlugin"
        }

        create("scoop") {
            id = "org.noelware.charted.distribution.scoop"
            implementationClass = "org.noelware.charted.gradle.plugins.scoop.ChartedScoopPlugin"
        }

        create("golang") {
            id = "org.noelware.charted.golang"
            implementationClass = "org.noelware.charted.gradle.plugins.golang.GoPlugin"
        }
    }
}

spotless {
    java {
        licenseHeaderFile("${rootProject.projectDir.parentFile}/assets/HEADING")
        trimTrailingWhitespace()
        removeUnusedImports()
        palantirJavaFormat()
        endWithNewline()
    }
}
