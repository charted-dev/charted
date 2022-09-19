package org.noelware.charted.common.extensions

import java.io.File
import java.nio.file.Path

/**
 * Returns the real path for resolving symbolic links or relative paths (i.e, `./owo.txt` -> `$DIR/owo.txt`)
 */
fun File.toRealPath(): Path = toPath().toRealPath()
