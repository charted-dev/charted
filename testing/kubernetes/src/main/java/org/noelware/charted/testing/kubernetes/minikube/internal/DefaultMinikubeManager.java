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

package org.noelware.charted.testing.kubernetes.minikube.internal;

import io.kubernetes.client.util.ClientBuilder;
import io.kubernetes.client.util.KubeConfig;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.testing.kubernetes.KubernetesEnvironment;
import org.noelware.charted.testing.kubernetes.exceptions.GenericStdoutException;
import org.noelware.charted.testing.kubernetes.minikube.MinikubeKubernetesEnvironment;
import org.noelware.charted.testing.kubernetes.minikube.MinikubeManager;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class DefaultMinikubeManager implements MinikubeManager {
    private final String minikubePath;
    private final AtomicBoolean started = new AtomicBoolean(false);
    private final Logger log = LoggerFactory.getLogger(DefaultMinikubeManager.class);

    private MinikubeKubernetesEnvironment environment;

    public DefaultMinikubeManager(String minikubePath) {
        this.minikubePath = minikubePath;
    }

    /**
     * Returns the {@link KubernetesEnvironment Kubernetes Environment} that
     * can access the Kubernetes client.
     */
    @Override
    public @NotNull MinikubeKubernetesEnvironment getEnvironment() {
        if (!hasStarted()) throw new IllegalStateException("Can't retrieve environment if Minikube hasn't started!");

        assert environment != null : "environment was not set!!!!!!!";
        return environment;
    }

    /**
     * If the Minikube process has started or not.
     */
    @Override
    public boolean hasStarted() {
        return started.get();
    }

    /**
     * Starts the Minikube process.
     */
    @Override
    public void start() throws IOException, InterruptedException {
        if (started.compareAndSet(false, true)) {
            log.info("Starting the Minikube process...");

            // Start the Minikube process! This will run `minikube start`, so you will
            // need to download it if it is not available on the system.
            final var proc = new ProcessBuilder(minikubePath, "start")
                    .redirectOutput(ProcessBuilder.Redirect.PIPE)
                    .redirectError(ProcessBuilder.Redirect.PIPE)
                    .start();

            // Minikube shouldn't really take that long unless you have a slow internet
            // connection.
            if (!proc.waitFor(5, TimeUnit.MINUTES)) {
                throw new IllegalStateException("Running `minikube start` took more than 5 minutes.");
            }

            log.info("Received exit code [{}] on \"minikube start\"", proc.exitValue());
            if (proc.exitValue() != 0) {
                throw new GenericStdoutException(
                        "Received exit code [%d] when running \"minikube start\"", proc.getInputStream());
            }

            // create the environment
            final var config = KubeConfig.loadKubeConfig(new FileReader("%s%s.kube%sconfig"
                    .formatted(System.getProperty("user.home", ""), File.separator, File.separator)));

            config.setContext("minikube");
            final var client = ClientBuilder.kubeconfig(config).build();

            environment = new MinikubeKubernetesEnvironment(client);
        }
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * <p> As noted in {@link AutoCloseable#close()}, cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * <em>mark</em> the {@code Closeable} as closed, prior to throwing
     * the {@code IOException}.
     *
     * @throws IOException if an I/O error occurs
     */
    @Override
    public void close() throws IOException {
        if (!started.get()) return;

        log.warn("Shutting down the Minikube process!");

        // Start the Minikube process! This will run `minikube stop`, so you will
        // need to download it if it is not available on the system. The unit tests
        // will download a local Minikube binary if it was not found on the host system
        final var proc = new ProcessBuilder(minikubePath, "stop")
                .redirectOutput(ProcessBuilder.Redirect.PIPE)
                .redirectError(ProcessBuilder.Redirect.PIPE)
                .start();

        // Sometimes, tearing down Minikube might take some time, so let's wait
        // 10 minutes.
        try {
            if (!proc.waitFor(10, TimeUnit.MINUTES)) {
                throw new IllegalStateException("Running `minikube stop` took more than 10 minutes.");
            }
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }

        log.info("Received exit code [{}] on \"minikube stop\"", proc.exitValue());
        if (proc.exitValue() != 0) {
            throw new GenericStdoutException(
                    "Received exit code [%d] when running \"minikube stop\"", proc.getInputStream());
        }
    }
}
