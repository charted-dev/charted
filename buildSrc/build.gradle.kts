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

import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    `java-gradle-plugin`
    `kotlin-dsl`
    java
}

repositories {
    maven("https://maven.floofy.dev/repo/releases")
    maven("https://maven.noelware.org")
    gradlePluginPortal()
    mavenCentral()
    mavenLocal()
}

dependencies {
    implementation("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.19.0")
    implementation("com.diffplug.spotless:spotless-plugin-gradle:6.15.0")
    implementation("com.netflix.nebula:gradle-ospackage-plugin:11.0.0")
    implementation("com.google.protobuf:protobuf-gradle-plugin:0.9.2")
    implementation("org.noelware.gradle:gradle-infra-plugin:1.3.0")
    implementation("dev.floofy.commons:gradle:2.5.0")
    implementation(kotlin("serialization", "1.8.10"))
    implementation(kotlin("gradle-plugin", "1.8.10"))
    implementation(gradleApi())
}

gradlePlugin {
    plugins {
        create("nebula") {
            id = "org.noelware.charted.dist.nebula"
            implementationClass = "org.noelware.charted.gradle.plugins.nebula.ChartedNebulaPlugin"
        }
    }
}

kotlin {
    jvmToolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

tasks.withType<KotlinCompile>().configureEach {
    kotlinOptions.jvmTarget = "17"
}

configurations.configureEach {
    if (isCanBeResolved) {
        attributes {
            attribute(GradlePluginApiVersion.GRADLE_PLUGIN_API_VERSION_ATTRIBUTE, project.objects.named(GradleVersion.current().version))
        }
    }
}
