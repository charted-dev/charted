package org.noelware.charted.configuration.kotlin.dsl.metrics.keysets.jvm

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.enumSets.EnumSet

@Serializable
public enum class JvmThreadInfoKeysets {
    @SerialName("charted_jvm_threads_background")
    BackgroundThreads,

    @SerialName("charted_jvm_threads_available")
    AvailableThreads,

    @SerialName("charted_jvm_peak_threads")
    PeakThreads,

    @SerialName("*")
    Wildcard,

    @SerialName("charted_jvm_threads")
    Threads;

    public object EnumSet: org.noelware.charted.configuration.kotlin.dsl.enumSets.EnumSet<JvmThreadInfoKeysets>(JvmThreadInfoKeysets::class) {
        override val wildcard: JvmThreadInfoKeysets
            get() = Wildcard
    }
}
