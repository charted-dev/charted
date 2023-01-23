package org.noelware.charted.extensions.reflection

import org.noelware.charted.common.ReflectionUtils

/**
 * Reflectively set a field by [name] and setting the value to what [value] is.
 * @param name The field name
 * @param value The field value
 */
public fun <T, U> T.setField(name: String, value: U): Boolean = ReflectionUtils.setField(this, name, value)
