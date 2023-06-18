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

import kotlin.reflect.KClass

/**
 * Represents a context for a [TestBed].
 */
public interface TestContext {
    /**
     * Injects a class from a Koin module, returns `null` if the injectable
     * was not found.
     *
     * @param klazz The [KClass] to use to reference the injectable.
     * @return The injected instance, or `null` if it was not found.
     */
    public fun <T: Any> injectOrNull(klazz: KClass<*>): T?

    /**
     * Injects a class from a Koin module, will throw a [NullPointerException]
     * if it was not found.
     *
     * @param klazz The [KClass] to use to reference the injectable.
     * @throws NullPointerException If the [klazz] was not available from the Koin module.
     * @return The injected instance.
     */
    public fun <T: Any> inject(klazz: KClass<*>): T = injectOrNull(klazz) ?: throw NullPointerException()
}
