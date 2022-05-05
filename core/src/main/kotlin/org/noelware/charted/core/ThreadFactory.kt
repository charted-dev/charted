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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core

import java.util.concurrent.ThreadFactory
import java.util.concurrent.atomic.AtomicLong

fun createThreadFactory(prefix: String): ThreadFactory = object: ThreadFactory {
    private val id = AtomicLong(0)
    private val group = Thread.currentThread().threadGroup

    override fun newThread(r: Runnable): Thread {
        val name = "$prefix[${id.incrementAndGet()}]"
        val thread = Thread(group, r, name)

        if (thread.priority != Thread.NORM_PRIORITY)
            thread.priority = Thread.NORM_PRIORITY

        return thread
    }
}
