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

package org.noelware.charted.workers.messaging.queue.tests

import org.noelware.charted.workers.messaging.queue.BaseMessagingQueue
import org.noelware.charted.workers.messaging.queue.WorkerQueue
import org.noelware.charted.workers.messaging.queue.annotations.WorkerTrigger
import kotlin.reflect.full.callSuspend
import kotlin.reflect.full.createType
import kotlin.reflect.full.findAnnotation
import kotlin.reflect.full.functions

class InMemoryMessagingQueue: BaseMessagingQueue {
    private val _queues: MutableMap<String, WorkerQueue> = mutableMapOf()
    override val queues: Map<String, WorkerQueue> = _queues

    /**
     * Checks if the messaging queue is closed. This usually true
     * if the server is shutting down.
     */
    override val closed: Boolean = false

    /**
     * The messaging queue name. Internally, the full name when inserted into the
     * queue itself will be `[name]:<trigger name>`.
     */
    override val name: String = "inmemory"

    /**
     * Handles a worker trigger to handle processes in a different thread executor.
     * @param name The trigger name
     * @param args The arguments to invoke the trigger itself.
     */
    override suspend fun handleTrigger(name: String, vararg args: Any) {
        if (name.isEmpty()) {
            throw IllegalStateException("Trigger name can't be ")
        }

        val queueName = name.split(':', limit = 2)
        if (queueName.size < 2) {
            throw IllegalStateException("Queue name must be `worker:<trigger name>`")
        }

        if (!_queues.any { it.key == queueName.first() }) {
            throw IllegalStateException("Unable to find worker queue by name ${queueName.first()}!")
        }

        val queueToHandle = _queues[queueName.first()]!!
        val triggers = queueToHandle::class.functions.filter { fn ->
            val anno = fn.findAnnotation<WorkerTrigger>() ?: return@filter false
            anno.name == queueName.last()
        }

        if (triggers.isEmpty()) {
            throw IllegalArgumentException("There was less than no triggers present with name $name (found 0 other triggers)")
        }

        if (triggers.size > 1) {
            throw IllegalArgumentException("There was more than one trigger present with name $name (found ${triggers.size} other triggers)")
        }

        val trigger = triggers.first()

        // the first argument will always be the class instance, so let's just ignore it
        // and only go off by the rest
        val triggerArgs = trigger.parameters.filterIndexed { i, _ -> i != 0 }
        if (triggerArgs.size != args.size) {
            throw IllegalArgumentException("Trigger ${queueName.last()} (member ${queueToHandle::class}#${trigger.name} -> ${trigger.returnType}) had ${triggerArgs.size} arguments, you only added ${args.size} arguments!")
        }

        // validate the arguments
        for ((index, arg) in triggerArgs.withIndex()) {
            val type = args[index]::class.createType()
            if (arg.type != type) {
                throw IllegalStateException("Argument #$index [${arg.name} -> ${arg.type}] was invalid! You used $type, not ${arg.type}.")
            }
        }

        if (trigger.isSuspend) {
            trigger.callSuspend(queueToHandle, *args)
        } else {
            trigger.call(queueToHandle, *args)
        }
    }

    override fun <T: WorkerQueue> registerQueue(queue: T) {
        _queues[queue.name] = queue
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * As noted in [AutoCloseable.close], cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * *mark* the `Closeable` as closed, prior to throwing
     * the `IOException`.
     *
     * @throws java.io.IOException if an I/O error occurs
     */
    override fun close() {
        _queues.clear()
    }
}
