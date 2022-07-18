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

package org.noelware.charted.gradle.plugins.homebrew;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import groovy.text.GStringTemplateEngine;
import groovy.text.TemplateEngine;
import java.io.*;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Map;
import javax.inject.Inject;
import org.gradle.api.DefaultTask;
import org.gradle.api.file.RegularFileProperty;
import org.gradle.api.model.ObjectFactory;
import org.gradle.api.tasks.InputFile;
import org.gradle.api.tasks.TaskAction;
import org.gradle.work.DisableCachingByDefault;

@DisableCachingByDefault(because = "Not worth caching")
public class GenerateHomebrewFormulaTask extends DefaultTask {
    private final TemplateEngine templateEngine = new GStringTemplateEngine();
    private final HttpClient client = HttpClient.newHttpClient();
    private final Gson gson = new Gson();

    @InputFile private RegularFileProperty homebrewFormulaFile;

    @Inject
    public GenerateHomebrewFormulaTask(ObjectFactory objectFactory) {
        this.homebrewFormulaFile = objectFactory.fileProperty();
    }

    public RegularFileProperty getHomebrewFormulaFile() {
        return this.homebrewFormulaFile;
    }

    @TaskAction
    public void execute() throws IOException, ClassNotFoundException {
        var project = getProject().getRootProject();
        var description = project.getDescription();
        var version = project.getVersion().toString();
        var bindings =
                Map.ofEntries(
                        Map.entry("description", description),
                        Map.entry("version", version),
                        Map.entry("url", "https://boop.com"),
                        Map.entry("checksum", "abcdef"),
                        Map.entry(
                                "generatedAt",
                                new SimpleDateFormat("MMM dd, YYYY 'at' HH:mm:ss")
                                        .format(new Date())));

        var homebrewFile = this.homebrewFormulaFile.get().getAsFile();
        var template = templateEngine.createTemplate(new FileReader(homebrewFile));
        var outputDirectory = new File(getProject().getBuildDir(), "generated/homebrew");
        if (!outputDirectory.exists()) {
            Files.createDirectories(outputDirectory.toPath());
        }

        var outputFile = new File(outputDirectory, "charted-server.rb");
        outputFile.createNewFile();

        var output = template.make(bindings).toString();
        try (var os = new FileOutputStream(outputFile)) {
            os.write(output.getBytes(StandardCharsets.UTF_8));
        }
    }

    private JsonObject getJsonObjectFrom(URI url) throws IOException, InterruptedException {
        var request =
                HttpRequest.newBuilder()
                        .GET()
                        .uri(url)
                        .setHeader("User-Agent", "Noelware/charted-server")
                        .build();

        var response = client.send(request, HttpResponse.BodyHandlers.ofInputStream());
        var data = gson.fromJson(new InputStreamReader(response.body()), JsonObject.class);
        response.body().close();

        return data;
    }
}
