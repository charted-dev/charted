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

import org.noelware.charted.gradle.*

plugins {
    `charted-module`
}

dependencies {
    implementation("net.bytebuddy:byte-buddy-agent:1.14.3")
    implementation("net.bytebuddy:byte-buddy:1.14.4")
}

tasks.withType<Jar>().configureEach {
    manifest {
        attributes(
            mapOf(
                "Can-Retransform-Classes" to "true",
                "Implementation-Version" to "$VERSION",
                "Implementation-Vendor" to "Noelware, LLC. [team@noelware.org]",
                "Implementation-Title" to "charted-server",
                "Can-Redefine-Classes" to "true",
                "Agent-Class" to "org.noelware.charted.modules.tracing.agent.TracingAgent",
            ),
        )
    }
}
