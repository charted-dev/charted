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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.tracing.api

/**
 * Represents a generic clock used for [span events][Event].
 */
interface Clock {
    /**
     * Represents a [Clock] that is calculated by nanosecond precision.
     */
    object NanoTime: Clock {
        override fun provide(): Long = System.nanoTime()
    }

    /**
     * Represents a [Clock] that is calculated by millisecond precision.
     */
    object MillisecondTime: Clock {
        override fun provide(): Long = System.currentTimeMillis()
    }

    /**
     * Provides the timestamp for this specific clock.
     */
    fun provide(): Long
}
