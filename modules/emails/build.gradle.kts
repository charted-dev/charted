/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

import org.noelware.charted.gradle.util.FindBinaryUtil
import com.google.protobuf.gradle.*
import de.undercouch.gradle.tasks.download.Download

plugins {
    id("de.undercouch.download") version "5.4.0"
    id("com.google.protobuf")
    id("charted-module")
    idea
}

repositories {
    gradlePluginPortal()
}

dependencies {
    compileOnly(libs.tomcat.annotations.api)
    api(libs.grpc.kotlin.stub)
    api(libs.protobuf.kotlin)
    runtimeOnly(libs.grpc.netty.shaded)
    api(libs.protobuf.java)
    api(libs.grpc.protobuf)
    api(libs.grpc.services)
    api(libs.grpc.stub)
}

tasks {
    create<Download>("downloadProtos") {
        overwrite(true)
        dest(file("$projectDir/src/main/proto/emails.proto"))
        src("https://raw.githubusercontent.com/charted-dev/email-service/main/protos/emails.proto")
    }
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
            logger.lifecycle("Found `CHARTED_PROTOC_PATH` environment variable! Using [$protocPath] as the `protoc` binary path")
            path = protocPath
        }

        val protocBinary = FindBinaryUtil.find("protoc")
        if (path == null && protocBinary != null) {
            logger.lifecycle("Found `protoc` binary from system \$PATH! Using [$protocBinary] as the `protoc` binary path")
            path = protocBinary
        }

        if (path == null) {
            logger.lifecycle("Using `protoc` binary from Maven artifact [com.google.protobuf:protoc]. Releases from this are very un-deterministic and might not be available on Maven, so please install `protoc` on the host machine.")
            artifact = "com.google.protobuf:protoc:3.23.2"
        }
    }

    plugins {
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:1.56.0"
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

tasks {
    named<ProtobufExtract>("extractIncludeTestProto") {
        dependsOn("transformAtomicfuClasses")
    }
}
