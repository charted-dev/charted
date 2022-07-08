package org.noelware.charted.gradle.plugins.helm

import org.gradle.api.provider.Property
import java.io.File

abstract class HelmPluginExtension {
    abstract val valuesPath: Property<File>
    abstract val chartPath: Property<File>
}
