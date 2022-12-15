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

import org.noelware.charted.gradle.Architecture
import org.noelware.charted.gradle.VERSION
import com.netflix.gradle.plugins.rpm.Rpm
import dev.floofy.utils.gradle.*

val Architecture.rpmArch: org.redline_rpm.header.Architecture
    get() = when (this) {
        Architecture.AARCH64 -> org.redline_rpm.header.Architecture.AARCH64
        Architecture.X64 -> org.redline_rpm.header.Architecture.X86_64
    }

plugins {
    id("org.noelware.charted.dist.nebula")
    distribution
}

val buildRpmRepo by tasks.registering(Rpm::class) {
    val architecture = Architecture.current()

    packageName = "charted-server"
    setArch(architecture.rpmArch)

    archiveFileName by "charted-server-$VERSION-${architecture.getName()}.rpm"
    destinationDirectory.set(file("$buildDir/distributions"))
    archiveVersion by "$VERSION"

//    setPreInstall(file("scripts/preinstall"))
//    setPostInstall(file("scripts/postinstall"))
//    setPreUninstall(file("scripts/preuninstall"))
//    setPreUninstall(file("scripts/postuninstall"))

    // /etc/noelware/charted/server will be the main directory where
    // the binary, libraries, and configuration will live in.
    val distBaseUrl = File(project(":cli").projectDir, "distribution")
    val distribution = project(":cli").extensions.getByName<DistributionContainer>("distributions").named("main").get()
    into("/etc/noelware/charted/server") {
        include("${project(":databases:clickhouse:migrations").projectDir}/bin/ch-migrations")
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
