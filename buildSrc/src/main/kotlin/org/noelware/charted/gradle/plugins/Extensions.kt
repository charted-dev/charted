package org.noelware.charted.gradle.plugins

import org.gradle.api.plugins.ExtensionContainer
import org.gradle.kotlin.dsl.getByName
import org.noelware.charted.gradle.plugins.docker.DockerPluginExtension
import org.noelware.charted.gradle.plugins.helm.HelmPluginExtension

val ExtensionContainer.docker: DockerPluginExtension?
    get() {
        try {
            return getByName<DockerPluginExtension>("docker")
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
