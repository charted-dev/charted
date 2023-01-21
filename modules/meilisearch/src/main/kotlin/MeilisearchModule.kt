package org.noelware.charted.modules.search.meilisearch

import java.io.Closeable

interface MeilisearchModule {
    /** meilisearch version */
    val serverVersion: String

    /**
     * Initializes this Meilisearch module
     */
    suspend fun init()
}
