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

package org.noelware.charted.testing.restIntegTest.kotlin.dsl.matchers

import kotlin.reflect.KClass

/**
 * Represents a generic matcher that can be used to do a bunch of
 * assertions. This adds custom assertions and brings in JUnit5's assertions
 * as well.
 */
public interface MatcherContext {
    /**
     * Asserts that the response body that was executed from a test operation was [T] or
     * not.
     *
     * @param expected Expected class
     */
    public fun <T: Any> assertResponseBody(expected: KClass<T>): T
}

public inline fun <reified T: Any> MatcherContext.assertResponseBody(): T = assertResponseBody(T::class)
