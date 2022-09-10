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

import java.io.File;
import org.gradle.api.provider.Property;

/**
 * Represents the extension of the Golang Gradle plugin.
 */
public abstract class GoExtension {
    /**
     * Returns a {@link Property} whether if we should use the local system's Go compiler
     * that was installed on the system rather than downloaded and cached in rootProject/build/golang.
     */
    abstract Property<Boolean> getUseLocalSystemCompiler();

    /**
     * Returns the Go path to use if {@link #getUseLocalSystemCompiler()} is true.
     */
    abstract Property<File> getGoPath();

    /**
     * Returns a {@link Property} of the minimum Golang version to use. If the version is lower
     * than the one on the system, then it will force-install it, so it can be compatible.
     */
    abstract Property<String> getMinGoVersion();
}
