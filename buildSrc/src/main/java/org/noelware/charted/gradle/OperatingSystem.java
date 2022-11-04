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

package org.noelware.charted.gradle;

/**
 * Represents the host operating system.
 */
public enum OperatingSystem {
    WINDOWS,
    LINUX,
    MACOS,
    UNSUPPORTED;

    public static OperatingSystem current() {
        final String os = System.getProperty("os.name");
        if (os.equals("Linux")) return LINUX;
        if (os.equals("Mac OS X")) return MACOS;
        if (os.startsWith("Windows")) return WINDOWS;

        return UNSUPPORTED;
    }

    public String getName() {
        return switch (this) {
            case UNSUPPORTED -> null;
            case WINDOWS -> "windows";
            case MACOS -> "macos";
            case LINUX -> "linux";
        };
    }

    public boolean isUnsupported() {
        return this == UNSUPPORTED;
    }

    public boolean isWindows() {
        return this == WINDOWS;
    }

    public boolean isMacOS() {
        return this == MACOS;
    }

    public boolean isLinux() {
        return this == LINUX;
    }

    public boolean isUnix() {
        return isLinux() || isMacOS();
    }

    @Override
    public String toString() {
        return "OperatingSystem(%s)".formatted(getName() == null ? "unsupported" : getName());
    }
}
