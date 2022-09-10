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

import org.gradle.api.Project;
import org.jetbrains.annotations.Nullable;

import java.io.File;

/**
 * Represents the downloader to install Golang in the root project's build directory.
 */
public class GoDownloader {
    private final String GOLANG_DOWNLOAD_URL = "https://go.dev/dl/go%s%s-%s.tar.gz";
    private final Project rootProject;

    public GoDownloader(Project rootProject) {
        this.rootProject = rootProject;
    }

    /**
     * Returns whether the downloader has been downloaded Go or not.
     */
    public boolean isDownloaded() {
        final var directory = new File(rootProject.getBuildDir(), "golang");
        return directory.exists();
    }

    /**
     * Returns the {@link File directory} where the Go installation is at, or null
     * if {@link #isDownloaded()} is false.
     */
    @Nullable
    public File getPath() {
        if (!isDownloaded()) return null;
        return new File(rootProject.getBuildDir(), "golang");
    }

    /**
     * Returns the <code>bin</code> directory of the downloaded Go installation, or null
     * if the downloader hasn't downloaded Go.
     */
    @Nullable
    public File getBinPath() {
        final var root = getPath();
        return root == null ? null : new File(root, "bin");
    }

    public void download() {

    }
}
