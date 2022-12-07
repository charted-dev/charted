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

package org.noelware.charted.server.tests.plugins

import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.server.application.*
import org.junit.jupiter.api.Test
import org.noelware.charted.server.endpoints.v1.api.AdminEndpoint
import org.noelware.charted.server.tests.AbstractServerTest
import org.noelware.ktor.NoelKtorRouting
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class IsAdminGuardTests: AbstractServerTest() {
    @Test
    fun `bail out with no authentication and non-acceptable auth`() = withChartedServer({
        install(NoelKtorRouting) {
            endpoints(AdminEndpoint())
        }
    }) {
        // The session middleware runs first, and we don't have an [Authorization] header,
        // so the server bails out with a 403 Forbidden
        val res = client.get("/admin")
        assertTrue(res.status.value == 403)

        // Now, let's try sending a token that is not a valid one. First, we will
        // try to do "Owo <someblahtoken>"
        val res2 = client.get("/admin") {
            header("Authorization", "Owo someblahtoken")
        }

        assertTrue(res2.status.value == 400)

        val body: String = res2.body()
        assertEquals(body, """{"success":false,"errors":[{"code":"UNKNOWN_AUTH_STRATEGY","message":"The prefix specified [Owo] was not 'Bearer' or 'ApiKey'","detail":{"method":"GET","uri":"/admin"}}]}""")
    }
}
