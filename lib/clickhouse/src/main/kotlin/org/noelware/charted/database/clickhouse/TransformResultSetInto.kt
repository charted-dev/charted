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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.database.clickhouse

import java.sql.ResultSet

/**
 * Transforms a [ResultSet][java.sql.ResultSet] into the object you want as [T].
 */
interface TransformResultSetInto<T> {
    /**
     * Transforms a [ResultSet][java.sql.ResultSet] into the object you want as [T].
     * @param rs The result set object from the SQL query.
     */
    fun transform(rs: ResultSet): T
}

/**
 * Simple function to convert an inner [transform] function into a [TransformResultSetInto] object.
 * @param transform The inner lambda function to convert into the object as [T].
 * @return A transformer object.
 */
fun <T> transformInto(transform: ResultSet.() -> T): TransformResultSetInto<T> = object: TransformResultSetInto<T> {
    override fun transform(rs: ResultSet): T = rs.transform()
}
