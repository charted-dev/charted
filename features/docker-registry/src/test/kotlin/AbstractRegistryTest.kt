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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.features.docker.registry.tests

import dev.floofy.utils.slf4j.logging
import org.noelware.charted.common.SetOnceGetValue
import org.slf4j.LoggerFactory
import org.testcontainers.containers.BindMode
import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.output.Slf4jLogConsumer
import org.testcontainers.containers.wait.strategy.HttpWaitStrategy
import org.testcontainers.utility.DockerImageName
import kotlin.test.AfterTest
import kotlin.test.BeforeTest

open class AbstractRegistryTest {
    private val _container: SetOnceGetValue<GenericContainer<*>> = SetOnceGetValue()
    private val log by logging<AbstractRegistryTest>()

    val container: GenericContainer<*>
        get() = _container.value

    @BeforeTest
    fun start() {
        if (_container.wasSet()) {
            return
        }

        log.info("Starting Docker registry container!")

        val image = DockerImageName.parse("registry:2.8.1")
        _container.value = GenericContainer(image)
            .withExposedPorts(5000)
            .withAccessToHost(true)
            .withClasspathResourceMapping(
                "/registry.yml",
                "/etc/docker/registry/config.yml",
                BindMode.READ_WRITE
            )
            .waitingFor(HttpWaitStrategy().forPort(5000))
            .withLogConsumer(Slf4jLogConsumer(LoggerFactory.getLogger("com.docker.registry")))

        _container.value.start()
    }

    @AfterTest
    fun destroy() {
        if (!_container.wasSet()) {
            throw IllegalStateException("Container has to be set before running #destroy()")
        }

        log.warn("Destroying container...")
        _container.value.stop()
    }
}
