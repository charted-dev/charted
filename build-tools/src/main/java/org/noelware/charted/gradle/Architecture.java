package org.noelware.charted.gradle;

import org.jetbrains.annotations.NotNull;

/**
 * Represents the host's CPU architecture.
 */
public enum Architecture {
    X64,
    AARCH64,
    UNSUPPORTED;

    /**
     * @return the current host architecture, never null
     */
    @NotNull
    public static Architecture current() {
        final String arch = System.getProperty("os.arch");
        return switch (arch) {
            case "x86_64", "amd64" -> X64;
            case "aarch64", "arm64" -> AARCH64;
            default -> UNSUPPORTED;
        };
    }

    /**
     * @return if the host architecture is x86_64
     */
    public boolean isX64() {
        return this == X64;
    }

    /**
     * @return whether if the host architecture is aarch64
     */
    public boolean isArm64() {
        return this == AARCH64;
    }

    /**
     * @return whether if the host architecture is unsupported
     */
    public boolean isUnsupported() {
        return this == UNSUPPORTED;
    }

    /**
     * @return the host architecture's prettified name, never null
     */
    @NotNull
    public String getName() {
        return switch (this) {
            case UNSUPPORTED -> "unsupported";
            case AARCH64 -> "aarch64";
            case X64 -> "x86_64";
        };
    }

    @Override
    public String toString() {
        return "Architecture(%s)".formatted(getName());
    }
}
