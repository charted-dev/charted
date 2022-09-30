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

package org.noelware.charted.testing.kubernetes.k3s;

import io.kubernetes.client.util.ClientBuilder;
import io.kubernetes.client.util.KubeConfig;
import java.io.Closeable;
import java.io.IOException;
import java.io.StringReader;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.common.SetOnceGetValue;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.testcontainers.k3s.K3sContainer;
import org.testcontainers.utility.DockerImageName;

/**
 * Represents the container for running Rancher K3S on Docker with Testcontainers.
 */
public class K3SContainer implements Closeable {
    private final SetOnceGetValue<K3sContainer> container = new SetOnceGetValue<>();
    private final Logger log = LoggerFactory.getLogger(K3SContainer.class);

    // only not null when #start is called.
    private K3SKubernetesEnvironment environment;

    /**
     * Returns the Kubernetes environment for this {@link K3SContainer container}. Can be null
     * if the container has never started.
     */
    @Nullable
    public K3SKubernetesEnvironment getEnvironment() {
        return environment;
    }

    public boolean hasStarted() {
        return container.wasSet();
    }

    public void start() throws IOException {
        if (container.wasSet()) return;

        // TODO: allow this to run older versions of Kubernetes
        log.info("Starting K3s container...");

        final var image = DockerImageName.parse("rancher/k3s").withTag("v1.23.12-k3s1");
        final var k3s = new K3sContainer(image);
        k3s.start();

        container.setValue(k3s);
        environment = new K3SKubernetesEnvironment(
                ClientBuilder.kubeconfig(KubeConfig.loadKubeConfig(new StringReader(k3s.getKubeConfigYaml())))
                        .build());
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
        if (!hasStarted()) return;

        container.getValue().close();
        environment = null;
    }
}
