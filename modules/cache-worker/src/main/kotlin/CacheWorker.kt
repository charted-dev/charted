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

package org.noelware.charted.modules.caching

import kotlin.reflect.KClass

/**
 * Blueprint to implement cache workers, to cache objects efficiently in-memory or in Redis.
 */
interface CacheWorker {
    /**
     * Invalidates all the cache in this worker. Not recommended, but
     * it'll probably fix issues.
     */
    suspend fun invalidateAll()

    /**
     * Gets an entry from the cache, or pushes a new entry with the
     * [push] closure if it wasn't found. If the push closure returns
     * `null`, then it will not be inserted in cache, and will return
     * null.
     *
     * @param key Cache key to retrieve from
     * @param klazz Class to cast the inserted object as.
     * @param push Closure to push a new entry in this cache
     * @return Cached or newly cached object found in this worker.
     * @throws ClassCastException If the [class][klazz] used couldn't be cast
     * correctly with the cached object.
     */
    suspend fun <T: Any> getOrPut(key: String, klazz: KClass<T>, push: suspend (key: String) -> T?): T?

    /**
     * Counts how many cached objects that are available for the
     * [class][klazz] given.
     *
     * @param klazz The class to use to retrieve objects from
     * @return The cached objects available as a [Int], this will
     * return -1 if it couldn't be retrieved correctly.
     */
    // suspend fun <T: Any> count(klazz: KClass<T>): Int

    /**
     * Stats for this [CacheWorker]. At the moment, this only gives
     * the [counts][count] of objects that we store.
     */
    // suspend fun stats(): Stats
}

/**
 * Gets an entry from the cache, or pushes a new entry with the
 * [push] closure if it wasn't found. If the push closure returns
 * `null`, then it will not be inserted in cache, and will return
 * null.
 *
 * This extension will use the reified type parameter as the class to cast
 * the cached object as.
 *
 * @param key Cache key to retrieve from
 * @param push Closure to push a new entry in this cache
 * @return Cached or newly cached object found in this worker.
 * @throws ClassCastException If the [class][T] used couldn't be cast
 * correctly with the cached object.
 * @see CacheWorker.getOrPut
 */
suspend inline fun <reified T: Any> CacheWorker.getOrPut(key: String, noinline push: (key: String) -> T?): T? =
    getOrPut(key, T::class, push)

/**
 * Counts how many cached objects that are available for the
 * [class][T] given.
 *
 * This extension will use the reified type parameter as the class to
 * find objects to count.
 *
 * @return The cached objects available as a [Int], this will
 * return -1 if it couldn't be retrieved correctly.
 */
// suspend inline fun <reified T: Any> CacheWorker.count(): Int = count(T::class)
