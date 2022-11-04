package org.noelware.charted.gradle.plugins.golang;

import org.gradle.api.Project;
import org.gradle.api.provider.Property;
import org.noelware.charted.gradle.Architecture;
import org.noelware.charted.gradle.OperatingSystem;

public class GolangExtension {
    private final Property<String> downloadUrl;
    private final Property<Boolean> useLocalInstall;
    private final Property<String> version;

    public GolangExtension(Project project) {
        this.useLocalInstall = project.getObjects().property(Boolean.class);
        this.downloadUrl     = project.getObjects().property(String.class);
        this.version         = project.getObjects().property(String.class);

        useLocalInstall.convention(false);
        version.convention("1.19.2");
        downloadUrl.convention("https://go.dev/dl/go%s.%s-%s.tar.gz".formatted(
                version.get(),
                OperatingSystem.current().getName(),
                Architecture.current().isX64() ? "amd64" : "arm64"
        ));
    }

    public Property<String> getDownloadUrl() {
        return downloadUrl;
    }

    public Property<Boolean> getUseLocalInstall() {
        return useLocalInstall;
    }

    public Property<String> getVersion() {
        return version;
    }
}
