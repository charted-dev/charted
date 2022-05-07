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

plugins {
    `kotlin-dsl`
    groovy
}

repositories {
    maven("https://maven.floofy.dev/repo/releases")
    gradlePluginPortal()
    mavenCentral()
    mavenLocal()
}

dependencies {
    implementation("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.17.2")
    implementation("com.diffplug.spotless:spotless-plugin-gradle:6.5.2")
    implementation("com.google.protobuf:protobuf-gradle-plugin:0.8.18")
    implementation("org.jetbrains.kotlin:kotlin-gradle-plugin:1.6.21")
    implementation("org.jetbrains.kotlin:kotlin-serialization:1.6.21")
    implementation("com.netflix.nebula:gradle-ospackage-plugin:9.1.1")
    implementation("dev.floofy.commons:gradle:2.1.0")
}
