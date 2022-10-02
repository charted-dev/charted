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

import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import org.noelware.charted.workers.messaging.queue.BaseMessagingQueue
import java.lang.IllegalArgumentException
import java.lang.IllegalStateException

class BaseMessagingQueueTests {
    @Test
    fun `can we handle the triggers`() {
        assertDoesNotThrow {
            runBlocking {
                queue.handleTrigger("basic:owo")
            }
        }

        assertDoesNotThrow {
            runBlocking {
                queue.handleTrigger("basic:heck", "uwu da owo")
            }
        }

        val exception = assertThrows<IllegalArgumentException> {
            runBlocking { queue.handleTrigger("basic:heck", 1, 2, 3) }
        }

        assertEquals("Trigger heck (member class org.noelware.charted.workers.messaging.queue.tests.BaseWorkerQueue#handleString -> kotlin.Unit) had 1 arguments, you only added 3 arguments!", exception.message)

        val ex2 = assertThrows<IllegalStateException> {
            runBlocking { queue.handleTrigger("basic:heck", 1) }
        }

        assertEquals("Argument #0 [value -> kotlin.String] was invalid! You used kotlin.Int, not kotlin.String.", ex2.message)
    }

    companion object {
        internal val queue: BaseMessagingQueue = InMemoryMessagingQueue()

        @JvmStatic
        @BeforeAll
        fun start() {
            queue.registerQueue(BaseWorkerQueue())
        }
    }
}
