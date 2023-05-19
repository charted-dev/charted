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

package org.noelware.charted

import dev.floofy.utils.kotlin.threading.createThreadFactory
import io.sentry.Sentry
import io.sentry.kotlin.SentryContext
import kotlinx.coroutines.*
import java.util.concurrent.Executors
import kotlin.coroutines.CoroutineContext

public object ChartedScope: CoroutineScope {
    override val coroutineContext: CoroutineContext =
        SupervisorJob() + Executors.newCachedThreadPool(createThreadFactory("Charted-CoroutineExecutor")).asCoroutineDispatcher()
}

/**
 * Launches a new coroutine [job][Job] without blocking the main thread. This should
 * be the preferred way to launch coroutines since it will automatically attach a [SentryContext]
 * to the job if it is enabled by the API server.
 *
 * @param start [CoroutineStart] on how to start the coroutine
 * @param block Code to execute when launched
 */
public fun ChartedScope.launch(
    start: CoroutineStart = CoroutineStart.DEFAULT,
    block: suspend CoroutineScope.() -> Unit = {}
): Job = launch(
    if (Sentry.isEnabled()) SentryContext() + coroutineContext else coroutineContext,
    start,
    block,
)

public fun <T> ChartedScope.async(
    start: CoroutineStart = CoroutineStart.DEFAULT,
    block: suspend CoroutineScope.() -> T
): Deferred<T> = async(
    if (Sentry.isEnabled()) SentryContext() + coroutineContext else coroutineContext,
    start,
    block,
)
