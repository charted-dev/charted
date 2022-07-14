package org.noelware.charted.gradle.plugins.rpm;

import com.netflix.gradle.plugins.packaging.ProjectPackagingExtension;
import com.netflix.gradle.plugins.rpm.Rpm;
import org.gradle.api.Action;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.gradle.api.plugins.ExtensionAware;

import java.io.File;

public class ChartedRPMPlugin implements Plugin<Project> {
    @Override
    public void apply(Project project) {
        project.getPluginManager().apply("nebula.ospackage-base");
        ((ExtensionAware) this).getExtensions().configure("ospackage", new Action<ProjectPackagingExtension>() {
            @Override
            public void execute(ProjectPackagingExtension extension) {
                extension.setMaintainer("Noelware, LLC. <team@noelware.org>");
                extension.setSummary("\uD83D\uDCE6 Free, open source, and reliable Helm Chart registry made in Kotlin");
                extension.setUrl("https://charts.noelware.org");

                extension.setPackageDescription("""
                charted-server is the main backend server for the charted Platform created and
                maintained by Noelware. It serves as a free, reliable, and open source Helm Chart
                registry server for public or private instances to distribute Helm Charts easily
                while being easily configurable.
                
                This only exposes a RESTful transport layer, if you wish to have a frontend UI to
                visualise your data or how the server is dying, you can install Pak, which is the
                official frontend UI for charted-server.
                ❯ https://charts.noelware.org/docs/pak
                
                If you want more information on how the charted Platform works or why we choose
                Kotlin? You can read up on via our documentation site:
                ❯ https://charts.noelware.org/docs
                
                Any issues pop up while running charted-server? Security issues? Data loss while
                upgrading? You can report it to the charted team at Noelware via GitHub Issues:
                ❯ https://github.com/charted-dev/charted/issues
                
                ~ Noelware, LLC. ^-^
                """);

                var signingPassword = System.getenv("NOELWARE_SIGNING_PASSWORD");
                if (signingPassword != null) {
                    extension.setSigningKeyPassphrase(signingPassword);
                    extension.setSigningKeyId(System.getenv("NOELWARE_SIGNING_KEY_ID"));

                    var ringPath = System.getenv("NOELWARE_SIGNING_RING_PATH");
                    extension.setSigningKeyRingFile(new File(ringPath != null ? ringPath : System.getProperty("user.home"), ".gnupg/secring.gpg"));
                }

                extension.setPermissionGroup("root");
                extension.setFileMode(0644);
                extension.setDirMode(0755);
                extension.setUser("root");
                extension.into("/etc/noelware/charted/server");
            }
        });

        var tasks = project.getTasks();
        tasks.register("installRpm", Rpm.class, new Action<Rpm>() {
            @Override
            public void execute(Rpm rpm) {
                rpm.setPackageDescription("\uD83D\uDCE6 Free, open source, and reliable Helm Chart registry made in Kotlin");
                rpm.setRelease("1");
                rpm.setMaintainer("Noelware, LLC. <team@noelware.org>");
                rpm.setVendor("Noelware, LLC. <team@noelware.org>");
            }
        });
    }
}
