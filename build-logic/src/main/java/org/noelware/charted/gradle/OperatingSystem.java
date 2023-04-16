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

package org.noelware.charted.gradle;

import org.jetbrains.annotations.NotNull;

/**
 * Represents the host operating system.
 */
public enum OperatingSystem {
    WINDOWS,
    LINUX,
    MACOS,
    UNSUPPORTED;

    /**
     * @return the host operating system, never null
     */
    @NotNull
    public static OperatingSystem current() {
        final String os = System.getProperty("os.name");
        if (os.equals("Linux")) return LINUX;
        if (os.equals("Mac OS X")) return MACOS;
        if (os.startsWith("Windows")) return WINDOWS;

        return UNSUPPORTED;
    }

    /**
     * @return the host architecture's prettified name, never null
     */
    @NotNull
    public String getName() {
        return switch (this) {
            case UNSUPPORTED -> "unsupported";
            case WINDOWS -> "windows";
            case MACOS -> "macos";
            case LINUX -> "linux";
        };
    }

    /**
     * @return whether if the host os is unsupported
     */
    public boolean isUnsupported() {
        return this == UNSUPPORTED;
    }

    /**
     * @return whether if the host os is Windows
     */
    public boolean isWindows() {
        return this == WINDOWS;
    }

    /**
     * @return whether if the host os is macOS
     */
    public boolean isMacOS() {
        return this == MACOS;
    }

    /**
     * @return whether if the host os is Linux
     */
    public boolean isLinux() {
        return this == LINUX;
    }

    /**
     * @return whether if the host os is Unix-like
     */
    public boolean isUnix() {
        return isLinux() || isMacOS();
    }

    @Override
    public String toString() {
        return "OperatingSystem(%s)".formatted(getName());
    }
}
