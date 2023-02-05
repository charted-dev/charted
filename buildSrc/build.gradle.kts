/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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
    implementation("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.19.0")
    implementation("com.diffplug.spotless:spotless-plugin-gradle:6.14.1")
    implementation("com.netflix.nebula:gradle-ospackage-plugin:11.0.0")
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

configurations.configureEach {
    if (isCanBeResolved) {
        attributes {
            @Suppress("UnstableApiUsage")
            attribute(GradlePluginApiVersion.GRADLE_PLUGIN_API_VERSION_ATTRIBUTE, project.objects.named(GradleVersion.current().version))
        }
    }
}
