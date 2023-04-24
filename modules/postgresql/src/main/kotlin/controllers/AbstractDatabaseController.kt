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
import org.jetbrains.exposed.dao.LongEntity
import org.jetbrains.exposed.dao.LongEntityClass
import org.jetbrains.exposed.sql.Column
import org.jetbrains.exposed.sql.Op
import org.jetbrains.exposed.sql.SqlExpressionBuilder
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.modules.postgresql.SnowflakeTable
import org.noelware.charted.modules.postgresql.asyncTransaction
import kotlin.reflect.KProperty0

/**
 * Represents an abstraction for creating database controllers. Since we fetch, update,
 * delete, and create data mostly the same, this was needed to repeat ourselves
 * when using in the API server.
 *
 * @param T The entity class that is represented
 * @param Entity The backing Exposed [Entity] for this controller
 * @param Created Object that is used to create a [T] type.
 * @param Patched Object that is used to patch the [T] type.
 * @param table The [SnowflakeTable] to do operations on
 * @param entityClass The [LongEntityClass] to perform SQL expressions on
 */
@Suppress("MemberVisibilityCanBePrivate")
abstract class AbstractDatabaseController<T, Entity: LongEntity, Created: Any, Patched: Any>(
    internal val table: SnowflakeTable,
    internal val entityClass: LongEntityClass<Entity>,
    internal val onEntityResolve: (entity: Entity) -> T
) {
    // these are all unique to each entity, so it must be explicitly specified.
    /**
     * Creates a new entity with the [Created] payload that is used to create
     * [T].
     *
     * @param call Underlying [ApplicationCall] from the API server
     * @param data Payload that should be never null.
     * @return new entity as [T].
     */
    abstract suspend fun create(call: ApplicationCall, data: Created): T

    /**
     * Updates an entity with the specified [patched][Patched] data.
     *
     * @param call Underlying [ApplicationCall] from the API server
     * @param id snowflake to update this entity
     * @param patched Patched data
     */
    abstract suspend fun update(call: ApplicationCall, id: Long, patched: Patched)

    /**
     * Retrieve the [Entity] from this controller with a specified [id], or `null`
     * if the [Entity] couldn't be found.
     *
     * @param id The Snowflake to resolve the entity as
     * @return the entity as [Entity], or `null` if it wasn't found.
     */
    suspend fun getEntityOrNull(id: Long): Entity? = getEntityOrNull { table.id eq id }

    /**
     * Retrieve the [Entity] from this controller with one or multiple pairs
     * of SQL expressions to resolve this [Entity].
     *
     * @param expr The expression to resolve
     * @return entity as [Entity], or `null` if it wasn't found
     */
    suspend fun <V> getEntityOrNull(expr: Pair<KProperty0<Column<V>>, V>): Entity? = getEntityOrNull {
        expr.first.get() eq expr.second
    }

    /**
     * Retrieve the [Entity] from this controller with a SQL expression builder. Alias for
     * [org.jetbrains.exposed.dao.EntityClass.find] but returns the first element
     * it could find, or `null` if it couldn't.
     *
     * @param sql The SQL expression builder
     * @return entity as [Entity], or `null` if it wasn't found
     */
    suspend fun getEntityOrNull(sql: SqlExpressionBuilder.() -> Op<Boolean>): Entity? = asyncTransaction {
        entityClass.find(sql).firstOrNull()
    }

    /**
     * Retrieve an entity from this controller from the specified [id], or `null`
     * if the entity couldn't be found.
     *
     * @param id The snowflake to find this entity
     * @return entity as [T], or `null` if it wasn't found.
     */
    suspend fun getOrNull(id: Long): T? = getEntityOrNull(id)?.let(onEntityResolve)

    /**
     * Retrieve an entity from this controller with a specified property, or `null`
     * if the entity couldn't be found with this property.
     *
     * @param expr Expression to resolve
     * @return entity as [T], or `null` if it wasn't found.
     */
    suspend fun <V> getOrNull(expr: Pair<KProperty0<Column<V>>, V>): T? = getEntityOrNull(expr)?.let(onEntityResolve)

    /**
     * Returns all the elements of this entity with an optional condition.
     * @param expr Optional expression to resolve, or `null` to resolve all entities. This might
     * be slow!
     *
     * @return List of elements that were queried by the [expr], or all the entities
     * available otherwise.
     */
    suspend fun <V> all(expr: Pair<KProperty0<Column<V>>, V>? = null): List<T> = asyncTransaction {
        if (expr == null) {
            entityClass.all().map(onEntityResolve)
        } else {
            entityClass.find { expr.first.get() eq expr.second }.map(onEntityResolve)
        }
    }

    /**
     * Deletes an entity from the database with the specified [id].
     * @param id snowflake to delete this entity
     */
    suspend fun delete(id: Long): Unit = asyncTransaction {
        table.deleteWhere { table.id eq id }
    }
}
