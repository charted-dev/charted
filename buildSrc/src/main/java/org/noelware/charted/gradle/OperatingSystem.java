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
