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

package org.noelware.charted.lib.tracing

/**
 * Represents the context of a trace. This includes all the spans.
 */
interface TraceContext {
    /**
     * Represents all the spans in a given [TraceContext].
     */
    val spans: List<ISpan>

    /**
     * Represents a list of attributes this [TraceContext] has.
     */
    val attributes: Map<String, String>

    /**
     * Represents the name of the trace context.
     */
    val name: String

    /**
     * Starts a span in this [TraceContext] object.
     * @param name The name of the span
     * @param operation The operation the span is doing.
     */
    fun startSpan(name: String, operation: String): ISpan

    /**
     * Removes a span from this [TraceContext].
     * @param name The name of this trace context
     */
    fun stopSpan(name: String): ISpan?

    /**
     * Starts a scoped [span][ISpan] with the [block] being the span that was spawned. Once
     * the block is finished executing, it will stop the span, and you will be returned
     * the span object that was once constructed.
     */
    fun withScope(name: String, operation: String, block: ISpan.() -> Unit): ISpan
}
