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

package org.noelware.charted.gradle;

import org.jetbrains.annotations.NotNull;

/**
 * Represents the current operating system.
 * @since 11.07.2022
 * @author Noel <cutie@floofy.dev>
 */
public enum OperatingSystem {
    WINDOWS,
    MACOS,
    LINUX;

    /**
     * Returns the current operating system.
     */
    public static @NotNull OperatingSystem current() {
        var os = System.getProperty("os.name", "");
        if (os.startsWith("Windows")) return OperatingSystem.WINDOWS;
        if (os.startsWith("Mac")) return OperatingSystem.MACOS;
        if (os.startsWith("Linux")) return OperatingSystem.LINUX;

        throw new RuntimeException(String.format("Unknown operating system: [%s]", os));
    }

    public boolean isWindows() {
        return this == OperatingSystem.WINDOWS;
    }

    public boolean isMacOS() {
        return this == OperatingSystem.MACOS;
    }

    public boolean isLinux() {
        return this == OperatingSystem.LINUX;
    }
}
