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

package org.noelware.charted.configuration.kotlin.dsl

import org.junit.jupiter.api.Test
import org.noelware.charted.configuration.kotlin.dsl.enumSets.serialName
import org.noelware.charted.configuration.kotlin.dsl.features.Feature
import org.noelware.charted.configuration.kotlin.dsl.features.enumSet
import kotlin.test.*

class EnumSetTests {
    @Test
    fun `test enumsets`() {
        assertTrue(Feature.enumSet.isWildcard(listOf(Feature.Wildcard, Feature.DockerRegistry)))

        assertTrue(Feature.enumSet.enabled(listOf(Feature.DockerRegistry), Feature.DockerRegistry))
        assertTrue(Feature.enumSet.enabled(listOf(Feature.DockerRegistry), "docker_registry"))
        assertTrue(Feature.enumSet.enabled(listOf(Feature.Wildcard), "woof"))
        assertFalse(Feature.enumSet.enabled(listOf(), Feature.DockerRegistry))
    }

    @Test
    fun `test serialName extension`() {
        assertEquals("docker_registry", Feature.DockerRegistry.serialName)
        assertNotEquals("*", Feature.DockerRegistry.serialName)

        assertNull(Woof.Wildcard.serialName)
    }

    private enum class Woof {
        Wildcard
    }
}
