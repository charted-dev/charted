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

import dev.floofy.utils.gradle.*
import org.gradle.api.JavaVersion

/**
 * Refers to the current release channel.
 */
val CHANNEL: Channels = Channels.BETA

/**
 * Refers to the current version of charted-server.
 */
val VERSION: Version = Version(0, 1, 0, CHANNEL.releaseType, true)

/**
 * Refers to the minimum Java version that is required and will
 * be compiled towards.
 */
val JAVA_VERSION: JavaVersion = JavaVersion.VERSION_17
