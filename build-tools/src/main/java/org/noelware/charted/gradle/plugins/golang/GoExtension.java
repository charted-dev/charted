package org.noelware.charted.gradle.plugins.golang;

import org.gradle.api.provider.Property;

import java.io.File;

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
