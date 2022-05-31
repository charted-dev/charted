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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.common

import kotlinx.datetime.Clock
import java.util.concurrent.atomic.AtomicLong

private const val PROCESS_ID = 1L
private const val WORKER_ID = 0L // TODO: make this modifiable per cluster node

@kotlinx.serialization.Serializable
data class DeconstructedSnowflake(
    val id: Long,
    val timestamp: Long,
    val workerId: Long,
    val processId: Long,
    val increment: Long,
    val epoch: Long = Snowflake.EPOCH
)

/**
 * A class for generating and destructuring Twitter snowflakes.
 *
 * A {@link https://developer.twitter.com/en/docs/twitter-ids Twitter snowflake}
 * is a 64-bit unsigned integer with 4 fields that have a fixed epoch value.
 *
 * If we have a snowflake `266241948824764416` we can represent it as binary:
 * ```
 * 64                                          22     17     12          0
 *  000000111011000111100001101001000101000000  00001  00000  000000000000
 *           number of ms since epoch           worker  pid    increment
 * ```
 */
class Snowflake {
    companion object {
        private val increment = AtomicLong(0)
        val EPOCH = 1651276800000

        fun generate(): Long {
            val timestamp = Clock.System.now().toEpochMilliseconds()
            val newIncrement = increment.getAndIncrement()
            if (newIncrement >= 4095)
                increment.set(0)

            return ((timestamp - EPOCH) shl 22) or ((WORKER_ID and 0b11111) shl 17) or ((PROCESS_ID and 0b11111) shl 12) or newIncrement
        }

        fun decrypt(id: Long): DeconstructedSnowflake = DeconstructedSnowflake(
            id,
            (id shr 22) + EPOCH,
            (id shr 17) and 0b11111,
            (id shr 12) and 0b11111,
            id and 0b111111111111
        )
    }
}
