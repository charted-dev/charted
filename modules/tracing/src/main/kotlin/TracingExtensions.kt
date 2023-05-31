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

package org.noelware.charted.modules.tracing

import kotlin.contracts.ExperimentalContracts
import kotlin.contracts.InvocationKind
import kotlin.contracts.contract

object NoopTracer: Tracer {
    override fun createTransaction(name: String, operation: String?): Transaction = NoopTransaction(name, operation)
    override fun createTransaction(name: String): Transaction = createTransaction(name, null)
    override fun currentTransaction(): Transaction? = null
    override fun withTransaction(name: String, operation: String?): AutoCloseable = AutoCloseable {}
    override fun withTransaction(name: String): AutoCloseable = withTransaction(name, null)

    override fun close() {
        // do nothing
    }
}

private class NoopTransaction(private val name: String, private val operation: String? = null): Transaction {
    override fun createSpan(name: String, operation: String?): Span = NoopSpan(name, operation, this)
    override fun createSpan(name: String): Span = createSpan(name, null)
    override fun tracer(): Tracer = NoopTracer
    override fun operation(): String? = operation
    override fun name(): String = name

    override fun end(throwable: Throwable?) {
        // do nothing
    }
}

private class NoopSpan(private val name: String, private val operation: String? = null, private val parent: NoopTransaction): Span {
    override fun transaction(): Transaction = parent
    override fun operation(): String? = operation
    override fun name(): String = name
}

@OptIn(ExperimentalContracts::class)
fun withTracing(name: String, operation: String? = null, block: Transaction.() -> Unit = {}) {
    contract { callsInPlace(block, InvocationKind.EXACTLY_ONCE) }

    val tracer = Tracer.globalOrNull() ?: NoopTracer
    val transaction = tracer.createTransaction(name, operation)
    return transaction.block().also { transaction.end(null) }
}
