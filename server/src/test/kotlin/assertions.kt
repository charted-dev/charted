package org.noelware.charted.server

import org.noelware.charted.common.types.responses.ApiError
import org.noelware.charted.common.types.responses.ApiResponse
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

fun <T> assertSuccessfulResponse(response: ApiResponse<T>) {
    assertTrue(response.success, "Expected successful response")
    assertTrue(response is ApiResponse.Ok<T>, "Expected API response to be [ApiResponse.Ok]")
}

fun <T> assertFailedResponse(response: ApiResponse<T>) {
    assertFalse(response.success, "Expected fail response")
    assertTrue(response is ApiResponse.Err, "Expected API response to be [ApiResponse.Err]")
}

fun assertEqualApiError(`actual`: ApiError, expected: ApiError) {
    assertEquals(`actual`.message, expected.message)
    assertEquals(`actual`.code, expected.code)
}
