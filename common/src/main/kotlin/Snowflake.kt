/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted

import kotlinx.atomicfu.atomic
import kotlinx.datetime.Clock

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
@Deprecated("This will be replaced with org.noelware.charted.snowflake.Snowflake in v0.3.3-nightly")
public class Snowflake {
    /**
     * Represents a deconstructed [Snowflake]
     */
    @kotlinx.serialization.Serializable
    public data class Deconstructed(
        val id: Long,
        val epoch: Long = EPOCH,
        val workerID: Long,
        val timestamp: Long,
        val processID: Long,
        val increment: Long
    )

    public companion object {
        private val increment = atomic(0L)
        public const val EPOCH: Long = 1651276800000

        public fun generate(): Long {
            val timestamp = Clock.System.now().toEpochMilliseconds()
            val inc = increment.getAndIncrement()
            if (inc >= 4095) {
                increment.value = 0L
            }

            return ((timestamp - EPOCH) shl 22) or ((0 and 0b11111) shl 17) or ((1 and 0b11111) shl 12) or inc
        }

        public fun decrypt(id: Long): Deconstructed = Deconstructed(
            id,
            EPOCH,
            (id shr 17) and 0b11111,
            (id shr 22) + EPOCH,
            (id shr 12) and 0b11111,
            id and 0b111111111111
        )
    }
}
