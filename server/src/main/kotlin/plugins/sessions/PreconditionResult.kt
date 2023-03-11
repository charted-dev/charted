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

package org.noelware.charted.server.plugins.sessions

import io.ktor.http.*
import org.noelware.charted.common.types.responses.ApiError

/**
 * Represents a result of a session precondition.
 */
sealed class PreconditionResult {
    /**
     * Represents a successful precondition.
     */
    object Success: PreconditionResult()

    /**
     * Represents a failed precondition.
     * @param status HTTP status code to send, defaults to [HttpStatusCode.PreconditionFailed]
     * @param errors List of [API errors][ApiError] to send to the user.
     */
    class Failed(
        val status: HttpStatusCode = HttpStatusCode.PreconditionFailed,
        val errors: List<ApiError> = listOf()
    ): PreconditionResult() {
        /**
         * Secondary constructor for the failed precondition.
         * @param status HTTP status code to send, defaults to [HttpStatusCode.PreconditionFailed]
         * @param error single [API error][ApiError] to send back to the user
         */
        constructor(status: HttpStatusCode = HttpStatusCode.PreconditionFailed, error: ApiError): this(status, listOf(error))
    }
}

/**
 * Checks if this [PreconditionResult] is the [PreconditionResult.Failed] stage.
 */
fun PreconditionResult.isFailure(): Boolean = this is PreconditionResult.Failed
