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
