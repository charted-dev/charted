package org.noelware.charted.database.cassandra.extensions

import com.datastax.oss.driver.api.core.cql.AsyncResultSet
import com.datastax.oss.driver.api.core.cql.Row

/**
 * Extension to call the iterator of [AsyncResultSet.currentPage]
 */
fun AsyncResultSet.iterator(): Iterator<Row> = currentPage().iterator()

/**
 * Extension to call [Iterator.hasNext].
 */
fun AsyncResultSet.hasNext(): Boolean = iterator().hasNext()
