package org.noelware.charted.server.testing.plugins

import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Test
import org.noelware.charted.server.testing.AbstractChartedServerTest
import kotlin.test.assertEquals

class SessionsPluginsTest: AbstractChartedServerTest() {
    @DisplayName("bail if no `Authorization` header")
    @Test
    fun test0(): Unit = withChartedServer {
        val res = client.get("/users/@me")
        assertEquals(res.status, HttpStatusCode.Forbidden)
        assertEquals("""{"success":false,"errors":[{"code":"MISSING_AUTHORIZATION_HEADER","message":"Rest handler requires an Authorization header to be present","detail":{"method":"GET","uri":"/users/@me"}}]}""", res.bodyAsText())
    }
}
