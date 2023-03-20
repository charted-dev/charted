package org.noelware.charted.server.routing

import io.ktor.client.call.*
import io.ktor.client.request.*
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.api.Test
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.server.AbstractServerTest
import org.noelware.charted.server.assertSuccessfulResponse
import org.noelware.charted.server.createHttpClient
import org.noelware.charted.server.routing.v1.FeaturesResponse
import kotlin.test.assertNotNull

class FeaturesRestControllerTest: AbstractServerTest() {
    @DisplayName("request to GET /features controller")
    @Test
    fun test0(): Unit = withServer {
        val client = createHttpClient()
        val res1: ApiResponse<FeaturesResponse> = client.get("/features").body()
        assertSuccessfulResponse(res1)

        val res = res1 as ApiResponse.Ok<FeaturesResponse>
        assertNotNull(res.data)
    }
}
