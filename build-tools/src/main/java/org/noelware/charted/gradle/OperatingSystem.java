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
