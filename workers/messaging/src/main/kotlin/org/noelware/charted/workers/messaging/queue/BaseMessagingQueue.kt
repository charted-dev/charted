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

package org.noelware.charted.workers.messaging.queue

import java.io.Closeable

/**
 * Represents a base messaging queue interface. This interface contains basic lifecycle events
 * if the messaging queue is external.
 *
 * The queue is also responsible for handling worker triggers. [#handleTrigger(String, Any...)][handleTrigger] will
 * handle the actual trigger.
 */
interface BaseMessagingQueue: Closeable {
    /**
     * Represents a list of queues in this [BaseMessagingQueue].
     */
    val queues: Map<String, WorkerQueue>

    /**
     * Checks if the messaging queue is closed. This usually true
     * if the server is shutting down.
     */
    val closed: Boolean

    /**
     * The messaging queue name. Internally, the full name when inserted into the
     * queue itself will be `[name]:<trigger name>`.
     */
    val name: String

    /**
     * Handles a worker trigger to handle processes in a different thread executor.
     * @param name The trigger name
     * @param args The arguments to invoke the trigger itself.
     */
    suspend fun handleTrigger(name: String, vararg args: Any)

    /**
     * Registers a worker queue in this [base queue][BaseMessagingQueue].
     */
    fun <T: WorkerQueue> registerQueue(queue: T)

    /**
     * Basic lifecycle method to connect to an external messaging queue. By default,
     * this will be a no-operation if not implemented. If so, this method should only
     * be called once and never triggered again.
     */
    fun connect() {
        /* no-op */
    }
}
