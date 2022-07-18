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

package org.noelware.charted.core.ratelimiter

import kotlinx.datetime.*

@kotlinx.serialization.Serializable
data class Ratelimit(
    val remaining: Long = 1200,
    val resetAt: Instant,
    val limit: Long = 1200
)

fun Ratelimit.consume(): Ratelimit = copy(remaining = (remaining - 1).coerceAtLeast(0))
val Ratelimit.exceeded: Boolean
    get() = !expired && remaining == 0L

val Ratelimit.expired: Boolean
    get() = resetAt >= Clock.System.now()
