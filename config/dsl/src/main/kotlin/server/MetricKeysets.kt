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

package org.noelware.charted.configuration.kotlin.dsl.server

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Represents all the key-sets available to export Prometheus metrics for the
 * API server (i.e, request histogram/gauge)
 */
@Serializable
public enum class MetricKeysets {
    /**
     * A histogram of all the average latency between all API requests made
     * in this session.
     */
    @SerialName("request_latency")
    RequestLatency,

    /**
     * Gauge to determine how many requests in total that people made
     * to this instance.
     */
    @SerialName("requests")
    Requests,

    /**
     * Wildcard key-set, which enables all the keys in this key-set.
     */
    @SerialName("*")
    Wildcard
}

/**
 * Determine if the given list of [MetricKeysets] is a wildcard, so we list
 * all the options in the key-set instead of specific keys.
 */
public fun List<MetricKeysets>.isWildcard(): Boolean {
    if (isEmpty()) return false
    if (size == 1) return first() == MetricKeysets.Wildcard

    return any { it == MetricKeysets.Wildcard }
}
