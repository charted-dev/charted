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

package org.noelware.charted.lib.tracing.events

/**
 * Represents the event type of [span event][org.noelware.charted.lib.tracing.IEvent].
 */
sealed class EventType(val id: Int) {
    /**
     * Represents an unknown event type.
     */
    object Unknown: EventType(-1)

    /**
     * The default type for a span event, nothing hasn't really happened.
     */
    object Empty: EventType(0)

    /**
     * Represents a normal successful event. This might have some metadata attached
     * to it about the context of a span event.
     */
    object Success: EventType(1)

    /**
     * Represents an abnormal thrown exception that might've occurred.
     */
    object Exception: EventType(2)

    companion object {
        @JvmField
        val UnknownEventType = Unknown

        @JvmField
        val EmptyEventType = Empty

        @JvmField
        val SuccessEventType = Success

        @JvmField
        val ExceptionEventType = Exception
    }
}
