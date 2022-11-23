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

package org.noelware.charted.server

import com.github.benmanes.caffeine.cache.AsyncCache
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.async
import kotlinx.coroutines.future.asCompletableFuture
import kotlinx.coroutines.future.await

/**
 * Represents a wrapper of [AsyncCache] to support kotlinx.coroutines patterns.
 */
class CoroutinesBasedCaffeineCache<K, V>(
    private val coroutineScope: CoroutineScope,
    private val asyncCache: AsyncCache<K, V>
): AsyncCache<K, V> by asyncCache {
    suspend fun getOrPut(key: K, compute: suspend (K) -> V): V = asyncCache.get(key) { k, _ ->
        coroutineScope.async { compute(k) }.asCompletableFuture()
    }.await()
}
