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

package org.noelware.charted.gradle

import dev.floofy.utils.gradle.ReleaseType
import org.gradle.api.Incubating

/**
 * Represents the different version channels that charted-server is currently
 * being used on.
 */
enum class Channels {
    /**
     * Represents the Nightly channel, this is different from the [UNSTABLE] channel
     * as this has a 75% working build; there is some major and minor bugs that need
     * to be fixed before being promoted to [BETA].
     */
    @Incubating
    NIGHTLY,

    /**
     * Represents the Unstable channel; this is by far the most unstable
     * that charted-server will be, this is not a recommended channel to be
     * in any circumstances.
     *
     * This has a 50% working build; there is many bugs in this build that
     * might cause unexpected results.
     */
    UNSTABLE,

    /**
     * Represents the Stable channel, this is the most recommended channel
     * to be on for production environments.
     *
     * This has a 100% working build, there might be bugs, but they should be
     * minor that a patch release might be required.
     */
    STABLE,

    /**
     * Represents the Beta channel, this is not a recommended channel but
     * recommended to try out new features once they are out of the Nightly
     * stage.
     *
     * This has a 90% working build, there might be minor bugs that might
     * need to be fixed but should be *stable* enough for testing
     * purposes.
     */
    @Incubating
    BETA
}

val Channels.releaseType: ReleaseType
    get() = when (this) {
        Channels.UNSTABLE -> ReleaseType("unstable")
        Channels.NIGHTLY -> ReleaseType("nightly")
        Channels.STABLE -> ReleaseType.None
        Channels.BETA -> ReleaseType.Beta
    }
