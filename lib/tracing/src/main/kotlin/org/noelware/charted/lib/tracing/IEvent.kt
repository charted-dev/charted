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

import org.noelware.charted.lib.tracing.events.EventType

/**
 * Represents an event that occurred in a [span][ISpan].
 */
interface IEvent {
    /**
     * The description of the event.
     */
    val description: String

    /**
     * Metadata about the event.
     */
    val metadata: Map<String, Any>

    /**
     * The [EventType] that this event represents. This can change overtime.
     */
    var type: EventType

    /**
     * Sets a metadata attribute to this span event.
     * @param key The metadata key
     * @param value The metadata value, which can be represented as [Any].
     */
    operator fun <T: Any> set(key: String, value: T)

    /**
     * Fetches a metadata attribute of a span event. Might be `null` if the
     * metadata attribute wasn't set.
     *
     * @param key The metadata key
     * @return The metadata attribute, if any, otherwise `null`.
     */
    operator fun <T: Any> get(key: String): T?
}
