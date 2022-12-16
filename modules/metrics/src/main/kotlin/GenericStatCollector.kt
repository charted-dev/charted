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

/**
 * Represents a generic statistics collector. This interface is only for collecting generic
 * statistics that the [MetricStatCollector] and other sources can consume
 */
interface GenericStatCollector<T> {
    /** Returns the name of this statistics collector */
    val name: String

    /**
     * Collects all the statistics and returns the result.
     */
    fun collect(): T
}
