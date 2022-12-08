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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

@file:JvmName("ChartedGrpcExtensionsKt")

package org.noelware.charted.modules.analytics.kotlin.dsl

import com.google.protobuf.NullValue
import com.google.protobuf.Struct
import com.google.protobuf.Value
import org.noelware.analytics.jvm.server.serialization.Serializable
import org.noelware.analytics.jvm.server.util.GrpcValueUtil
import kotlin.reflect.KProperty1

interface ToGrpcValue {
    fun toGrpcValue(): Value
}

/**
 * Kotlin DSL for creating a [Struct].
 * @param builder struct builder to use
 * @return constructed [Struct] object
 */
fun Struct(builder: Struct.Builder.() -> Unit = {}): Struct = Struct.newBuilder().apply(builder).build()

/**
 * Appends a new value into the [Struct.Builder] with the given [KProperty1].
 *
 * ```kotlin
 * @Serializable
 * data class MyStruct(val id: Int): org.noelware.analytics.jvm.server.serialization.Serializable {
 *    override fun toGrpcValue(): Value = Struct {
 *       put(this, MyStruct::id)
 *    }.toGrpcValue()
 * }
 * ```
 *
 * @param builder The [Struct.Builder] to append the value to
 * @param property The [KProperty1] to use for the name and value. The value must be:
 *
 *      - String
 *      - Number (double, int, long, short, byte)
 *      - Boolean
 *      - List<*>
 *      - Struct
 *      - Any that implements org.noelware.analytics.jvm.server.serialization.Serializable
 *
 * @return The [Struct.Builder] to chain methods
 */
fun <T: Any, U: Any> T.put(builder: Struct.Builder, property: KProperty1<T, U?>): Struct.Builder {
    val result = property.get(this)
    builder.putFields(property.name, result.toGrpcValue())

    return builder
}

private fun Any?.toGrpcValue(): Value = when (this) {
    null -> Value.newBuilder().apply { nullValue = NullValue.NULL_VALUE }.build()
    is String -> toGrpcValue()
    is Number -> toGrpcValue()
    is Boolean -> toGrpcValue()
    is List<*> -> toGrpcValue()
    is Struct -> toGrpcValue()
    is Serializable -> toGrpcValue()
    is Map<*, *> -> Struct {
        for ((key, value) in this@toGrpcValue) {
            if (key !is String) throw IllegalStateException("Map keys should always be strings, received $key")
            putFields(key, value.toGrpcValue())
        }
    }.toGrpcValue()

    else -> throw IllegalStateException("Value $this doesn't implement `Serializable`")
}

/**
 * Transforms this string into a [Value]. This internally uses the [GrpcValueUtil.toValue] method.
 * @return [serializable gRPC value][Value]
 */
fun String.toGrpcValue(): Value = GrpcValueUtil.toValue(this)

/**
 * Transforms this number into a [Value]. This internally uses the [GrpcValueUtil.toValue] method.
 * @return [serializable gRPC value][Value]
 */
fun Number.toGrpcValue(): Value = GrpcValueUtil.toValue(this)

/**
 * Transforms this boolean into a [Value]. This internally uses the [GrpcValueUtil.toValue] method.
 * @return [serializable gRPC value][Value]
 */
fun Boolean.toGrpcValue(): Value = GrpcValueUtil.toValue(this)

/**
 * Transforms this list into a [Value]. This internally uses the [GrpcValueUtil.toValue] method.
 * @return [serializable gRPC value][Value]
 */
fun <T> List<T>.toGrpcValue(): Value = GrpcValueUtil.toValue(this)

/**
 * Transforms a [Struct] into a [Value]. Internally uses the [GrpcValueUtil.toValue] method.
 * @return [serializable gRPC value][Value]
 */
fun Struct.toGrpcValue(): Value = GrpcValueUtil.toValue(this)
