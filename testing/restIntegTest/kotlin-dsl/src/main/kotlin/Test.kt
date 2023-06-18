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

package org.noelware.charted.testing.restIntegTest.kotlin.dsl

import io.ktor.http.*
import org.noelware.charted.testing.restIntegTest.kotlin.dsl.operation.Operation
import org.noelware.charted.testing.restIntegTest.kotlin.dsl.operation.TestOperation
import kotlin.reflect.typeOf

public data class Test<Ctx: TestContext>(
    val name: String,
    val context: Ctx,
    val parent: TestBed<Ctx>,
    private val execute: suspend (Test<Ctx>) -> Unit
) {
    val testOperations: MutableList<TestOperation> = mutableListOf()

    @Suppress("UNCHECKED_CAST")
    public inline fun <reified T: Any> op(
        pairing: Pair<HttpMethod, String>,
        noinline block: (Operation<T>) -> Unit
    ): Test<Ctx> {
        val (method, path) = pairing
        if (method == HttpMethod.Get || method == HttpMethod.Head) {
            if (typeOf<T>() != typeOf<Unit>()) {
                throw IllegalCallerException("For Get/Head operations, you must use `Unit` as the reified generic type.")
            }
        }

        val op = if (testOperations.any { it.path == pairing.second }) testOperations.single { it.path == path } else TestOperation(path)
        op.addOperation(path, method, block as (Operation<*>) -> Unit)

        testOperations.remove(op)
        testOperations.add(op)

        return this
    }
}
