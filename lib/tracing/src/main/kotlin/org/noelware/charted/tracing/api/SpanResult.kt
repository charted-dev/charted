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

package org.noelware.charted.tracing.api

/**
 * Represents the result of a [Span]. Each result has a key to represent
 * each other.
 */
sealed class SpanResult(val key: String) {
    /**
     * Represents that the span was successful and didn't throw any errors.
     */
    object Ok: SpanResult("OK")

    /**
     * Default outcome of the span. This is usually the result if [Span.finish] wasn't called.
     */
    object Unknown: SpanResult("Unknown")

    /**
     * Represents that an exception was thrown during the span's execution.
     * @param exception The exception that the span threw.
     */
    class Error<T: Throwable>(val exception: T): SpanResult("Error")
}
