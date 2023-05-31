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

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encodeToString
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import org.noelware.charted.common.types.responses.ApiResponse

private object UnitSerializer: KSerializer<Unit> {
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor("charted.Unit")
    override fun deserialize(decoder: Decoder) {
        throw SerializationException("Can't deserialize this object.")
    }

    override fun serialize(encoder: Encoder, value: Unit) {
        throw SerializationException("Can't serialize this object.")
    }
}

class ApiResponseTests {
    @Test
    fun `test if it can serialize ApiResponse elements`() {
        val res = ApiResponse.ok("heck uwu!!!")
        assertEquals("""{"success":true,"data":"heck uwu!!!"}""", Json.encodeToString(res))

        val res2 = ApiResponse.err(SerializationException("oh no!!!"))
        assertEquals("""{"success":false,"errors":[{"code":"INTERNAL_SERVER_ERROR","message":"oh no!!!"}]}""", Json.encodeToString(ApiResponse.serializer(UnitSerializer), res2))

        val res3 = ApiResponse.err(
            "puby gang", "you are in puby gang, not polar gang :(",
            buildJsonObject {
                put("woof", true)
            },
        )

        assertEquals("""{"success":false,"errors":[{"code":"puby gang","message":"you are in puby gang, not polar gang :(","detail":{"woof":true}}]}""", Json.encodeToString(ApiResponse.serializer(UnitSerializer), res3))
    }

    @Test
    fun `test if it can deserialize ApiResponse elements`() {
        assertDoesNotThrow {
            val result = Json.decodeFromString<ApiResponse<String>>("""{"success":true,"data":"a string"}""")
            assertTrue(result.success)
            assertEquals("a string", (result as ApiResponse.Ok<String>).data)
        }

        assertDoesNotThrow {
            val result = Json.decodeFromString<ApiResponse<Unit>>("""{"success":false,"errors":[{"code":"heck","message":"heck"}]}""")
            assertFalse(result.success)
            assertEquals(1, (result as ApiResponse.Err).errors.size)
        }
    }
}
