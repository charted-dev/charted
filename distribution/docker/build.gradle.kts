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

import org.noelware.charted.gradle.plugins.docker.BuildDockerImageTask
import org.noelware.charted.gradle.plugins.docker
import org.gradle.internal.os.OperatingSystem
import org.noelware.charted.gradle.*
import dev.floofy.utils.gradle.*

plugins {
    id("org.noelware.charted.distribution.docker")
}

docker {
    dockerfile(file("./amd64.Dockerfile")) {
        addBuildArgument("VERSION" to "$VERSION")
        addBuildArgument("COMMIT_SHA" to COMMIT_HASH)
    }

    dockerfile(file("./arm64.Dockerfile")) {
        platform = "linux/arm64"
        addBuildArgument("VERSION" to "$VERSION")
        addBuildArgument("COMMIT_SHA" to COMMIT_HASH)
    }

    dockerfile(file("./windows.Dockerfile")) {
        platform = "windows/amd64"
        addBuildArgument("VERSION" to "$VERSION")
        addBuildArgument("COMMIT_SHA" to COMMIT_HASH)
    }
}

val docker = extensions.docker!!
tasks.register<BuildDockerImageTask>("buildLinux64Image") {
    dockerfile by docker.dockerfiles.single { it.platform == "linux/amd64" }
    tag by "$VERSION"
}

tasks.register<BuildDockerImageTask>("buildLinuxArmImage") {
    doFirst {
        // TODO: support building with QEMU (fetching images, check if the multi-arch image exists, etc etc)
        val arch = System.getProperty("os.arch")
        if (!listOf("arm64", "aarch64").contains(arch)) {
            throw GradleException("Can't build arm64 image with architecture [$arch]")
        }
    }

    doLast {
        (this as BuildDockerImageTask).dockerfile by docker.dockerfiles.single { it.platform == "linux/arm64" }
        tag by "$VERSION"
    }
}

tasks.register<BuildDockerImageTask>("buildWindowsImage") {
    doFirst {
        val os = OperatingSystem.current()
        if (!os.isWindows) {
            throw GradleException("Unable to build Windows image on a non-Windows host.")
        }
    }

    doLast {
        (this as BuildDockerImageTask).dockerfile by docker.dockerfiles.single { it.platform == "windows/amd64" }
        tag by "$VERSION"
    }
}
