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

package org.noelware.charted.tests.modules.redis

import io.kubernetes.client.openapi.apis.CoreV1Api
import io.kubernetes.client.openapi.models.V1Namespace
import io.kubernetes.client.openapi.models.V1ObjectMeta
import io.kubernetes.client.util.ClientBuilder
import io.kubernetes.client.util.KubeConfig
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import org.junit.jupiter.api.*
import org.junit.jupiter.api.condition.DisabledOnOs
import org.junit.jupiter.api.condition.EnabledOnOs
import org.junit.jupiter.api.condition.OS
import org.noelware.charted.configuration.kotlin.dsl.RedisConfig
import org.noelware.charted.modules.redis.DefaultRedisClient
import org.noelware.charted.modules.redis.RedisClient
import org.slf4j.LoggerFactory
import org.testcontainers.containers.DockerComposeContainer
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.Network
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.containers.startupcheck.OneShotStartupCheckStrategy
import org.testcontainers.containers.wait.strategy.Wait
import org.testcontainers.images.builder.Transferable
import org.testcontainers.junit.jupiter.Testcontainers
import org.testcontainers.k3s.K3sContainer
import org.testcontainers.utility.DockerImageName
import java.io.File
import java.io.StringReader
import kotlin.io.path.createTempDirectory
import kotlin.test.assertEquals
import kotlin.time.Duration.Companion.seconds
import kotlin.time.toJavaDuration

@Testcontainers(disabledWithoutDocker = true)
@DisabledOnOs(architectures = ["aarch64", "arm64"], disabledReason = "docker/compose Docker image doesn't support ARM and rancher/k3s requires the image to be privileged")
class RedisSentinelConnectionTests {
    @Test
    @Disabled // at the moment, this doesn't work :<
    fun `can we connect to sentinel (kubernetes with bitnami helm chart)`(): Unit = runBlocking {
        val network = Network.newNetwork()

        // First, we need to start the Kubernetes cluster. For now, it will be Rancher's K3s project.
        // The test container also only supports Kubernetes <=1.23, anything else just fails.
        //
        // The network is also allowed for `kubectl`/`helm` usages with the cluster itself.
        val k3sContainer = K3sContainer(DockerImageName.parse("rancher/k3s:v1.23.14-k3s1"))
            .withNetwork(network)
            .withNetworkAliases("k3s")

        k3sContainer.start()

        // Create a Kubernetes client, so we can create the namespace and do other
        // fancy magic.
        val client = ClientBuilder.kubeconfig(KubeConfig.loadKubeConfig(StringReader(k3sContainer.kubeConfigYaml))).build()
        val coreApi = CoreV1Api(client)
        val nodes = coreApi.listNode(
            null,
            false,
            null,
            null,
            null,
            100,
            null,
            null,
            30,
            false
        )

        assertEquals(1, nodes.items.size)
        val ns = coreApi.createNamespace(
            V1Namespace().apply {
                metadata(
                    V1ObjectMeta().apply {
                        name("redis-system")
                    }
                )
            },
            null, null, null, null
        )

        assertEquals("redis-system", ns.metadata!!.name)

        // Create temporary directory, so we can have ~/.helm to our disposable
        val helmDir = createTempDirectory("helm")
        val cacheDir = createTempDirectory("helm-cache")

        // Install the Bitnami Redis chart on the redis-system namespace
        val helmRepoAdd = GenericContainer(DockerImageName.parse("alpine/helm:3.10.2"))
            .withNetwork(network)
            .withCommand("repo add bitnami https://charts.bitnami.com/bitnami")
            .withStartupCheckStrategy(OneShotStartupCheckStrategy().withTimeout(30.seconds.toJavaDuration()))
            .withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("sh.helm.docker.repo.add")))
            .withFileSystemBind(
                withContext(Dispatchers.IO) {
                    helmDir.toRealPath()
                }.toString(),
                "/root/.config/helm"
            )
            .withFileSystemBind(
                withContext(Dispatchers.IO) {
                    cacheDir.toRealPath()
                }.toString(),
                "/root/.cache/helm"
            )

        helmRepoAdd.start()

        val helmInstall = GenericContainer(DockerImageName.parse("alpine/helm:3.10.2"))
            .withNetwork(network)
            .withCommand("install redis-ha bitnami/redis --set sentinel.enabled=true --namespace redis-system")
            .withStartupCheckStrategy(OneShotStartupCheckStrategy().withTimeout(30.seconds.toJavaDuration()))
            .withCopyToContainer(Transferable.of(k3sContainer.kubeConfigYaml), "/root/.kube/config")
            .withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("sh.helm.docker.install")))
            .withFileSystemBind(
                withContext(Dispatchers.IO) {
                    helmDir.toRealPath()
                }.toString(),
                "/root/.config/helm"
            )
            .withFileSystemBind(
                withContext(Dispatchers.IO) {
                    cacheDir.toRealPath()
                }.toString(),
                "/root/.cache/helm"
            )

        helmInstall.start()

        // Check if we have 3 pods running in the `redis-system` namespace
        // val pods = coreApi.listNamespacedPod("redis-system", null, false, null, null, null, 100, null, null, 30, null)

        k3sContainer.close()
    }

    @Test
    @EnabledOnOs(OS.LINUX) // macOS and windows runners (on GitHub) don't have good Docker support
    fun `can we connect to sentinel (docker compose, auth)`(): Unit = runBlocking {
        val container: DockerComposeContainer<*> = DockerComposeContainer(File("src/test/resources/docker-compose.auth.yml")).apply {
            withExposedService("redis_1", 6379)
            withExposedService("redis-sentinel_1", 26379)

            withLogConsumer("redis-sentinel_1", Slf4jLogConsumer(LoggerFactory.getLogger("com.redis.sentinel")))
            withLogConsumer("redis_1", Slf4jLogConsumer(LoggerFactory.getLogger("com.redis.master")))

            waitingFor("redis-sentinel_1", Wait.forListeningPort())
        }

        container.start()

        val config = RedisConfig {
            val host = container.getServiceHost("redis-sentinel_1", 26379)
            val port = container.getServicePort("redis-sentinel_1", 26379)

            addSentinel("redis://:ahaanotherpasswordtolookup@$host:$port/6")
            masterName = "mymaster"
            password = "somesickpasswordithink"
        }

        val connection: RedisClient = DefaultRedisClient(config)
        assertDoesNotThrow { connection.connect() }

        withContext(Dispatchers.IO) {
            connection.close()
        }

        container.close()
    }

    @Test
    @EnabledOnOs(OS.LINUX) // macOS and windows runners (on GitHub) don't have good Docker support
    fun `can we connect to sentinel (docker compose, no auth)`(): Unit = runBlocking {
        val container: DockerComposeContainer<*> = DockerComposeContainer(File("src/test/resources/docker-compose.yml")).apply {
            withExposedService("redis_1", 6379)
            withExposedService("redis-sentinel_1", 26379)

            withLogConsumer("redis-sentinel_1", Slf4jLogConsumer(LoggerFactory.getLogger("com.redis.sentinel")))
            withLogConsumer("redis_1", Slf4jLogConsumer(LoggerFactory.getLogger("com.redis.master")))

            waitingFor("redis-sentinel_1", Wait.forListeningPort())
        }

        container.start()

        val config = RedisConfig {
            val host = container.getServiceHost("redis-sentinel_1", 26379)
            val port = container.getServicePort("redis-sentinel_1", 26379)

            addSentinel(host, port)
            masterName = "mymaster"
        }

        val connection: RedisClient = DefaultRedisClient(config)
        assertDoesNotThrow { connection.connect() }
        withContext(Dispatchers.IO) {
            connection.close()
        }

        container.close()
    }
}
