package org.noelware.charted.gradle.plugins.nebula;

import com.netflix.gradle.plugins.packaging.ProjectPackagingExtension;
import org.gradle.api.Action;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.gradle.Architecture;

import java.io.File;

/**
 * Gradle plugin to apply the <code>com.netflix.nebula.ospackage-base</code> plugin that
 * provides defaults for charted's Debian and RPM repositories.
 */
public class ChartedNebulaPlugin implements Plugin<Project> {
    private static final String DESCRIPTION = """
    charted-server is a Helm chart registry made in Kotlin for providing a self-managed
    cloud service to host Helm charts easily without configuring a lot of things.

    This software is packaged through Noelware's Artifacts Repository hosted at:
                        https://artifacts.noelware.cloud

    ❯ Documentation: https://charts.noelware.org/docs
    ❯ Issue Tracker: https://github.com/charted-dev/charted/issues

    ~ Noelware, LLC. ^~^
    """.trim();

    @Override
    public void apply(@NotNull Project project) {
        final Architecture arch = Architecture.current();

        project.getPlugins().apply("com.netflix.nebula.ospackage-base");
        project.getExtensions().configure("ospackage", (Action<ProjectPackagingExtension>) (ext) -> {
            ext.setMaintainer("Noelware, LLC. <team@noelware.org>");
            ext.setSummary("\uD83D\uDCE6 You know, for Helm charts?");
            ext.setUrl("https://charts.noelware.org");
            ext.setPackageDescription(DESCRIPTION);
            ext.setArchStr(arch.isX64() ? "amd64" : "arm64");
            ext.requires("temurin-17-jdk");

            final String signingPassword = System.getenv("NOELWARE_SIGNING_PASSWORD");
            if (signingPassword != null) {
                ext.setSigningKeyPassphrase(signingPassword);
                ext.setSigningKeyId(System.getenv("NOELWARE_SIGNING_KEY_ID"));

                var ringPath = System.getenv("NOELWARE_SIGNING_RING_PATH");
                ext.setSigningKeyRingFile(
                        new File(ringPath != null ? ringPath : System.getProperty("user.home"), ".gnupg/secring.gpg"));
            }
        });
    }
}
