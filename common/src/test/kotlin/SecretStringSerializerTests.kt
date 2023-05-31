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

package org.noelware.charted.common.tests

import kotlinx.serialization.*
import kotlinx.serialization.json.*
import org.noelware.charted.common.tests.annotations.SystemStubExtension
import org.assertj.core.api.Assertions.*
import org.junit.jupiter.api.Test
import org.noelware.charted.common.serializers.SecretStringSerializer
import uk.org.webcompere.systemstubs.environment.EnvironmentVariables

@Serializable
private data class Hello(
    @Serializable(with = SecretStringSerializer::class)
    val hello: String
)

@SystemStubExtension
class SecretStringSerializerTests {
    @Test
    fun `deserialize it correctly`() {
        // when encoding, we don't want to encode the actual value of "WORLD" (in this case,
        // it doesn't exist), so it'll return ${WORLD} instead of an actual value
        // of the "WORLD" environment variable.
        assertThat(Json.encodeToString(Hello("\${WORLD}")))
            .isEqualTo("{\"hello\":\"\${WORLD}\"}")

        // decoding test case 1. replace "${WORLD}" with "123"
        EnvironmentVariables(
            mapOf(
                "WORLD" to "123",
            ),
        ).execute {
            val text = "{\"hello\": \"\${WORLD}\"}"
            assertThat(Json.decodeFromString<Hello>(text)).isEqualTo(Hello("123"))
        }

        // decoding test case 2. return "234" similarly seen in Bash if we couldn't
        // find the NON_EXISTANT environment variables
        EnvironmentVariables(
            mapOf(
                "WORLD" to "123",
            ),
        ).execute {
            val text = "{\"hello\": \"\${NON_EXISTANT:-234}\"}"
            assertThat(Json.decodeFromString<Hello>(text)).isEqualTo(Hello("234"))
        }

        // decoding test case 3. return an empty string if we couldn't find
        // the environment variable
        EnvironmentVariables(
            mapOf(
                "WORLD" to "123",
            ),
        ).execute {
            val text = "{\"hello\": \"\${NON_EXISTANT}\"}"
            assertThat(Json.decodeFromString<Hello>(text)).isEqualTo(Hello(""))
        }
    }
}
