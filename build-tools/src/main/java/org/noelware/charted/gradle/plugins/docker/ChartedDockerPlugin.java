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

package org.noelware.charted.gradle.plugins.docker;

import io.github.z4kn4fein.semver.Version;
import io.github.z4kn4fein.semver.constraints.Constraint;
import java.io.ByteArrayOutputStream;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.jetbrains.annotations.NotNull;

public class ChartedDockerPlugin implements Plugin<Project> {
    @Override
    public void apply(@NotNull Project project) {
        var extension = project.getExtensions().create("docker", DockerExtension.class);
        project.getLogger().lifecycle("[distribution:docker] Checking if `docker` is installed...");

        var stdout = new ByteArrayOutputStream();
        try {
            final var result = project.exec(spec -> {
                spec.setCommandLine("docker");
                spec.args("version", "--format='{{.Client.Version}}'");
                spec.setStandardOutput(stdout);
            });

            final var data = stdout.toString();
            if (result.getExitValue() != 0) {
                project.getLogger()
                        .lifecycle(
                                "[distribution:docker] Unable to run 'docker version':\n" + "[== STDOUT ==]\n\n", data);
                return;
            }

            var minVersion = extension.getMinDockerVersion().get();
            project.getLogger()
                    .lifecycle(
                            String.format("[distribution:docker] Checking if Docker version satisfies %s", minVersion));

            final var constraint = Constraint.Companion.parse(minVersion);
            final var current = data.replaceAll("'", "").trim();

            if (!constraint.isSatisfiedBy(Version.Companion.parse(current, false))) {
                project.getLogger()
                        .lifecycle(String.format(
                                "[distribution:docker] Version %s was not satisified by" + " constraint %s.",
                                current, minVersion));
                return;
            }

            project.getLogger()
                    .lifecycle(String.format("[distribution:docker] \uD83D\uDC33 Using Docker v%s", current));
        } catch (Exception e) {
            project.getLogger().lifecycle("Unable to check if Docker exists on host:", e);
        }
    }
}
