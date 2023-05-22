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

package org.noelware.charted;

import io.ktor.http.HttpStatusCode;
import java.util.List;
import java.util.Objects;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.common.types.responses.ApiError;

/**
 * {@link RuntimeException} to throw to send back to Ktor when a request from
 * any subproject that can't rely on {@link io.ktor.server.application.ApplicationCall}
 * directly.
 *
 * @author Noel Towa (cutie@floofy.dev)
 * @since 19.05.23
 */
public class KtorHttpRespondException extends RuntimeException {
    private final transient HttpStatusCode statusCode;
    private final List<ApiError> errors;

    /**
     * Constructs a new {@link KtorHttpRespondException}.
     * @param statusCode The status code to use.
     * @param error A single error to use when throwing back to the user.
     * @param cause Cause itself, can be null.
     */
    public KtorHttpRespondException(HttpStatusCode statusCode, ApiError error, Throwable cause) {
        this(statusCode, List.of(error), cause);
    }

    /**
     * Constructs a new {@link KtorHttpRespondException}.
     * @param statusCode The status code to use.
     * @param error A single error to use when throwing back to the user.
     */
    public KtorHttpRespondException(HttpStatusCode statusCode, ApiError error) {
        this(statusCode, List.of(error), null);
    }

    /**
     * Constructs a new {@link KtorHttpRespondException}.
     * @param statusCode The status code to use.
     * @param errors A list of errors to send back to the user. This should include multiple errors.
     */
    public KtorHttpRespondException(HttpStatusCode statusCode, List<ApiError> errors) {
        this(statusCode, errors, null);
    }

    /**
     * Constructs a new {@link KtorHttpRespondException}.
     * @param statusCode The status code to use.
     * @param errors A list of errors to send back to the user. This should include multiple errors.
     * @param cause Cause itself, can be null.
     */
    public KtorHttpRespondException(HttpStatusCode statusCode, List<ApiError> errors, Throwable cause) {
        super(cause);

        this.statusCode = Objects.requireNonNull(statusCode, "Status Code cannot be null");
        this.errors = Objects.requireNonNull(errors, "Errors cannot be null");
    }

    /**
     * @return HTTP status code to use for this {@link KtorHttpRespondException}.
     */
    @NotNull
    public HttpStatusCode httpStatusCode() {
        return statusCode;
    }

    /**
     * @return list of errors to send back to the user.
     */
    @NotNull
    public List<ApiError> errors() {
        return errors;
    }
}
