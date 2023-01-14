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

package org.noelware.charted.server.testing.plugins

import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Test
import org.noelware.charted.server.testing.AbstractChartedServerTest
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals

@Testcontainers(disabledWithoutDocker = true)
class SessionsPluginsTest: AbstractChartedServerTest() {
    @DisplayName("bail if no `Authorization` header")
    @Test
    fun test0(): Unit = withChartedServer {
        val res = client.get("/users/@me")
        assertEquals(res.status, HttpStatusCode.Forbidden)
        assertEquals("""{"success":false,"errors":[{"code":"MISSING_AUTHORIZATION_HEADER","message":"Rest handler requires an Authorization header to be present","detail":{"method":"GET","uri":"/users/@me"}}]}""", res.bodyAsText())
    }
}
