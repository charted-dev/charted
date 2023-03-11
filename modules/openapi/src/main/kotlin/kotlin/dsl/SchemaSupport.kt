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

package org.noelware.charted.modules.openapi.kotlin.dsl

import org.noelware.charted.annotations.ChartedDsl
import kotlin.reflect.KType
import kotlin.reflect.typeOf

/**
 * Represents an interface to have a `schema()` function without repeating it on DSL
 * objects that might require it.
 */
@ChartedDsl
interface SchemaSupport {
    /**
     * The schema that this DSL object might use to represent itself
     * @param type [KType] of the object that is the schema itself
     */
    fun schema(type: KType)
}

/**
 * Uses the [typeOf] function to get the type of schema it is
 * represented as.
 */
inline fun <reified T> SchemaSupport.schema() {
    schema(typeOf<T>())
}
