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

package org.noelware.charted.server.tests.endpoints

import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.server.application.*
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Test
import org.noelware.charted.server.tests.AbstractServerTest
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class NormalEndpointTests: AbstractServerTest() {
    @DisplayName("Test [GET /] endpoint")
    @Test
    fun test0(): Unit = withChartedServer {
        val res = client.get("/")
        assertTrue(res.status.value == 200)

        val body = res.bodyAsText()
        assertEquals("""{"success":true,"data":{"message":"Hello, world! \uD83D\uDC4B","tagline":"You know, for Helm charts?","docs":"https://charts.noelware.org/docs"}}""", body)
    }

    @DisplayName("Test [GET /features] endpoint")
    @Test
    fun test1(): Unit = withChartedServer {
        val res = client.get("/features")
        assertTrue(res.status.value == 200)

        val body = res.bodyAsText()
        println(body)
    }
}
