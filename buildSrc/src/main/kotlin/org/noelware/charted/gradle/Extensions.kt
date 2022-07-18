package org.noelware.charted.gradle

import org.gradle.api.file.RegularFile
import java.io.File

/**
 * Returns this file as a [RegularFile].
 */
fun File.toRegularFile(): RegularFile = RegularFile { this }
