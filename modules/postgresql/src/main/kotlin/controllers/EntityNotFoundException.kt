/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.modules.postgresql.controllers

import org.jetbrains.exposed.sql.Column
import java.lang.RuntimeException
import kotlin.reflect.KProperty0

class EntityNotFoundException internal constructor(message: String): RuntimeException(message) {
    internal constructor(id: Long): this("Entity with ID [$id] was not found")
    internal constructor(expr: Pair<KProperty0<Column<Any>>, Any>): this("Entity with ${expr.first.get().name} [${expr.second}] was not found")
}