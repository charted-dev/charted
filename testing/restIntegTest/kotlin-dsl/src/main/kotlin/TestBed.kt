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

import org.noelware.charted.annotations.ChartedDsl

/**
 * Represents a test bed that has multiple tests configured to it.
 * @param name The name of the test bed
 * @param context The context that this [TestBed] is configured to use.
 */
@ChartedDsl
public data class TestBed<Ctx: TestContext>(
    val name: String,
    val context: Ctx
) {
    private val allTests = mutableListOf<Test<Ctx>>()

    /**
     * Registers a singular test into this test bed.
     * @param name The name of the test.
     * @param execute Function to execute when this test is called.
     */
    public fun test(name: String, execute: suspend (Test<Ctx>) -> Unit): TestBed<Ctx> {
        allTests.add(Test(name, context, this, execute))
        return this
    }
}
