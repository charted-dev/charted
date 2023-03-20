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
