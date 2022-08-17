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
 * Represents a trace span, which can include external metadata about a trace.
 */
interface ISpan {
    /** The operation the span is doing. */
    val operation: String

    /** All the span's events that occurred. */
    val events: List<IEvent>

    /** The name of the span */
    val name: String

    /**
     * Starts a span event.
     * @param description The description of the span event.
     * @param metadata Any metadata to append to the span event.
     * @return a [span event][IEvent] spawned.
     */
    fun startEvent(description: String, metadata: Map<String, Any>): IEvent
}
