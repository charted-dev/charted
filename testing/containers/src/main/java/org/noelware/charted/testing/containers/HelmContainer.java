/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.testing.containers;

import java.io.File;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.utility.DockerImageName;

public class HelmContainer extends GenericContainer<HelmContainer> {
    private static final String DEFAULT_HELM_VERSION = "3.11.1";
    private final File helmCacheDir;
    private final File helmDir;

    public HelmContainer() {
        this(DEFAULT_HELM_VERSION);
    }

    public HelmContainer(@NotNull String helmVersion) {
        this(helmVersion, null, null);
    }

    public HelmContainer(@NotNull String helmVersion, @Nullable File helmDir, @Nullable File helmCacheDir) {
        super(DockerImageName.parse("alpine/helm").withTag(helmVersion));

        // helm cache dir (i.e, ~/.config/helm)
        this.helmCacheDir = helmCacheDir;

        // helm dir (i.e, ~/.helm)
        this.helmDir = helmDir;
    }
}
