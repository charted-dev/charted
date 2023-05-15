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

package org.noelware.charted.modules.tracing.sentry

import io.sentry.Sentry
import org.noelware.charted.modules.tracing.Tracer
import org.noelware.charted.modules.tracing.Transaction

/**
 * Represents a [Tracer] for Sentry, which will output all tracing
 * metadata to a Sentry server configured by the DSN.
 */
object SentryTracer: Tracer {
    override fun createTransaction(name: String, operation: String?): Transaction = SentryTransaction(
        this,
        Sentry.startTransaction(name, operation ?: "(unknown)"),
    )

    override fun createTransaction(name: String): Transaction = createTransaction(name, null)

    override fun close() {
    }
}
