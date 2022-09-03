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

package org.noelware.charted.gradle.plugins.aur;

import groovy.text.SimpleTemplateEngine;
import groovy.text.TemplateEngine;
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import javax.inject.Inject;
import org.gradle.api.DefaultTask;
import org.gradle.api.file.RegularFileProperty;
import org.gradle.api.model.ObjectFactory;
import org.gradle.api.tasks.InputFile;
import org.gradle.api.tasks.TaskAction;
import org.gradle.process.ExecOperations;
import org.gradle.work.DisableCachingByDefault;
import org.noelware.charted.gradle.Architecture;

@DisableCachingByDefault(because = "Not worth caching")
public class GeneratePkgBuildTask extends DefaultTask {
    private final TemplateEngine engine = new SimpleTemplateEngine();
    private final ExecOperations execOperations;

    @InputFile
    private final RegularFileProperty templateFile;

    @Inject
    public GeneratePkgBuildTask(ObjectFactory objectFactory, ExecOperations operations) {
        this.execOperations = operations;
        this.templateFile = objectFactory.fileProperty();
    }

    public RegularFileProperty getTemplateFile() {
        return templateFile;
    }

    @TaskAction
    public void execute() throws IOException, InterruptedException, ClassNotFoundException {
        final var project = getProject().getRootProject();
        final var version = (String) project.getVersion();

        // Get the checksum from the downloads repository
        final var arch = Architecture.current();
        final var checksum = "beepboop";
        //        final var checksum = HttpRequest.text(
        //                "https://dl.noelware.org/charted/server/%s/%s/charted-server.tar.gz.sha256".formatted(version,
        // arch));

        getLogger().lifecycle("Received checksum for charted-server.tar.gz.sha256 [{}]", checksum);

        final var file = templateFile.getAsFile().get();
        final var od = new File(getProject().getBuildDir(), "generated/aur");
        if (!od.exists()) {
            Files.createDirectories(od.toPath());
        }

        try (final var is = new FileInputStream(file)) {
            final var destination = new File(od, "PKGBUILD");
            destination.createNewFile();

            var output = new String(is.readAllBytes());
            output = output.replace("${version}", "1.0");
            output = output.replace("${checksum}", checksum);

            try (final var os = new FileOutputStream(destination)) {
                os.write(output.getBytes(StandardCharsets.UTF_8));
            }
        }

        // since we wrote it, let's create a virtual environment,
        // it'll live in build/generated/aur since the AUR doesn't
        // accept files in other directories in the current working
        // directory.
        final var makepkgsumsResult = execOperations.exec(spec -> {
            spec.commandLine("makepkg");
            spec.args("-si");
            spec.setWorkingDir(od);
            spec.setIgnoreExitValue(true);
            spec.setStandardOutput(System.out);
            spec.setErrorOutput(System.err);
        });

        if (makepkgsumsResult.getExitValue() != 0) {
            getLogger().lifecycle("Couldn't run 'makepkg' command in directory [{}]", od);
        }
    }
}
