/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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
import com.github.benmanes.caffeine.cache.Cache
import com.github.benmanes.caffeine.cache.Caffeine
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.async
import kotlinx.coroutines.future.asCompletableFuture
import kotlinx.coroutines.future.await

interface CoroutinesCaffeineCache<K : Any, V> {
    val cache: Cache<K, V>

    suspend fun get(key: K, predicate: (key: K) -> V?): V?
}

internal class DefaultCoroutinesCaffeineCache<K : Any, V>(
    private val coroutineScope: CoroutineScope,
    private val asyncCache: AsyncCache<K, V>
) : CoroutinesCaffeineCache<K, V> {
    override val cache: Cache<K, V>
        get() = asyncCache.synchronous()

    override suspend fun get(key: K, predicate: (key: K) -> V?): V? = asyncCache.get(key) { k, _ ->
        coroutineScope.async { predicate(k) }.asCompletableFuture()
    }.await()
}

@OptIn(DelicateCoroutinesApi::class)
fun <K : Any, V, K1 : K, V1 : V> Caffeine<K, V>.buildCoroutinesBased(
    coroutineScope: CoroutineScope = GlobalScope
): CoroutinesCaffeineCache<K1, V1> = DefaultCoroutinesCaffeineCache(coroutineScope, buildAsync())
