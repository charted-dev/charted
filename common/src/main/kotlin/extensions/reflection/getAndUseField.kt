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

package org.noelware.charted.extensions.reflection

import org.noelware.charted.common.ReflectionUtils

/**
 * A Kotlin extension for the [ReflectionUtils.getAndUseField] method, which uses a reified type for the given
 * inferred type.
 *
 * @param fieldName The field's name to fetch
 */
inline fun <reified U, T> T.getAndUseField(fieldName: String): U? = ReflectionUtils.getAndUseField(
    this,
    U::class.java,
    fieldName
)
