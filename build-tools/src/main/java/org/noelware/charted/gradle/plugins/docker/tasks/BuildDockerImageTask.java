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

package org.noelware.charted.gradle.plugins.docker.tasks;

import com.google.gson.Gson;
import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import java.io.ByteArrayOutputStream;
import java.util.Map;
import javax.inject.Inject;
import org.gradle.api.DefaultTask;
import org.gradle.api.file.DirectoryProperty;
import org.gradle.api.model.ObjectFactory;
import org.gradle.api.provider.Property;
import org.gradle.api.tasks.TaskAction;
import org.gradle.process.ExecOperations;
import org.gradle.work.DisableCachingByDefault;
import org.gradle.workers.WorkAction;
import org.gradle.workers.WorkParameters;
import org.gradle.workers.WorkerExecutor;
import org.noelware.charted.gradle.OperatingSystem;
import org.noelware.charted.gradle.plugins.docker.Dockerfile;

@DisableCachingByDefault(because = "Docker already caches if enabled, so not worth caching.")
public class BuildDockerImageTask extends DefaultTask {
    private final WorkerExecutor workerExecutor;
    private final Gson gson = new Gson();

    public final Property<Dockerfile> dockerfile;
    public final DirectoryProperty dockerContext;
    public final Property<Boolean> useDockerBuildx;
    public final Property<Boolean> shouldCache;
    public final DirectoryProperty cacheFrom;
    public final DirectoryProperty cacheTo;

    @Inject
    public BuildDockerImageTask(WorkerExecutor executor, ObjectFactory objectFactory) {
        this.useDockerBuildx = objectFactory.property(boolean.class);
        this.workerExecutor = executor;
        this.dockerContext = objectFactory.directoryProperty();
        this.shouldCache = objectFactory.property(boolean.class);
        this.dockerfile = objectFactory.property(Dockerfile.class);
        this.cacheFrom = objectFactory.directoryProperty();
        this.cacheTo = objectFactory.directoryProperty();

        this.shouldCache.convention(true);
        setGroup("build");
    }

    private String replaceLast(String data, String toReplace, String replacement) {
        var pos = data.lastIndexOf(toReplace);
        return pos > -1 ? data.substring(0, pos) + replacement + data.substring(pos + toReplace.length()) : data;
    }

    @TaskAction
    public void execute() {
        var current = OperatingSystem.current();
        if (dockerfile.isPresent() && dockerfile.get().isWindows()) {
            if (!current.isWindows()) throw new IllegalStateException("You must be using Windows to build this image!");
        }

        var shouldUseBuildx = false;
        if (useDockerBuildx.get()) {
            var log = getLogger();
            log.lifecycle("Checking if `buildx` is available...");

            var stdout = new ByteArrayOutputStream();
            var result = getProject().exec((spec) -> {
                spec.setCommandLine("docker");
                spec.args("info", "--format", "\"{{json .ClientInfo.Plugins}}\"");
                spec.setStandardOutput(stdout);
            });

            if (result.getExitValue() != 0) {
                log.lifecycle("Unable to run 'docker info --format \"{{json .ClientInfo.Plugins}}\"', not"
                        + " using Docker buildx!");
            } else {
                var stdoutData = replaceLast(stdout.toString().replaceFirst("\"", ""), "\"", "");
                var plugins = gson.fromJson(stdoutData, JsonArray.class);
                JsonObject found = null;

                for (JsonElement plugin : plugins) {
                    if (plugin.isJsonObject()) {
                        var data = plugin.getAsJsonObject();
                        var name = data.getAsJsonPrimitive("Name").getAsString();
                        if (name.equals("buildx")) {
                            found = data;
                            break;
                        }
                    }
                }

                if (found == null) {
                    log.lifecycle("Docker Buildx plugin wasn't found in client tree, skipping.");
                } else {
                    log.lifecycle(String.format(
                            "Found Docker Buildx plugin %s in [%s]",
                            found.getAsJsonPrimitive("Version").getAsString(),
                            found.getAsJsonPrimitive("Path").getAsString()));

                    shouldUseBuildx = true;
                }
            }
        }

        boolean finalShouldUseBuildx = shouldUseBuildx;
        workerExecutor.noIsolation().submit(BuildDockerImage.class, params -> {
            params.getDockerfile().set(dockerfile);
            params.getProjectVersion()
                    .set((String) getProject().getRootProject().getVersion());

            params.getCacheFrom().set(cacheFrom);
            params.getCacheTo().set(cacheTo);
            params.getDockerContext().set(dockerContext);
            params.getShouldCache().set(shouldCache);
            params.getDockerBuildxAvailable().set(finalShouldUseBuildx);
        });

        var isCi = System.getenv("CI") != null;
        if (isCi) {
            var df = dockerfile.get();
            var images = String.join(" ", df.tags());

            System.out.printf("::set-output name=images::%s%n", images);
        }
    }

    public abstract static class BuildDockerImage implements WorkAction<Parameters> {
        private final ExecOperations operations;

        @Inject
        public BuildDockerImage(ExecOperations operations) {
            this.operations = operations;
        }

        @Override
        public void execute() {
            var parameters = getParameters();
            var dockerfile = parameters.getDockerfile().get();
            var cacheFrom = parameters.getCacheFrom().getOrNull();
            var cacheTo = parameters.getCacheTo().getOrNull();
            var dockerContext = parameters.getDockerContext().getOrNull();
            var shouldCache = parameters.getShouldCache().getOrElse(false);
            var isDockerBuildxAvailable = parameters.getDockerBuildxAvailable().getOrElse(false);

            try {
                var result = this.operations.exec((spec) -> {
                    spec.setCommandLine("docker");
                    spec.setStandardOutput(System.out);
                    spec.setErrorOutput(System.err);
                    spec.setIgnoreExitValue(true);

                    if (isDockerBuildxAvailable) {
                        spec.args("buildx", "build");
                    } else {
                        spec.args("build");
                    }

                    if (dockerContext != null) {
                        spec.args(dockerContext.getAsFile().getAbsolutePath());
                    } else {
                        spec.args(".");
                    }

                    spec.args("--platform", dockerfile.platform());
                    spec.args("-f", dockerfile.path());
                    for (String tag : dockerfile.tags()) {
                        spec.args("--tag", tag);
                    }

                    if (cacheFrom != null) {
                        spec.args("--cache-from", cacheFrom.getAsFile().getAbsolutePath());
                    }

                    if (cacheTo != null) {
                        spec.args("--cache-to", cacheTo.getAsFile().getAbsolutePath());
                    }

                    if (!shouldCache) {
                        spec.args("--no-cache");
                    }

                    for (Map.Entry<String, String> buildArgs :
                            dockerfile.buildArguments().entrySet()) {
                        spec.args("--build-arg", String.format("%s=%s", buildArgs.getKey(), buildArgs.getValue()));
                    }
                });

                if (result.getExitValue() != 0) {
                    throw new Exception("Unable to build Docker image (^ view output above to see why!)");
                }
            } catch (Exception e) {
                throw new RuntimeException("Unable to build Docker image", e);
            }
        }
    }

    public interface Parameters extends WorkParameters {
        Property<Boolean> getDockerBuildxAvailable();

        Property<Dockerfile> getDockerfile();

        DirectoryProperty getDockerContext();

        Property<String> getProjectVersion();

        Property<Boolean> getShouldCache();

        DirectoryProperty getCacheFrom();

        DirectoryProperty getCacheTo();
    }
}
