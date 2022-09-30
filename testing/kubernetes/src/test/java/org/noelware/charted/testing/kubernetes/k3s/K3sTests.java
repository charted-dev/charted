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

import static org.junit.jupiter.api.Assertions.*;

import java.io.IOException;
import java.util.List;
import java.util.Objects;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.condition.DisabledOnOs;
import org.junit.jupiter.api.condition.OS;
import org.noelware.charted.testing.kubernetes.KubernetesEnvironment;
import org.testcontainers.junit.jupiter.Testcontainers;

@DisabledOnOs({OS.WINDOWS, OS.MAC})
@Testcontainers(disabledWithoutDocker = true)
public class K3sTests {
    private static final K3SContainer container = new K3SContainer();

    @BeforeAll
    public static void setup() throws IOException {
        container.start();
    }

    @AfterAll
    public static void stop() throws IOException {
        container.close();
    }

    @DisplayName("Can we collect all namespaces")
    @Test
    public void collectNamespaces() {
        assertTrue(container.hasStarted());

        final KubernetesEnvironment env = container.getEnvironment();
        assertNotNull(env, "Kubernetes container didn't start, so we couldn't grab environment");

        // to be honest, i kinda wish they did a builder api rather than this! but whatever~
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
        assertEquals(List.of("default", "kube-system", "kube-public", "kube-node-lease"), namespaces);
    }
}
