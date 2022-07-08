package org.noelware.charted.gradle.plugins.aur

import org.apache.tools.ant.filters.ReplaceTokens
import org.gradle.api.DefaultTask
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Copy
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.InputFile
import org.gradle.api.tasks.OutputFile
import org.gradle.api.tasks.TaskAction
import org.gradle.kotlin.dsl.filter
import org.noelware.charted.gradle.VERSION
import java.io.File

abstract class InstallPkgBuildRepositoryTask: Copy() {
    @TaskAction
    fun execute() {
        from("PKGBUILD") {
            filter<ReplaceTokens>(
                "tokens" to mapOf("package.version" to "$VERSION")
            )
        }

        into(".repo/PKGBUILD")
    }
}
