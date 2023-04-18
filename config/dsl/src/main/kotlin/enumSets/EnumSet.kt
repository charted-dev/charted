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

package org.noelware.charted.configuration.kotlin.dsl.enumSets

import dev.floofy.utils.slf4j.logging
import kotlinx.serialization.SerialName
import kotlin.reflect.KClass

/**
 * Represents an enum set with a `wildcard`
 */
public abstract class EnumSet<E: Enum<E>>(private val klazz: KClass<E>) {
    private val log by logging<EnumSet<E>>()

    public abstract val wildcard: E

    @Suppress("MemberVisibilityCanBePrivate")
    public fun isWildcard(set: List<E>): Boolean {
        if (set.isEmpty()) return false
        return set.any { it == wildcard }
    }

    /**
     * Determines if an enum value is enabled with a provided set.
     * @param set The list of configured enums
     * @param value Enum value to check
     * @return true if the [set] contains [value] or is a wildcard value, or false
     * if not.
     */
    public fun enabled(set: List<E>, value: E): Boolean = isWildcard(set) || set.any { it == value }

    /**
     * Determines if an enum value from its [SerialName] annotation
     * is enabled or not from the [set].
     *
     * @param set The list of configured enums
     * @param value Value string from the [SerialName] annotation's value
     * @return true if the [set] contains [value] or is a wildcard value, or false
     * if not.
     */
    @Suppress("UNCHECKED_CAST")
    public fun enabled(set: List<E>, value: String): Boolean {
        val enumField = klazz.java.fields.firstOrNull { field ->
            if (!field.isEnumConstant) return@firstOrNull false
            if (field.isAnnotationPresent(SerialName::class.java)) {
                val serialName = field.getAnnotation(SerialName::class.java)!!
                return@firstOrNull serialName.value == value
            }

            false
        }?.get(null) as? E ?: return isWildcard(set) // check if we have a wildcard on an invalid value

        return enabled(set, enumField)
    }
}

public inline val <reified E: Enum<E>> E.serialName: String?
    get() {
        var result: String? = null
        for (field in this::class.java.fields) {
            if (!field.isEnumConstant) continue
            if (field.name == name) {
                result = field.getAnnotation(SerialName::class.java)?.value
                break
            }
        }

        return result
    }
