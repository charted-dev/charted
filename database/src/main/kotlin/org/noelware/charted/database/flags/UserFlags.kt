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

import org.noelware.charted.common.Bitfield

private val FLAGS = mapOf(
    "premium" to (1 shl 0).toLong(),
    "employee" to (1 shl 1).toLong(),
    "verified:publisher" to (1 shl 2).toLong()
)

class UserFlags(originalBits: Long = 0L): Bitfield(originalBits, FLAGS)
