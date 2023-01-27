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
import org.junit.jupiter.api.assertDoesNotThrow
import org.koin.core.context.GlobalContext
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.server.testing.AbstractChartedServerTest
import org.noelware.charted.types.responses.ApiResponse
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

@Testcontainers(disabledWithoutDocker = true)
class SessionsPluginsTest: AbstractChartedServerTest() {
    @DisplayName("bail if no `Authorization` header")
    @Test
    fun test0(): Unit = withChartedServer {
        val res = client.get("/users/@me")
        assertEquals(res.status, HttpStatusCode.Forbidden)
        assertEquals(
            """
            {"success":false,"errors":[{"code":"MISSING_AUTHORIZATION_HEADER","message":"Rest handler requires an Authorization header to be present","detail":{"method":"GET","uri":"/users/@me"}}]}
            """.trimIndent(),
            res.bodyAsText(),
        )
    }

    @DisplayName("bail if invalid auth header")
    @Test
    fun test1(): Unit = withChartedServer {
        val res = client.get("/users/@me") {
            header("Authorization", "aaaaaa")
        }

        assertEquals(res.status, HttpStatusCode.NotAcceptable)
        assertEquals(
            """
            {"success":false,"errors":[{"code":"INVALID_AUTHORIZATION_HEADER","message":"Request authorization header given didn't follow '[Bearer|ApiKey|Basic] [Token]'","detail":{"method":"GET","uri":"/users/@me"}}]}
            """.trimIndent(),
            res.bodyAsText(),
        )
    }

    @DisplayName("bail if the auth header value is invalid")
    @Test
    fun test2(): Unit = withChartedServer {
        val res = client.get("/users/@me") {
            header("Authorization", "Wuff wuff!")
        }

        assertEquals(res.status, HttpStatusCode.NotAcceptable)
        assertEquals(
            """
            {"success":false,"errors":[{"code":"INVALID_AUTHORIZATION_HEADER_PREFIX","message":"The given authorization prefix [Wuff] was not 'Bearer', 'ApiKey', or 'Basic'"}]}
            """.trimIndent(),
            res.bodyAsText(),
        )
    }

    @DisplayName("[basic credentials] bail if invalid user credentials")
    @Test
    fun test3(): Unit = withChartedServer {
        // 1. Check if it is invalid value
        val res1 = client.get("/users/@me") {
            header("Authorization", "Basic ${"woofwoofuwu".encodeBase64()}")
        }

        assertEquals(res1.status, HttpStatusCode.NotAcceptable)
        assertEquals(
            """
            {"success":false,"errors":[{"code":"INVALID_BASIC_AUTH_CREDENTIALS","message":"Basic authentication needs to be 'username:password'","detail":{"method":"GET","uri":"/users/@me"}}]}
            """.trimIndent(),
            res1.bodyAsText(),
        )

        // 2. Check if the user is not in the database
        val res2 = client.get("/users/@me") {
            header("Authorization", "Basic ${"someuser:aninvalidpassword".encodeBase64()}")
        }

        assertEquals(res2.status, HttpStatusCode.NotFound)
        assertEquals(
            """
            {"success":false,"errors":[{"code":"UNKNOWN_USER","message":"User with username [someuser] doesn't exist","detail":{"method":"GET","uri":"/users/@me"}}]}
            """.trimIndent(),
            res2.bodyAsText(),
        )
    }

    @DisplayName("generate fake user, bail if invalid password, accept if valid password")
    @Test
    fun test4(): Unit = withChartedServer {
        val client = createClient {
            install(ContentNegotiation) {
                json(GlobalContext.retrieve())
            }
        }

        val user = generateFakeUser(password = "noeliscutieuwu")
        val res1 = client.get("/users/@me") {
            header("Authorization", "Basic ${"noel:aninvalidpassword".encodeBase64()}")
        }

        assertEquals(res1.status, HttpStatusCode.Unauthorized)
        assertEquals(
            """
            {"success":false,"errors":[{"code":"INVALID_PASSWORD","message":"Invalid password!","detail":{"method":"GET","uri":"/users/@me"}}]}
            """.trimIndent(),
            res1.bodyAsText(),
        )

        val res2 = client.get("/users/@me") {
            header("Authorization", "Basic ${"${user.username}:noeliscutieuwu".encodeBase64()}")
        }

        assertEquals(res2.status, HttpStatusCode.OK)

        // If it couldn't be deserialized, then this will throw
        assertDoesNotThrow {
            val userPayload: ApiResponse<User> = res2.body()
            assertTrue(userPayload is ApiResponse.Ok)

            val userReturned = userPayload.data
            assertNotNull(userReturned)
            assertEquals(user.username, userReturned.username)
        }
    }
}
