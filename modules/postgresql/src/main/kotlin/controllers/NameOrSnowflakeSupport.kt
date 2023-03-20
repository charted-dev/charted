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

import org.jetbrains.exposed.dao.LongEntity
import org.jetbrains.exposed.sql.Column
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import kotlin.reflect.KProperty0

suspend fun <T, Entity: LongEntity, Created: Any, Patched: Any> AbstractDatabaseController<T, Entity, Created, Patched>.getByIdOrNameOrNull(
    idOrName: String,
    column: KProperty0<Column<String>>
): T? = when {
    idOrName.toLongOrNull() != null -> getOrNull(idOrName.toLong())
    idOrName.matchesNameAndIdRegex() -> getOrNull(column to idOrName)
    else -> null
}

suspend fun <T, Entity: LongEntity, Created: Any, Patched: Any> AbstractDatabaseController<T, Entity, Created, Patched>.getEntityByIdOrNameOrNull(
    idOrName: String,
    column: KProperty0<Column<String>>
): Entity? = when {
    idOrName.toLongOrNull() != null -> getEntityOrNull(idOrName.toLong())
    idOrName.matchesNameAndIdRegex() -> getEntityOrNull(column to idOrName)
    else -> null
}
