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

import org.noelware.charted.gradle.Architecture
import org.noelware.charted.gradle.VERSION
import com.netflix.gradle.plugins.deb.Deb
import dev.floofy.utils.gradle.*

plugins {
    id("org.noelware.charted.dist.nebula")
    distribution
}

val buildDebRepo by tasks.registering(Deb::class) {
    val architecture = Architecture.current()

    packageName = "charted-server"
    setArch(if (architecture.isX64) "amd64" else "arm64")

    archiveFileName by "charted-server-$VERSION-${architecture.getName()}.deb"
    destinationDirectory.set(file("$buildDir/distributions"))
    archiveVersion by VERSION.toString()

//    setPreInstall(file("scripts/preinstall"))
//    setPostInstall(file("scripts/postinstall"))
//    setPreUninstall(file("scripts/preuninstall"))
//    setPreUninstall(file("scripts/postuninstall"))

    // /etc/noelware/charted/server will be the main directory where
    // the binary, libraries, and configuration will live in.
    val distBaseUrl = File(project(":cli").projectDir, "distribution")
    val distribution = project(":cli").extensions.getByName<DistributionContainer>("distributions").named("main").get()
    into("/etc/noelware/charted/server") {
        with(distribution.contents.include("**/*.jar"))
        from(distBaseUrl) {
            include("config/logback.properties")
            include("config/charted.yaml")
            include("bin/charted")
            include("charted.service")
            include("README.txt")
            include("LICENSE")
        }
    }

    into("/var/lib/noelware/charted/server/data") {
        createDirectoryEntry = true
        includeEmptyDirs = true
        permissionGroup = "charted"
        dirMode = 2750
        user = "charted"
    }

    into("/var/log/noelware/charted/server") {
        createDirectoryEntry = true
        includeEmptyDirs = true
        permissionGroup = "charted"
        dirMode = 2750
        user = "charted"
    }
}
