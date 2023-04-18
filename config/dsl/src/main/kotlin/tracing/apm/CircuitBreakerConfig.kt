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

package org.noelware.charted.configuration.kotlin.dsl.tracing.apm

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.Buildable

@Serializable
public data class CircuitBreakerConfig(
    /**
     * The threshold used by the system CPU monitor to determine that the system is not under CPU stress. If the monitor detected
     * a CPU stress, the measured system CPU needs to be below this threshold for a duration of at least
     * [stress_monitor_cpu_duration_threshold][cpuThreshold] in order for the monitor to decide that the CPU stress has been relieved.
     */
    @ApmSystemProperty("stress_monitor_system_cpu_relief_threshold")
    @SerialName("cpu_relief_threshold")
    val cpuReliefThreshold: Double = 0.8,

    /**
     * The threshold used by the system CPU monitor to detect system CPU stress. If the system CPU crosses this threshold for
     * a duration of at least [stress_monitor_cpu_duration_threshold][cpuThreshold], the monitor considers this as a stress state.
     */
    @ApmSystemProperty("stress_monitor_system_cpu_stress_threshold")
    @SerialName("cpu_stress_threshold")
    val cpuStressThreshold: Double = 0.95,

    /**
     * The minimal time required in order to determine whether the system is either currently under stress, or that the stress detected
     * previously has been relieved. All measurements during this time must be consistent in comparison to the relevant threshold
     * in order to detect a change of stress state. Must be at least 1m.
     *
     * Supports the duration suffixes ms, s and m. Example: 1m.
     */
    @ApmSystemProperty("stress_monitor_cpu_duration_threshold")
    @SerialName("cpu_threshold")
    val cpuThreshold: String = "1m",

    /**
     * The threshold used by the GC monitor to rely on for identifying when the heap is not under stress.
     * If [stress_monitor_gc_stress_threshold][threshold] has been crossed, the agent will consider it a heap-stress state.
     *
     * In order to determine that the stress state is over, percentage of occupied memory in ALL heap pools should be lower
     * than this threshold. The GC monitor relies only on memory consumption measured after a recent GC.
     */
    @ApmSystemProperty("stress_monitor_gc_relief_threshold")
    @SerialName("relief_threshold")
    val reliefThreshold: Double = 0.75,

    /**
     * The threshold used by the GC monitor to rely on for identifying heap stress. The same threshold will be used
     * for all heap pools, so that if ANY has a usage percentage that crosses it, the agent will consider it as a heap stress.
     * The GC monitor relies only on memory consumption measured after a recent GC.
     */
    @ApmSystemProperty("stress_monitor_gc_stress_threshold")
    val threshold: Double = 0.95,

    /**
     * The interval at which the agent polls the stress monitors. Must be at least 1s.
     *
     * Supports the duration suffixes ms, s and m. Example: 5s.
     */
    @ApmSystemProperty("stress_monitoring_interval")
    val interval: String = "5s",

    /**
     * Whether the circuit breaker should be enabled or not. When enabled, the agent periodically polls stress monitors
     * to detect system/process/JVM stress state. If ANY of the monitors detects a stress indication, the agent will become inactive,
     * as if the recording configuration option has been set to false, thus reducing resource consumption to a minimum.
     *
     * When inactive, the agent continues polling the same monitors in order to detect whether the stress state has been relieved.
     * If ALL monitors approve that the system/process/JVM is not under stress anymore, the agent will resume and become fully
     * functional.
     */
    @ApmSystemProperty("circuit_breaker_enabled")
    val enabled: Boolean = false
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: Buildable<CircuitBreakerConfig> {
        /**
         * The threshold used by the system CPU monitor to determine that the system is not under CPU stress. If the monitor detected
         * a CPU stress, the measured system CPU needs to be below this threshold for a duration of at least
         * [stress_monitor_cpu_duration_threshold][cpuThreshold] in order for the monitor to decide that the CPU stress has been relieved.
         */
        public var cpuReliefThreshold: Double = 0.8

        /**
         * The threshold used by the system CPU monitor to detect system CPU stress. If the system CPU crosses this threshold for
         * a duration of at least [stress_monitor_cpu_duration_threshold][cpuThreshold], the monitor considers this as a stress state.
         */
        public var cpuStressThreshold: Double = 0.95

        /**
         * The minimal time required in order to determine whether the system is either currently under stress, or that the stress detected
         * previously has been relieved. All measurements during this time must be consistent in comparison to the relevant threshold
         * in order to detect a change of stress state. Must be at least 1m.
         *
         * Supports the duration suffixes ms, s and m. Example: 1m.
         */
        public var cpuThreshold: String = "1m"

        /**
         * The threshold used by the GC monitor to rely on for identifying when the heap is not under stress.
         * If [stress_monitor_gc_stress_threshold][threshold] has been crossed, the agent will consider it a heap-stress state.
         *
         * In order to determine that the stress state is over, percentage of occupied memory in ALL heap pools should be lower
         * than this threshold. The GC monitor relies only on memory consumption measured after a recent GC.
         */
        public var reliefThreshold: Double = 0.75

        /**
         * The threshold used by the GC monitor to rely on for identifying heap stress. The same threshold will be used
         * for all heap pools, so that if ANY has a usage percentage that crosses it, the agent will consider it as a heap stress.
         * The GC monitor relies only on memory consumption measured after a recent GC.
         */
        public var threshold: Double = 0.95

        /**
         * The interval at which the agent polls the stress monitors. Must be at least 1s.
         *
         * Supports the duration suffixes ms, s and m. Example: 5s.
         */
        public var interval: String = "5s"

        /**
         * Whether the circuit breaker should be enabled or not. When enabled, the agent periodically polls stress monitors
         * to detect system/process/JVM stress state. If ANY of the monitors detects a stress indication, the agent will become inactive,
         * as if the recording configuration option has been set to false, thus reducing resource consumption to a minimum.
         *
         * When inactive, the agent continues polling the same monitors in order to detect whether the stress state has been relieved.
         * If ALL monitors approve that the system/process/JVM is not under stress anymore, the agent will resume and become fully
         * functional.
         */
        public var enabled: Boolean = false

        override fun build(): CircuitBreakerConfig = CircuitBreakerConfig(
            cpuReliefThreshold,
            cpuStressThreshold,
            cpuThreshold,
            reliefThreshold,
            threshold,
            interval,
            enabled,
        )
    }
}
