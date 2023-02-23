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

import com.diffplug.gradle.spotless.SpotlessExtensionPredeclare
import org.noelware.charted.gradle.*

plugins {
    id("com.diffplug.spotless")
    application
}

group = "org.noelware.charted"
version = "$VERSION"
description = "\uD83D\uDCE6 You know, for Helm Charts?"

repositories {
    mavenCentral()
    mavenLocal()
}

spotless {
    predeclareDeps()
    encoding("UTF-8")
}

the<SpotlessExtensionPredeclare>().apply {
    kotlinGradle {
        ktlint()
    }

    kotlin {
        ktlint()
    }

    java {
        googleJavaFormat()
        palantirJavaFormat()
    }
}

tasks {
    wrapper {
        distributionType = Wrapper.DistributionType.ALL
    }

    test {
        useJUnitPlatform()
    }

    create<Copy>("precommitHook") {
        from(file("${project.rootDir}/scripts/pre-commit"))
        into(file("${project.rootDir}/.git/hooks"))
    }
}
