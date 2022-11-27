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

package org.noelware.charted.modules.helm.charts.tests

import io.kubernetes.client.openapi.ApiClient
import io.kubernetes.client.openapi.ApiException
import io.kubernetes.client.openapi.apis.CoreV1Api
import io.kubernetes.client.openapi.models.V1Namespace
import io.kubernetes.client.openapi.models.V1ObjectMeta
import io.kubernetes.client.util.ClientBuilder
import io.kubernetes.client.util.KubeConfig
import org.junit.jupiter.api.Disabled
import org.junit.jupiter.api.Test
import org.slf4j.LoggerFactory
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.k3s.K3sContainer
import org.testcontainers.utility.DockerImageName
import java.io.StringReader
import kotlin.test.assertEquals
import kotlin.test.fail

@Testcontainers(disabledWithoutDocker = true)
@Disabled("K3s container being weird :(")
class HelmChartModuleTests {
    @Container
    private val k3sContainer: K3sContainer = K3sContainer(DockerImageName.parse("rancher/k3s:v1.23.12-k3s1")).apply {
        withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("com.rancher.k3s")))
    }

    private val client: ApiClient by lazy {
        ClientBuilder.kubeconfig(KubeConfig.loadKubeConfig(StringReader(k3sContainer.kubeConfigYaml))).build()
    }

    private fun <T> List<T>.assertHasElements(vararg elements: T) {
        if (size != elements.size) throw AssertionError("size != elements.size ($size != ${elements.size})")
        for (el in this) {
            if (!elements.contains(el)) {
                throw AssertionError("Element $el doesn't exist in element tree")
            }
        }
    }

    @Test
    fun `can we connect to k3s container`() {
        val api = CoreV1Api(client)
        val namespaces = api.listNamespace(null, true, null, null, null, 100, null, null, 10, false)
        val items = namespaces.items.mapNotNull { it.metadata?.name }

        assertEquals(4, items.size)
        items.assertHasElements("default", "kube-node-lease", "kube-public", "kube-system")
    }

    @Test
    fun `can we create the charted-system namespace`() {
        val api = CoreV1Api(client)
        val ns = V1Namespace().apply {
            metadata(V1ObjectMeta().name("charted-system"))
        }

        try {
            api.createNamespace(ns, null, null, null, null)
        } catch (e: ApiException) {
            fail("Unable to create namespace:\n${e.responseBody}")
        }

        val namespaces = api.listNamespace(null, true, null, null, null, 100, null, null, 10, false)
        val items = namespaces.items.mapNotNull { it.metadata?.name }

        assertEquals(5, items.size)
        items.assertHasElements("default", "kube-node-lease", "kube-public", "kube-system", "charted-system")
    }

    @Test
    fun `test pushing a helm chart`() {
        /* todo: this */
    }
}
