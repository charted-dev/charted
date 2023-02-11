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

package org.noelware.charted.extensions

import dev.floofy.utils.kotlin.humanize
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.common.ByteSizeValue
import java.util.concurrent.TimeUnit

public fun Long.formatToSize(long: Boolean = false): String = ByteSizeValue.fromLong(this, long)
public fun StopWatch.doFormatTime(): String {
    val time = nanoTime
    if (time == 0L) return "<uninit>"

    // this is going to get real ugly, real quick.
    if (time < 1000) return "${time}ns"
    if (time < 1000000) return "${time}µs"

    // since it's ns -> ms
    val ms = TimeUnit.MILLISECONDS.convert(time, TimeUnit.NANOSECONDS)
    return ms.humanize(long = false, includeMs = true)
}
