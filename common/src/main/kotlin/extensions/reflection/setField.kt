package org.noelware.charted.extensions.reflection

import org.noelware.charted.common.ReflectionUtils

public fun <T, U> T.setField(name: String, value: U): Boolean = ReflectionUtils.setField(this, name, value)
