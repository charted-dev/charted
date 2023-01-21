package org.noelware.charted.extensions.closeable

import java.io.Closeable

public fun <T: Closeable> T.closeQuietly() {
    try {
        close()
    } catch (e: RuntimeException) {
        throw e
    } catch (ignored: Exception) {
        /* ignore */
    }
}
