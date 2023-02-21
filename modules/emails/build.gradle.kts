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
    implementation("com.google.protobuf:protobuf-kotlin:3.22.0")
    implementation("com.google.protobuf:protobuf-java:3.22.0")
    compileOnly("org.apache.tomcat:annotations-api:6.0.53")
    implementation("io.grpc:grpc-kotlin-stub:1.3.0")
    runtimeOnly("io.grpc:grpc-netty-shaded:1.53.0")
    implementation("io.grpc:grpc-protobuf:1.53.0")
    implementation("io.grpc:grpc-services:1.53.0")
    implementation("io.grpc:grpc-stub:1.53.0")
}

sourceSets {
    create("proto") {
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

spotless {
    kotlin {
        targetExclude(
            "build/generated/source/proto/main/grpc-kotlin/org/noelware/charted/emails/protobufs/v1/**/*.kt",
            "build/generated/source/proto/proto/grpc-kotlin/org/noelware/charted/emails/protobufs/v1/**/*.kt",
            "build/generated/source/proto/proto/kotlin/org/noelware/charted/emails/protobufs/v1/**/*.kt",
            "build/generated/source/proto/main/kotlin/org/noelware/charted/emails/protobufs/v1/**/*.kt",
            "build/generated/source/proto/proto/kotlin/com/google/protobuf/**/*.kt",
            "build/generated/source/proto/main/kotlin/com/google/protobuf/**/*.kt",
        )
    }

    java {
        targetExclude(
            "build/generated/source/proto/main/grpc/org/noelware/charted/emails/protobufs/v1/**/*.java",
            "build/generated/source/proto/main/java/org/noelware/charted/emails/protobufs/v1/**/*.java",
            "build/generated/source/proto/proto/grpc/org/noelware/charted/emails/protobufs/v1/**/*.java",
            "build/generated/source/proto/proto/java/org/noelware/charted/emails/protobufs/v1/**/*.java",
            "build/generated/source/proto/main/grpc/com/google/protobuf/**/*.java",
            "build/generated/source/proto/main/java/com/google/protobuf/**/*.java",
            "build/generated/source/proto/proto/grpc/com/google/protobuf/**/*.java",
            "build/generated/source/proto/proto/java/com/google/protobuf/**/*.java",
        )
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
            path = protocPath
        } else {
            artifact = "com.google.protobuf:protoc:3.22.0"
        }
    }

    plugins {
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:1.53.0"
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

            it.builtins {
                id("kotlin")
            }
        }
    }
}

val copyProtobufs by tasks.registering(Copy::class) {
    from(file("$projectDir/vendor/protos/protos"))
    into(file("$projectDir/src/main/proto"))

    include("*.proto")
}
