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

package org.noelware.charted.gradle.plugins.rpm;

import com.netflix.gradle.plugins.packaging.ProjectPackagingExtension;
import com.netflix.gradle.plugins.rpm.Rpm;
import java.io.File;
import java.util.List;
import org.gradle.api.Action;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.gradle.api.plugins.ExtensionAware;

public class ChartedRpmPlugin implements Plugin<Project> {
    public static final String DESCRIPTION = String.join(
            "\n",
            List.of(
                    "charted-server is the backend server for powering Noelware's Charts Platform.",
                    "It is a fully open sourced and reliable registry for providing Kubernetes Helm Charts",
                    "made in Kotlin made by JetBrains.",
                    "",
                    "The software packaged is from the main repository hosted on GitHub",
                    "and distributed to Noelware's APT repository hosted at",
                    "https://artifacts.noelware.org/deb/charted/server",
                    "",
                    "As the backend only exposes a RESTful transport layer, you can install",
                    "Pak, another component created by Noelware to bring a web UI to visualise",
                    "your Helm Charts. You can learn more here: https://charts.noelware.org/docs/frontend",
                    "",
                    "❯ Want to more information about how Noelware's Charts Platform began? You can read up",
                    "❯ on the documentation site: https://charts.noelware.org/docs",
                    "",
                    "❯ Received any issues while running charted-server? You can read up on the",
                    "❯ common troubleshooting page: https://charts.noelware.org/docs/server/troubleshooting",
                    "",
                    "~ Noelware, LLC. <team@noelware.org> ^~^"));

    @Override
    public void apply(Project project) {
        project.getPluginManager().apply("nebula.ospackage-base");
        ((ExtensionAware) project).getExtensions().configure("ospackage", (Action<ProjectPackagingExtension>)
                extension -> {
                    extension.setMaintainer("Noelware, LLC. <team@noelware.org>");
                    extension.setUrl("https://charts.noelware.org");
                    extension.setPackageDescription(DESCRIPTION);
                    extension.setSummary(
                            "\uD83D\uDCE6 Free, open source, and reliable Helm Chart" + " registry made in Kotlin");

                    var signingPassword = System.getenv("NOELWARE_SIGNING_PASSWORD");
                    if (signingPassword != null) {
                        extension.setSigningKeyPassphrase(signingPassword);
                        extension.setSigningKeyId(System.getenv("NOELWARE_SIGNING_KEY_ID"));

                        var ringPath = System.getenv("NOELWARE_SIGNING_RING_PATH");
                        extension.setSigningKeyRingFile(new File(
                                ringPath != null ? ringPath : System.getProperty("user.home"), ".gnupg/secring.gpg"));
                    }

                    extension.setPermissionGroup("root");
                    extension.setFileMode(0644);
                    extension.setDirMode(0755);
                    extension.setUser("root");
                    extension.into("/etc/noelware/charted/server");
                });

        var tasks = project.getTasks();
        tasks.register("installRpm", Rpm.class, rpm -> {
            rpm.setPackageDescription(
                    "\uD83D\uDCE6 Free, open source, and reliable Helm Chart registry" + " made in Kotlin");
            rpm.setRelease("1");
            rpm.setMaintainer("Noelware, LLC. <team@noelware.org>");
            rpm.setVendor("Noelware, LLC. <team@noelware.org>");
        });
    }
}
