package org.noelware.charted.stats

/**
 * Represents a basic statistics collector interface which only exports the [#collect()][collect] function.
 */
interface StatCollector<T> {
    /**
     * Collects the statistics and returns the stats as [T].
     */
    suspend fun collect(): T
}
