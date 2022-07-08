package org.noelware.charted.gradle.plugins.aur

import org.gradle.api.DefaultTask
import org.gradle.api.GradleException
import org.gradle.api.tasks.Exec
import org.gradle.api.tasks.TaskAction
import java.io.ByteArrayOutputStream

open class BuildPackageTask: DefaultTask() {
    init {
        group = "application"
    }

    @TaskAction
    fun execute() {

    }
}
