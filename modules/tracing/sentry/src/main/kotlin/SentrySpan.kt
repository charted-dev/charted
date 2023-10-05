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

import io.sentry.SpanStatus
import org.noelware.charted.modules.tracing.Span
import org.noelware.charted.modules.tracing.Transaction

class SentrySpan(private val parent: Transaction, private val inner: io.sentry.ISpan): Span {
    override fun transaction(): Transaction = parent
    override fun operation(): String? = inner.description
    override fun name(): String = inner.operation

    override fun end(throwable: Throwable?) {
        inner.finish(throwable?.let { SpanStatus.UNKNOWN_ERROR } ?: SpanStatus.OK)
    }
}