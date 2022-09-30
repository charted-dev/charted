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

package org.noelware.charted.testing.kubernetes.junit;

import java.io.IOException;
import java.lang.annotation.Annotation;
import java.util.Optional;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.extension.*;
import org.junit.platform.commons.support.AnnotationSupport;
import org.junit.platform.commons.support.SearchOption;
import org.noelware.charted.common.OperatingSystem;
import org.noelware.charted.testing.kubernetes.minikube.MinikubeManager;
import org.noelware.charted.testing.kubernetes.minikube.downloader.DefaultMinikubeDownloader;
import org.noelware.charted.testing.kubernetes.minikube.internal.DefaultMinikubeManager;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * Extension class for JUnit to disable tests if Minikube is not installed, or if the
 * {@link InstallMinikubeIfNotFound} annotation was present, it will install Minikube in a temp
 * directory, and uninstall it when the tests are done.
 */
public class NoelKubeExtension implements BeforeAllCallback, AfterAllCallback, ExecutionCondition {
    public static final ExtensionContext.Namespace NAMESPACE =
            ExtensionContext.Namespace.create(NoelKubeExtension.class);
    private final Logger log = LoggerFactory.getLogger(NoelKubeExtension.class);

    /**
     * Callback that is invoked once <em>after</em> all tests in the current
     * container.
     *
     * @param context the current extension context; never {@code null}
     */
    @Override
    public void afterAll(ExtensionContext context) throws Exception {
        final var store = context.getStore(NAMESPACE);
        final MinikubeManager manager = store.get("minikube:manager", DefaultMinikubeManager.class);
        // final var path = store.get("minikube:install", String.class);

        manager.close();
    }

    /**
     * Callback that is invoked once <em>before</em> all tests in the current
     * container.
     *
     * @param context the current extension context; never {@code null}
     */
    @Override
    public void beforeAll(ExtensionContext context) throws Exception {
        log.info("Creating Minikube manager...");

        final var store = context.getStore(NAMESPACE);
        final var path = store.get("minikube:install", String.class);
        final MinikubeManager manager = new DefaultMinikubeManager(path);
        manager.start();

        store.getOrComputeIfAbsent("minikube:manager", k -> manager);
    }

    /**
     * Evaluate this condition for the supplied {@link ExtensionContext}.
     *
     * <p>An {@linkplain ConditionEvaluationResult#enabled enabled} result
     * indicates that the container or test should be executed; whereas, a
     * {@linkplain ConditionEvaluationResult#disabled disabled} result
     * indicates that the container or test should not be executed.
     *
     * @param context the current extension context; never {@code null}
     * @return the result of evaluating this condition; never {@code null}
     */
    @Override
    public ConditionEvaluationResult evaluateExecutionCondition(ExtensionContext context) {
        if (OperatingSystem.current().isMacOS())
            return ConditionEvaluationResult.disabled("Host system is macOS and Minikube is not supported.");

        // Start a process to call "minikube --help" to determine if it is installed
        // or not.
        boolean enabled = false;
        try {
            final var proc = new ProcessBuilder("minikube", "--help")
                    .redirectError(ProcessBuilder.Redirect.PIPE)
                    .redirectOutput(ProcessBuilder.Redirect.PIPE)
                    .start();
            if (!proc.waitFor(3, TimeUnit.MINUTES))
                return ConditionEvaluationResult.disabled("Running `minikube --help` too way longer than expected");
            if (proc.exitValue() == 0) enabled = true;
        } catch (Exception e) {
            final var message = e.getMessage();
            if (message != null && !message.contains("No such file or directory")) {
                throw new RuntimeException(e);
            }
        }

        final var store = context.getStore(NAMESPACE);
        if (enabled) {
            store.getOrComputeIfAbsent("minikube:install", k -> "minikube");
            return ConditionEvaluationResult.enabled("Minikube was found");
        }

        final var disableAnno = findAnnotation(context, DisabledIfNoMinikube.class);
        if (disableAnno.isPresent())
            return ConditionEvaluationResult.disabled("Test is disabled because @DisabledIfNoMinikube was present");

        final var installIfNotFound = findAnnotation(context, InstallMinikubeIfNotFound.class);
        if (installIfNotFound.isPresent()) {
            try {
                final var installPath = DefaultMinikubeDownloader.download();
                store.getOrComputeIfAbsent("minikube:install", k -> installPath);
            } catch (IOException | InterruptedException e) {
                log.error("Unable to install Minikube locally", e);
                return ConditionEvaluationResult.disabled(
                        "Unable to install Minikube locally when @InstallMinikubeIfNotFound was present");
            }
        }

        // it's fun to be quirky sometimes
        return ConditionEvaluationResult.enabled("Running tests either way, expect errors!");
    }

    private <T extends Annotation> Optional<T> findAnnotation(ExtensionContext context, Class<T> annotationClass) {
        var current = Optional.of(context);
        while (current.isPresent()) {
            final var testClass = current.get().getTestClass().orElse(null);
            if (testClass == null) return Optional.empty();

            final Optional<T> anno = AnnotationSupport.findAnnotation(
                    testClass, annotationClass, SearchOption.INCLUDE_ENCLOSING_CLASSES);
            if (anno.isPresent()) return anno;

            current = current.get().getParent();
        }

        return Optional.empty();
    }
}
