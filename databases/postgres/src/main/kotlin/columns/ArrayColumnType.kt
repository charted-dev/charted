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

package org.noelware.charted.databases.postgres.columns

import org.jetbrains.exposed.sql.*
import org.jetbrains.exposed.sql.statements.jdbc.JdbcConnectionImpl
import org.jetbrains.exposed.sql.transactions.TransactionManager
import org.postgresql.jdbc.PgArray

fun <T> Table.array(name: String, type: ColumnType): Column<Array<T>> = registerColumn(name, ArrayColumnType(type))

class ArrayColumnType(private val type: ColumnType): ColumnType() {
    override fun sqlType(): String = "${type.sqlType()} ARRAY"
    override fun valueToDB(value: Any?): Any? =
        if (value is Array<*>) {
            val columnType = type.sqlType().split("(")[0]
            val connection = (TransactionManager.current().connection as JdbcConnectionImpl).connection
            connection.createArrayOf(columnType, value)
        } else {
            super.valueToDB(value)
        }

    @Suppress("UNCHECKED_CAST")
    override fun valueFromDB(value: Any): Array<*> {
        if (value is PgArray) {
            return value.array as Array<*>
        }

        if (value is java.sql.Array) {
            return value.array as Array<*>
        }

        if (value is Array<*>) return value

        error("Unable to return an Array from a non-array value. ($value, ${value::class})")
    }

    override fun notNullValueToDB(value: Any): Any {
        if (value is Array<*>) {
            if (value.isEmpty()) return "'{}'"

            val columnType = type.sqlType().split("(")[0]
            val connection = (TransactionManager.current().connection as JdbcConnectionImpl).connection
            return connection.createArrayOf(columnType, value)
        } else {
            return super.notNullValueToDB(value)
        }
    }
}

private class ContainsOp(expr1: Expression<*>, expr2: Expression<*>): ComparisonOp(expr1, expr2, "@>")
infix fun <T, S> ExpressionWithColumnType<T>.contains(array: Array<in S>): Op<Boolean> = ContainsOp(this, QueryParameter(array, columnType))

class AnyOp(val expr1: Expression<*>, val expr2: Expression<*>): Op<Boolean>() {
    override fun toQueryBuilder(queryBuilder: QueryBuilder) {
        if (expr2 is OrOp) {
            queryBuilder.append("(").append(expr2).append(")")
        } else {
            queryBuilder.append(expr2)
        }

        queryBuilder.append(" = ANY (")
        if (expr1 is OrOp) {
            queryBuilder.append("(").append(expr1).append(")")
        } else {
            queryBuilder.append(expr1)
        }

        queryBuilder.append(")")
    }
}

infix fun <T, S> ExpressionWithColumnType<T>.any(v: S): Op<Boolean> = if (v == null) {
    IsNullOp(this)
} else {
    AnyOp(this, QueryParameter(v, columnType))
}
