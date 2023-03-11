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

package org.noelware.charted.common.extensions.setonce

import dev.floofy.utils.java.SetOnce
import kotlin.reflect.KProperty

public operator fun <T: Any> SetOnce<T>.getValue(thisRef: Any?, property: KProperty<*>): T? = valueOrNull
public operator fun <T: Any> SetOnce<T>.setValue(thisRef: Any?, property: KProperty<*>, value: T?) {
    if (wasSet()) throw IllegalStateException("Value can't be changed")
    if (value == null) throw IllegalStateException("Can't set value to `null` since that's the default.")

    this.value = value
}
