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

package org.noelware.charted.configuration.kotlin.dsl.metrics.keysets.jvm

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.enumSets.EnumSet

@Serializable
public enum class JvmThreadInfoKeysets {
    @SerialName("charted_jvm_threads_background")
    BackgroundThreads,

    @SerialName("charted_jvm_threads_available")
    AvailableThreads,

    @SerialName("charted_jvm_peak_threads")
    PeakThreads,

    @SerialName("*")
    Wildcard,

    @SerialName("charted_jvm_threads")
    Threads;

    public object EnumSet: org.noelware.charted.configuration.kotlin.dsl.enumSets.EnumSet<JvmThreadInfoKeysets>(JvmThreadInfoKeysets::class) {
        override val wildcard: JvmThreadInfoKeysets
            get() = Wildcard
    }
}
