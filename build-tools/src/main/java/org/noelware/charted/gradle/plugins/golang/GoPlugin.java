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

package org.noelware.charted.gradle.plugins.golang;

import org.gradle.api.GradleException;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.gradle.OperatingSystem;

import java.io.File;
import java.io.IOException;
import java.util.Map;

public class GoPlugin implements Plugin<Project> {
    @Override
    public void apply(@NotNull Project project) {
        final var extension = project.getExtensions().create("golang", GoExtension.class);
        final var useLocalSystemCompiler = extension.getUseLocalSystemCompiler().getOrElse(true);
        final var goPath = extension.getGoPath();
        final var os = OperatingSystem.current();
        final var downloader = new GoDownloader(project.getRootProject());

        if (!useLocalSystemCompiler) {
            project.getLogger().lifecycle("Downloading Go 1.19 since useLocalSystemCompiler is set to false.");
            if (downloader.isDownloaded()) {
                final var bin = downloader.getBinPath();
                goPath.set(new File(bin, "go%s".formatted(os.isWindows() ? ".exe" : "")));
            } else {
                downloader.download();
            }
        } else {
            project.getLogger().lifecycle("Checking if `go env` can be executed...");
            final Map<String, String> env;
            try {
                env = GoEnv.getGoEnvironment((ExecOperationsLike) project);
            } catch (IOException e) {
                throw new GradleException("Unable to execute go env:", e);
            }

            if (env == null) {
                project.getLogger().lifecycle("Unable to execute go env for some reason, downloading it locally...");
                if (downloader.isDownloaded()) {
                    final var bin = downloader.getBinPath();
                    goPath.set(new File(bin, "go%s".formatted(os.isWindows() ? ".exe" : "")));
                } else {
                    downloader.download();
                }
            } else {
                final var goroot = env.get("GOROOT");
                if (goroot == null)
                    throw new GradleException("Go installation is corrupted. Missing `GOROOT` in go env.");

                goPath.set(new File(goroot));
            }
        }
    }
}
