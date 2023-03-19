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

@file:JvmName("DatabaseControllerExtensionsKt")

package org.noelware.charted.modules.postgresql.controllers

import org.jetbrains.exposed.dao.LongEntity
import org.jetbrains.exposed.dao.id.EntityID
import org.jetbrains.exposed.sql.Column
import org.noelware.charted.modules.postgresql.SnowflakeTable
import kotlin.reflect.KProperty0

/**
 * Retrieve an entity from this controller from the specified [id], or a
 * [EntityNotFoundException] will be thrown.
 *
 * @param id snowflake to find the entity
 * @return entity as [T], never null
 * @throws EntityNotFoundException If the entity couldn't be found.
 */
suspend fun <T, Entity: LongEntity, Created: Any, Patched: Any> AbstractDatabaseController<T, Entity, Created, Patched>.get(id: Long): T =
    getOrNull(id) ?: throw EntityNotFoundException(id)

/**
 * Retrieve an entity from this controller from the specified [expression][expr], or a
 * [EntityNotFoundException] will be thrown.
 *
 * @param expr Expression to resolve
 * @return entity as [T], never null
 * @throws EntityNotFoundException If the entity couldn't be found.
 */
@Suppress("UNCHECKED_CAST")
suspend fun <T, V, Entity: LongEntity, Created: Any, Patched: Any> AbstractDatabaseController<T, Entity, Created, Patched>.get(
    expr: Pair<KProperty0<Column<V>>, V>
): T = getOrNull(expr) ?: throw EntityNotFoundException(expr as Pair<KProperty0<Column<Any>>, Any>)

// special edge case for entity id -> long mapping.
/**
 * Retrieve an entity from this controller with a specified property from an [EntityID], or `null`
 * if the entity couldn't be found with this property.
 *
 * @param table The table that we should use to resolve the ID.
 * @param expr Expression to resolve
 * @return entity as [T], or `null` if it wasn't found.
 */
suspend fun <T, Table: SnowflakeTable, Entity: LongEntity, Created: Any, Patched: Any> AbstractDatabaseController<T, Entity, Created, Patched>.getOrNull(
    table: Table,
    expr: Pair<KProperty0<Column<EntityID<Long>>>, Long>
): T? = getOrNull(expr.first to EntityID(expr.second, table))
