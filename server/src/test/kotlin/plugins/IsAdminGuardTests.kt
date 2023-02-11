/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.server.testing.plugins

import dev.floofy.utils.koin.retrieve
import io.ktor.client.call.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import io.ktor.util.*
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Test
import org.koin.core.context.GlobalContext
import org.noelware.charted.server.testing.AbstractChartedServerTest
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals

@Testcontainers(disabledWithoutDocker = true)
class IsAdminGuardTests: AbstractChartedServerTest() {
    @DisplayName("bail if we are not admin")
    @Test
    fun test0(): Unit = withChartedServer {
        generateFakeUser(password = "noeliscutieuwu")
        val client = createClient {
            install(ContentNegotiation) {
                json(GlobalContext.retrieve())
            }
        }

        val res1 = client.get("/admin") {
            header("Authorization", "Basic ${"noel:noeliscutieuwu".encodeBase64()}")
        }

        assertEquals(res1.status, HttpStatusCode.Unauthorized)
        assertEquals(
            """
            {"success":false,"errors":[{"code":"NOT_AN_ADMIN","message":"You must have administrator privileges to access this route"}]}
            """.trimIndent(),
            res1.bodyAsText(),
        )
    }
}
