package org.noelware.charted.gradle.plugins.homebrew;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import groovy.text.GStringTemplateEngine;
import groovy.text.SimpleTemplateEngine;
import groovy.text.TemplateEngine;
import org.gradle.api.DefaultTask;
import org.gradle.api.file.RegularFileProperty;
import org.gradle.api.model.ObjectFactory;
import org.gradle.api.tasks.InputFile;
import org.gradle.api.tasks.TaskAction;
import org.gradle.work.DisableCachingByDefault;

import javax.inject.Inject;
import java.io.*;
import java.net.URI;
import java.net.URL;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Map;

@DisableCachingByDefault(because = "Not worth caching")
public class GenerateHomebrewFormulaTask extends DefaultTask {
    private final TemplateEngine templateEngine = new GStringTemplateEngine();
    private final HttpClient client = HttpClient.newHttpClient();
    private final Gson gson = new Gson();

    @InputFile
    private RegularFileProperty homebrewFormulaFile;

    @Inject
    public GenerateHomebrewFormulaTask(ObjectFactory objectFactory) {
        this.homebrewFormulaFile = objectFactory.fileProperty();
    }

    public RegularFileProperty getHomebrewFormulaFile() {
        return this.homebrewFormulaFile;
    }

    // The task is usually ran when we are publishing a new version. This is after our publish
    // worker has created the tarball/zip file and created metadata about it. So, we can
    // simple request `metadata.json` to dl.noelware.org/charted/server/metadata.json!
    @TaskAction
    public void execute() throws IOException, ClassNotFoundException {
        var project = getProject().getRootProject();
        var description = project.getDescription();
        var version = project.getVersion().toString();
        var bindings = Map.ofEntries(
                Map.entry("description", description),
                Map.entry("version", version),
                Map.entry("url", "https://boop.com"),
                Map.entry("checksum", "abcdef"),
                Map.entry("generatedAt", new SimpleDateFormat("MMM dd, YYYY 'at' HH:mm:ss").format(new Date()))
        );

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
                var request = HttpRequest.newBuilder()
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
