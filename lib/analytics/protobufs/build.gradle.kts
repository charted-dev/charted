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

import com.google.protobuf.gradle.*

plugins {
    id("com.google.protobuf")
    `charted-module`
    idea
}

repositories {
    gradlePluginPortal()
}

dependencies {
    compileOnly(libs.tomcat.annotations.api)
    runtimeOnly(libs.grpc.netty.shaded)

    api(libs.grpc.kotlin.stub)
    api(libs.protobufs.kotlin)
    api(libs.protobufs.java)
    api(libs.grpc.protobuf)
    api(libs.grpc.services)
    api(libs.grpc.stub)
}

sourceSets {
    create("protobuf") {
        proto {
            srcDir("src/main/proto")
        }
    }

    main {
        java {
            srcDirs("build/generated/source/proto/main/java")
        }
    }

    test {
        java {
            srcDirs("build/generated/source/proto/main/java")
        }
    }
}

protobuf {
    // The environment variable was added so the Docker image can compile the server with
    // `protoc`. The Maven artifact only compiles on glib rather than musl (gcompat didn't work D:), so a simple fix
    // is the CHARTED_PROTOC_PATH environment variable.
    //
    // https://github.com/google/protobuf-gradle-plugin/issues/265#issuecomment-421508779
    protoc {
        val protocPath = System.getenv("CHARTED_PROTOC_PATH")
        if (protocPath != null) {
            logger.lifecycle("Using `protoc` in path [$protocPath]")
            path = protocPath
        } else {
            logger.lifecycle("Using protoc artifact! If you wish to set a custom protoc path, use the `CHARTED_PROTOC_PATH` environment variable~")
            artifact = "com.google.protobuf:protoc:3.21.5"
        }
    }

    plugins {
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:1.49.0"
        }

        id("grpc-kotlin") {
            artifact = "io.grpc:protoc-gen-grpc-kotlin:1.3.0:jdk8@jar"
        }
    }

    generateProtoTasks {
        all().forEach {
            it.plugins {
                id("grpc")
                id("grpc-kotlin")
            }

            it.builtins { id("kotlin") }
        }
    }
}

// Update whenever the Protobuf code gets updated.
val protoFiles = listOf("connection.proto")
tasks.register<Copy>("copyProtobuf") {
    into("$projectDir/src/main/proto") {
        for (file in protoFiles) {
            from("${rootProject.projectDir}/vendor/protobufs/$file")
        }
    }
}
