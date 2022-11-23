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

import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    `java-gradle-plugin`
    `kotlin-dsl`
    java
}

repositories {
    maven("https://maven.floofy.dev/repo/releases")
    gradlePluginPortal()
    mavenCentral()
    mavenLocal()
}

dependencies {
    implementation("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.18.5")
    implementation("com.diffplug.spotless:spotless-plugin-gradle:6.11.0")
    implementation("com.netflix.nebula:gradle-ospackage-plugin:10.0.0")
    implementation("de.undercouch:gradle-download-task:5.3.0")
    implementation("dev.floofy.commons:gradle:2.3.0")
    implementation(kotlin("serialization", "1.7.21"))
    implementation(kotlin("gradle-plugin", "1.7.21"))
    implementation("com.google.code.gson:gson:2.10")
    implementation("org.slf4j:slf4j-simple:1.7.36")
}

gradlePlugin {
    plugins {
        create("golang") {
            id = "org.noelware.charted.golang"
            implementationClass = "org.noelware.charted.gradle.plugins.golang.GolangPlugin"
        }

        create("nebula") {
            id = "org.noelware.charted.dist.nebula"
            implementationClass = "org.noelware.charted.gradle.plugins.nebula.ChartedNebulaPlugin"
        }
    }
}

tasks.withType<KotlinCompile>().configureEach {
    kotlinOptions.jvmTarget = "17"
}
