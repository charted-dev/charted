package org.noelware.charted.engine.charts.tests

import kotlinx.coroutines.runBlocking
import org.noelware.charted.core.StorageWrapper
import org.noelware.remi.core.StorageTrailer
import org.noelware.remi.filesystem.FilesystemStorageTrailer

class MockStorageWrapper: StorageWrapper {
    override val trailer: StorageTrailer<*> = FilesystemStorageTrailer("/tmp/.test-remi-storage")
    init {
        runBlocking { trailer.init() }
    }
}
