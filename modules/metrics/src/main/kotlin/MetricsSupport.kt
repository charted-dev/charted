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

package org.noelware.charted.modules.metrics

import kotlin.reflect.KClass

/**
 * Represents an interface of handling all metrics into one interface.
 */
interface MetricsSupport {
    /**
     * Returns all the registered collectors available.
     */
    val collectors: List<Collector<*>>

    /**
     * Adds a collector to this metrics module
     * @param collector the collector to register
     */
    fun add(collector: Collector<*>)

    /**
     * Collects all the data from the given [collectors] collection and returns
     * the collection as a Map.
     */
    suspend fun collect(): Map<String, Any>

    /**
     * Collects data from a specific [Collector].
     * @param collector Collector to use to collect the data.
     * @return Data represented as
     */
    suspend fun <U: Any> collectFrom(collector: KClass<Collector<U>>): U?
}
