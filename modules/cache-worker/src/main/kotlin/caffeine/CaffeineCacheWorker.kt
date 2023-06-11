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

package org.noelware.charted.modules.caching.caffeine

import com.github.benmanes.caffeine.cache.Caffeine
import dev.floofy.utils.kotlin.threading.createThreadFactory
import kotlinx.coroutines.future.asCompletableFuture
import kotlinx.coroutines.future.await
import org.noelware.charted.ChartedScope
import org.noelware.charted.async
import org.noelware.charted.modules.caching.CacheWorker
import java.util.concurrent.Executors
import kotlin.reflect.KClass
import kotlin.reflect.safeCast
import kotlin.time.Duration.Companion.minutes
import kotlin.time.toJavaDuration

class CaffeineCacheWorker: CacheWorker {
    private val innerCache = Caffeine.newBuilder()
        .expireAfterAccess(15.minutes.toJavaDuration())
        .recordStats()
        .weakKeys()
        .executor(Executors.newSingleThreadExecutor(createThreadFactory("CaffeineCacheExecutor")))
        .buildAsync<String, Any>()

    override suspend fun invalidateAll() {
        innerCache.synchronous().invalidateAll()
    }

    override suspend fun <T: Any> getOrPut(key: String, klazz: KClass<T>, push: suspend (key: String) -> T?): T? {
        val value = innerCache.get(key) { _, _ ->
            ChartedScope.async { push(key) }.asCompletableFuture()
        }.await() ?: return null

        return klazz.safeCast(value)
            ?: throw ClassCastException("Unable to cast ${value::class} ~> $klazz")
    }

//    override suspend fun <T: Any> count(klazz: KClass<T>): Int = innerCache
//        .synchronous()
//        .estimatedSize()
//
//    override suspend fun stats(): Stats = Stats(
//        count<RepositoryRelease>(),
//        count<Organization>(),
//        count<Repository>(),
//        count<User>()
//    )
}
