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

package org.noelware.charted.testing.kubernetes.minikube;

import java.io.Closeable;
import java.io.IOException;
import org.jetbrains.annotations.NotNull;

/**
 * Represents the manager for handling Minikube. Since this is an external application,
 * we have to manage the lifecycle of the Minikube application.
 */
public interface MinikubeManager extends Closeable {
    /**
     * Returns the {@link org.noelware.charted.testing.kubernetes.KubernetesEnvironment Kubernetes Environment} that
     * can access the Kubernetes client.
     */
    @NotNull
    MinikubeKubernetesEnvironment getEnvironment();

    /**
     * If the Minikube process has started or not.
     */
    boolean hasStarted();

    /**
     * Starts the Minikube process.
     */
    void start() throws IOException, InterruptedException;
}
