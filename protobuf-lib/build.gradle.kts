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

import com.google.protobuf.gradle.*

plugins {
    `charted-java-module`
    idea
    id("com.google.protobuf")
}

// This copies the `analytics.proto` file from `protobufs/` into the
// src/main/proto directory.
val copyProtobuf by tasks.registering(Copy::class) {
    from(file("${rootProject.projectDir}/protobufs/connection.proto"))
    into(file("$projectDir/src/main/proto/connection.proto"))
}

sourceSets {
    create("protobufs") {
        proto {
            srcDir("src/main/proto")
        }
    }
}

repositories {
    gradlePluginPortal()
}

dependencies {
    implementation("com.google.protobuf:protobuf-java:3.20.1")
    implementation("io.grpc:grpc-protobuf:1.46.0")
    implementation("io.grpc:grpc-stub:1.46.0")
    compileOnly("org.apache.tomcat:annotations-api:6.0.53")
    runtimeOnly("io.grpc:grpc-netty-shaded:1.46.0")
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:3.20.1"
    }

    plugins {
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:1.46.0"
        }
    }

    generateProtoTasks {
        ofSourceSet("main").forEach {
            it.plugins {
                id("grpc")
            }
        }
    }
}
