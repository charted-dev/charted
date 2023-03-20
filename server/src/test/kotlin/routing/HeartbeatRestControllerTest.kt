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

package org.noelware.charted.server.routing

import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Test
import org.noelware.charted.server.AbstractServerTest
import org.noelware.charted.server.createHttpClient
import kotlin.test.assertEquals

class HeartbeatRestControllerTest: AbstractServerTest() {
    private fun String.contentType(): ContentType = ContentType.parse(this)

    @DisplayName("request to GET /heartbeat controller")
    @Test
    fun test0(): Unit = withServer {
        val client = createHttpClient()
        val res1 = client.get("/heartbeat")
        assertEquals(HttpStatusCode.OK, res1.status)
        assertEquals("text/plain; charset=utf-8".contentType(), res1.contentType())

        val res = res1.bodyAsText()
        assertEquals("Ok.", res)
    }
}
