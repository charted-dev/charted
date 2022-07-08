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
    "PRIVATE" to (1 shl 0).toLong(), // Organization is private and only the members who are in the organization has access to its repositories.
    "EXPERIMENTS" to (1 shl 1).toLong() // This organization has access to experimental features and can be enabled from the dashboard (/organizations/:name/settings#experimental)
)

class OrganizationFlags(originalBits: Long = 0): Bitfield(originalBits, FLAGS)
