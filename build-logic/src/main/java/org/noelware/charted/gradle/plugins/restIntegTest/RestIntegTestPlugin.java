/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.gradle.plugins.restIntegTest;

import java.util.Map;
import javax.inject.Inject;
import org.gradle.api.Plugin;
import org.gradle.api.Project;
import org.gradle.api.artifacts.Configuration;
import org.gradle.api.internal.project.ProjectInternal;
import org.gradle.api.internal.tasks.JvmConstants;
import org.gradle.api.model.ObjectFactory;
import org.gradle.api.plugins.JavaLibraryPlugin;
import org.gradle.api.plugins.JavaPluginExtension;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.gradle.plugins.restIntegTest.internal.DefaultRestIntegTestSourceDirectorySet;

public class RestIntegTestPlugin implements Plugin<Project> {
    public static final String INTEGRATION_TEST_CONFIGURATION_NAME = "restIntegTest";
    private final ObjectFactory objectFactory;

    @Inject
    public RestIntegTestPlugin(ObjectFactory objectFactory) {
        this.objectFactory = objectFactory;
    }

    @Override
    public void apply(@NotNull Project project) {
        project.getPlugins().apply(JavaLibraryPlugin.class);

        final Configuration configuration = ((ProjectInternal) project)
                .getConfigurations()
                .resolvableBucket(INTEGRATION_TEST_CONFIGURATION_NAME)
                .setVisible(false);
        configuration.defaultDependencies((deps) -> {
            // Only add the Kotlin DSL.
            deps.add(project.getDependencies()
                    .create(project.getDependencies().project(Map.of("path", ":testing:restIntegTest:kotlin-dsl"))));
        });

        // We only want the testImplementation dependencies
        Configuration apiConfiguration =
                project.getConfigurations().getByName(JvmConstants.TEST_IMPLEMENTATION_CONFIGURATION_NAME);
        apiConfiguration.extendsFrom(configuration);

        project.getExtensions()
                .getByType(JavaPluginExtension.class)
                .getSourceSets()
                .all(sourceSet -> {
                    final RestIntegTestSourceDirectorySet ourSourceSet = createSourceDirectory(objectFactory);
                    sourceSet.getExtensions().add(RestIntegTestSourceDirectorySet.class, "restIntegTest", ourSourceSet);

                    ourSourceSet.srcDir("src/" + sourceSet.getName() + "/restIntegTest");
                    sourceSet.getAllSource().source(ourSourceSet);

                    final String taskName = sourceSet.getTaskName("test", "IntegTest");
                    project.getTasks().register(taskName, RestIntegTask.class, task -> {
                        task.setDescription("Runs the integration test runner");
                        task.setClasspath(configuration);
                    });
                });
    }

    private static RestIntegTestSourceDirectorySet createSourceDirectory(ObjectFactory objectFactory) {
        RestIntegTestSourceDirectorySet sourceSet = objectFactory.newInstance(
                DefaultRestIntegTestSourceDirectorySet.class,
                objectFactory.sourceDirectorySet("restIntegTest", "Rest Integration Test Sources"));
        sourceSet.getFilter().include("**/*.test.kts");

        return sourceSet;
    }
}
