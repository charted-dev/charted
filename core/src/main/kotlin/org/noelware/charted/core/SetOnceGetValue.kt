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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core

import java.util.concurrent.atomic.AtomicBoolean
import kotlin.properties.ReadOnlyProperty

/**
 * Represents a value that can be set once using `set value = <type>`.
 *
 * ## Example
 * ```kotlin
 * val getAndSet = SetOnceGetValue<Int>()
 * getAndSet.value = 1
 * // => 1
 *
 * getAndSet.value = 2
 * // => 1
 *
 * getAndSet.value
 * // => 1
 * ```
 */
class SetOnceGetValue<T> {
    private val setter = AtomicBoolean(false)
    private var holder: T? = null

    var value: T
        get() = holder ?: error("This value wasn't populated yet, use `SetOnceGetValue.value = type` to properly set it!")
        set(value) {
            if (setter.compareAndSet(false, true)) {
                holder = value
            }
        }

    var valueOrNull: T? = holder

    override fun hashCode(): Int = holder.hashCode()
    override fun equals(other: Any?): Boolean {
        val setOnceGetValue = other as? SetOnceGetValue<*> ?: return false
        if (setOnceGetValue.holder == null || this.holder == null) return false

        return this.holder == setOnceGetValue.holder
    }

    override fun toString(): String = "SetOnceGetValue(${if (holder == null) "<uninit>" else "$holder"})"
}

/**
 * Simple readonly property delegation to construct a [SetOnceGetValue] instance.
 * @return a [ReadOnlyProperty] instance of [SetOnceGetValue].
 */
fun <T> setOnceAndGet(): ReadOnlyProperty<Any?, SetOnceGetValue<T>> = ReadOnlyProperty { _, _ ->
    SetOnceGetValue()
}
