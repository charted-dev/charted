/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.testing.modules.webhooks.events

import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.junit.jupiter.api.Disabled
import org.junit.jupiter.api.Test
import org.noelware.charted.modules.webhooks.events.UpdatedMetadata
import kotlin.test.assertEquals

class UpdatedMetadataTests {
    @Test
    fun `can we serialize UpdatedMetadata objects`() {
        val updated = UpdatedMetadata(
            UpdatedMetadata.New(
                mapOf("myvalue" to "anewvalue"),
            ),
            UpdatedMetadata.Old(
                mapOf("myvalue" to "aoldvalue"),
            ),
        )

        val result = Json.encodeToString(updated)
        assertEquals("""{"new":{"myvalue":"anewvalue"},"old":{"myvalue":"aoldvalue"}}""", result)
    }

    @Test
    @Disabled("AnySerializer doesn't support deserialization at this time")
    fun `can we deserialize UpdatedMetadata objects`() {
        val result = """{"new":{"myvalue":"anewvalue"},"old":{"myvalue":"aoldvalue"}}"""
        val deserialized = Json.decodeFromString(UpdatedMetadata.serializer(), result)

        assertEquals(deserialized.new.data, mapOf("myvalue" to "anewvalue"))
        assertEquals(deserialized.old.data, mapOf("myvalue" to "aoldvalue"))
    }
}
