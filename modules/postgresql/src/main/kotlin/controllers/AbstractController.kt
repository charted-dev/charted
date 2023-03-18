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

import io.ktor.server.application.*
import org.jetbrains.exposed.dao.id.EntityID
import org.jetbrains.exposed.sql.Column
import org.noelware.charted.modules.postgresql.SnowflakeTable
import kotlin.reflect.KProperty0

/**
 * Represents a base abstraction for implementing database controllers. Since all database
 * entities have an ID, this controller will be expected to have methods like [getOrNull], [get],
 * [update], and [delete].
 *
 * @param T The entity that is represented for this controller.
 * @param Created Object that is used to create a [T] type.
 * @param Patched Object that is used to patch the [T] type.
 */
abstract class AbstractController<T, Created, Patched> {
    /**
     * Returns all the elements of this entity with an optional condition.
     * @param condition Optional condition to use
     * @return List of elements that were queried by the [condition], or all the entities
     * available otherwise.
     */
    abstract suspend fun <V> all(condition: Pair<KProperty0<Column<V>>, V>? = null): List<T>

    /**
     * Retrieve an entity from this controller from the specified [id], or `null`
     * if the entity couldn't be found.
     *
     * @param id The snowflake to find this entity
     * @return entity as [T], or `null` if it wasn't found.
     */
    abstract suspend fun getOrNull(id: Long): T?

    /**
     * Retrieve an entity from this controller with a specified property, or `null`
     * if the entity couldn't be found with this property.
     *
     * @param prop The property to check
     * @return entity as [T], or `null` if it wasn't found.
     */
    abstract suspend fun <V> getOrNullByProp(prop: KProperty0<Column<V>>, value: V): T?

    /**
     * Creates a new entity with the [Created] payload that is used to create
     * [T].
     *
     * @param data Payload that should be never null.
     * @return new entity as [T].
     */
    abstract suspend fun create(call: ApplicationCall, data: Created): T

    /**
     * Updates an entity with the specified [patched][Patched] data.
     *
     * @param id snowflake to update this entity
     * @param patched Patched data
     */
    abstract suspend fun update(call: ApplicationCall, id: Long, patched: Patched)

    /**
     * Deletes an entity from the database with the specified [id].
     * @param id snowflake to delete this entity
     */
    abstract suspend fun delete(id: Long)
}

/**
 * Retrieve an entity from this controller from the specified [id], or a
 * [EntityNotFoundException] will be thrown.
 *
 * @param id snowflake to find the entity
 * @return entity as [T], never null
 * @throws EntityNotFoundException If the entity couldn't be found.
 */
suspend fun <T, Created, Patched> AbstractController<T, Created, Patched>.get(id: Long): T =
    getOrNull(id) ?: throw EntityNotFoundException(id)

/**
 * Retrieve an entity from this controller with a specified property, or `null`
 * if the entity couldn't be found with this property.
 *
 * @param pair A pair of prop -> value to check for.
 * @return entity as [T], or `null` if it wasn't found.
 */
suspend fun <T, V, Created, Patched> AbstractController<T, Created, Patched>.getOrNullByProp(
    pair: Pair<KProperty0<Column<V>>, V>
): T? = getOrNullByProp(pair.first, pair.second)

/**
 * Retrieve an entity from this controller with a specified property, or `null`
 * if the entity couldn't be found with this property.
 *
 * @param pair A pair of prop -> value to check for.
 * @return entity as [T], or `null` if it wasn't found.
 * @throws EntityNotFoundException If the entity couldn't be found.
 */
suspend fun <T, V, Created, Patched> AbstractController<T, Created, Patched>.getByProp(
    pair: Pair<KProperty0<Column<V>>, V>
): T = getOrNullByProp(pair.first, pair.second) ?: throw EntityNotFoundException()
// ^ we can't infer that V is a long, so we will have to be vague about it

// special edge case for entity id -> long mapping.
/**
 * Retrieve an entity from this controller with a specified property from an [EntityID], or `null`
 * if the entity couldn't be found with this property.
 *
 * @param pair A pair of prop -> value to check for.
 * @return entity as [T], or `null` if it wasn't found.
 */
suspend fun <T, Table: SnowflakeTable, Created, Patched> AbstractController<T, Created, Patched>.getOrNullByProp(
    table: Table,
    pair: Pair<KProperty0<Column<EntityID<Long>>>, Long>
): T? = getOrNullByProp(pair.first, EntityID(pair.second, table))
