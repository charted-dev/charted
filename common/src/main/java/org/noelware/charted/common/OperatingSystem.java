/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.common;

import org.jetbrains.annotations.NotNull;

/**
 * Represents the current host operating system.
 *
 * @since 11.07.2022
 * @author Noel <cutie@floofy.dev>
 */
public enum OperatingSystem {
    WINDOWS,
    MACOS,
    LINUX;

    /** Returns the current operating system. */
    public static @NotNull OperatingSystem current() {
        final String os = System.getProperty("os.name", "");
        if (os.startsWith("Windows")) return WINDOWS;
        if (os.startsWith("Mac")) return MACOS;
        if (os.startsWith("Linux")) return LINUX;

        throw new IllegalStateException("Unknown architecture [%s]".formatted(os));
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

    public boolean isUnix() {
        return isLinux() || isMacOS();
    }

    public String key() {
        return switch (this) {
            case WINDOWS -> "windows";
            case LINUX -> "linux";
            case MACOS -> "darwin";
        };
    }
}
