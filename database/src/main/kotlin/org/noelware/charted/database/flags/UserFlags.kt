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

package org.noelware.charted.database.flags

import org.noelware.charted.util.Bitfield

private val FLAGS_MAP = mapOf(
    // If the user is an administrator of this instance.
    "ADMIN" to (1L shl 0),

    // If the user is a Noelware employee, cannot be applied outside
    // of charts.noelware.org.
    "EMPLOYEE" to (1L shl 1),

    // If they have a premium subscription for extra benefits.
    "PREMIUM" to (1L shl 2)
)

class UserFlags(originalBits: Long = 0): Bitfield(FLAGS_MAP, originalBits)

fun Long.toUserFlags(): UserFlags = UserFlags(this)
