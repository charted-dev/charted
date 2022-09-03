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

package org.noelware.charted.gradle.plugins.aur;

import java.io.IOException;
import java.util.Objects;
import org.gradle.api.GradleException;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.gradle.UnixUtils;

public class ChartedAurPlugin implements Plugin<Project> {
    @Override
    public void apply(@NotNull Project project) {
        try {
            final var distro = UnixUtils.getDistroName();
            if (distro == null) {
                project.getLogger()
                        .lifecycle(
                                "[distribution:aur] AUR plugin is disabled because current host system was Windows or macOS,"
                                        + " please use the Docker image if you wish to use the AUR plugin located at ./distribution/aur/Dockerfile,"
                                        + " with the :buildAurDockerImage task.");
            }

            if (!Objects.equals(distro, "Arch Linux")) {
                project.getLogger()
                        .lifecycle(
                                "[distribution:aur] AUR plugin is disabled because current distribution was not Arch Linux."
                                        + " Please us the Docker image if you wish to continue using the AUR plugin located in ./distribution/aur/Dockerfile, or"
                                        + " with the :buildAurDockerImage task.");
            }

            project.getLogger().lifecycle("[distribution:aur] AUR plugin is enabled for publishing! ^-^");
        } catch (IOException e) {
            throw new GradleException("Unexpected IO exception occurred:", e);
        }
    }
}
