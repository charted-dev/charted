package org.noelware.charted.gradle.plugins

import org.gradle.api.file.RegularFile
import org.gradle.api.plugins.ExtensionContainer
import org.gradle.kotlin.dsl.getByName
import org.noelware.charted.gradle.plugins.docker.ChartedDockerExtension
import org.noelware.charted.gradle.plugins.helm.HelmPluginExtension
import java.io.File

val ExtensionContainer.docker: ChartedDockerExtension?
    get() {
        try {
            return getByName<ChartedDockerExtension>("docker")
        } catch (e: Exception) {
            return null
        }
    }

val ExtensionContainer.helm: HelmPluginExtension?
    get() {
        try {
            return getByName<HelmPluginExtension>("helm")
        } catch (e: Exception) {
            return null
        }
    }

fun File.toRegularFile(): RegularFile = RegularFile { this }
