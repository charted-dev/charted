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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.gradle.plugins.aur;

import org.gradle.api.GradleException;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.noelware.charted.gradle.OperatingSystem;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.util.HashMap;
import java.util.Locale;

public class ChartedAurPlugin implements Plugin<Project> {
    @Override
    public void apply(Project project) {
        if (OperatingSystem.current().isWindows() || OperatingSystem.current().isMacOS()) {
            project.getLogger().lifecycle("[distribution:aur] Disabled because current host system is Windows or MacOS. Use the Docker image at `./distribution/aur/Dockerfile` to publish the AUR package.");
            return;
        }

        var release = new File("/etc/os-release");
        var metadata = new HashMap<String, String>();

        try {
            var fd = new String(new FileInputStream(release).readAllBytes());
            var lines = fd.split("\n");
            for (var line: lines) {
                var data = line.split("=");
                metadata.putIfAbsent(data[0].toLowerCase(Locale.ROOT), data[1].replaceAll("\"", ""));
            }
        } catch (IOException e) {
            throw new GradleException("Unable to read /etc/os-release:", e);
        }

        if (!metadata.containsKey("name"))
            throw new GradleException("/etc/os-release is corrupted: missing `NAME` attribute.");

        var name = metadata.get("name");
        if (name != "Arch Linux") {
            project.getLogger().lifecycle("[distribution:aur] Current host system is not Arch Linux, please use the Docker image as an alternative: `./gradlew :distribution:aur:runAurDockerImage`");
            return;
        }
    }
}
