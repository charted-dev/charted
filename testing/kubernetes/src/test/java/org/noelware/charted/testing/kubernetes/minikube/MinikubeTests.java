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

import static org.junit.jupiter.api.Assertions.*;
import static org.noelware.charted.testing.kubernetes.Assertions.*;

import java.util.Objects;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.condition.DisabledOnOs;
import org.junit.jupiter.api.condition.OS;
import org.junit.jupiter.params.ParameterizedTest;
import org.noelware.charted.testing.kubernetes.KubernetesEnvironment;
import org.noelware.charted.testing.kubernetes.junit.InstallMinikubeIfNotFound;
import org.noelware.charted.testing.kubernetes.junit.MinikubeManagerSource;
import org.noelware.charted.testing.kubernetes.minikube.internal.DefaultMinikubeManager;

// macos is already disabled so !
@DisabledOnOs({OS.WINDOWS})
@InstallMinikubeIfNotFound
public class MinikubeTests {
    @ParameterizedTest
    @MinikubeManagerSource
    @DisplayName("Can we list all namespaces")
    public void listNamespaces(DefaultMinikubeManager manager) {
        final KubernetesEnvironment env = manager.getEnvironment();
        final var ns = assertDoesNotThrow(
                () -> env.getCoreV1Api().listNamespace(null, true, null, null, null, 100, null, null, 10, false));

        final var items = ns.getItems();
        final var namespaces = items.stream()
                .map(s -> {
                    final var metadata = s.getMetadata();
                    if (metadata != null) {
                        return metadata.getName();
                    }

                    return null;
                })
                .filter(Objects::nonNull)
                .toList();

        assertEquals(items.size(), 4);
        assertHasElements(namespaces, "default", "kube-node-lease", "kube-public", "kube-system");
    }
}
