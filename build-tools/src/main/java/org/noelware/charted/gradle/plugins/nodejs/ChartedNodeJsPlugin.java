/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.gradle.plugins.nodejs;

import de.undercouch.gradle.tasks.download.Download;
import java.io.File;
import org.gradle.api.GradleException;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.gradle.api.tasks.Copy;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.gradle.Architecture;
import org.noelware.charted.gradle.OperatingSystem;

public class ChartedNodeJsPlugin implements Plugin<Project> {
    /**
     * Apply this plugin to the given target object.
     * @param project The target object
     */
    @Override
    public void apply(@NotNull Project project) {
        final var ext = project.getExtensions().create("nodejs", NodeJsExtension.class);
        final var nodeVersion =
                ext.getNodeVersion().convention(NodeJsExtension.NODEJS_VERSION).get();
        final var nodeDistUrl =
                ext.getNodeDistUrl().convention(NodeJsExtension.NODEJS_DIST_URL).get();
        final var os = OperatingSystem.current();
        final var arch = Architecture.current();

        if (os.isWindows() && arch.isArm())
            throw new GradleException("Can't apply Node.js plugin due to no arm64 Windows distribution available.");

        final var root = project.getRootProject();
        final var download = project.getTasks().create("downloadLocalNode", Download.class, (dl) -> {
            dl.dest(new File(root.getBuildDir(), "nodejs/" + nodeVersion + (os.isWindows() ? ".zip" : ".tar.gz")));
            dl.src(
                    os.isWindows()
                            ? String.format("%s/v%s/node-v%s-win-x64.zip", nodeDistUrl, nodeVersion, nodeVersion)
                            : String.format(
                                    "%s/v%s/node-v%s-%s-%s.tar.gz",
                                    nodeDistUrl,
                                    nodeVersion,
                                    nodeVersion,
                                    os.isMacOS() ? "darwin" : "linux",
                                    arch.isX64() ? "x64" : "arm64"));
        });

        if (os.isUnix()) {
            project.getTasks().create("extractInstallation", Copy.class, (spec) -> {
                spec.dependsOn(download);
                spec.from(project.tarTree(download.getDest()));
                spec.into(new File(root.getBuildDir(), "nodejs/" + nodeVersion));
            });
        } else {
            project.getTasks().create("extractInstallation", Copy.class, (spec) -> {
                spec.dependsOn(download);
                spec.from(project.zipTree(download.getDest()));
                spec.into(new File(root.getBuildDir(), "nodejs/" + nodeVersion));
            });
        }
    }
}
