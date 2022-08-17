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

@file:JvmName("AnalyticsExtensionsKt")

package org.noelware.charted.analytics

import com.google.protobuf.ListValue
import com.google.protobuf.NullValue
import com.google.protobuf.Struct
import com.google.protobuf.StructKt
import com.google.protobuf.Value
import org.noelware.charted.analytics.protobufs.v1.BuildFlavour
import org.noelware.charted.common.DistributionType

fun DistributionType.toBuildFlavour(): BuildFlavour = when (this) {
    DistributionType.DOCKER -> BuildFlavour.DOCKER
    DistributionType.DEB -> BuildFlavour.DEB
    DistributionType.RPM -> BuildFlavour.RPM
    DistributionType.GIT -> BuildFlavour.GIT
    else -> BuildFlavour.UNRECOGNIZED
}

fun StructKt.Dsl.put(key: String, value: Boolean?) {
    fields.put(
        key,
        Value.newBuilder().apply {
            if (value != null) boolValue = value
            else nullValue = NullValue.NULL_VALUE
        }.build()
    )
}

fun StructKt.Dsl.put(key: String, value: String?) {
    fields.put(
        key,
        Value.newBuilder().apply {
            if (value != null) stringValue = value
            else nullValue = NullValue.NULL_VALUE
        }.build()
    )
}

fun StructKt.Dsl.put(key: String, value: Number?) {
    fields.put(
        key,
        Value.newBuilder().apply {
            if (value != null) numberValue = value.toDouble()
            else nullValue = NullValue.NULL_VALUE
        }.build()
    )
}

@JvmName("putStringList")
fun StructKt.Dsl.put(key: String, value: List<String?>) {
    val values = value.map { v ->
        Value.newBuilder().apply {
            if (v != null) stringValue = v
            else nullValue = NullValue.NULL_VALUE
        }.build()
    }

    fields.put(key, Value.newBuilder().setListValue(ListValue.newBuilder().addAllValues(values).build()).build())
}

@JvmName("putBooleanList")
fun StructKt.Dsl.put(key: String, value: List<Boolean?>) {
    val values = value.map { v ->
        Value.newBuilder().apply {
            if (v != null) boolValue = v
            else nullValue = NullValue.NULL_VALUE
        }.build()
    }

    fields.put(key, Value.newBuilder().setListValue(ListValue.newBuilder().addAllValues(values).build()).build())
}

@JvmName("putNumberList")
fun StructKt.Dsl.put(key: String, value: List<Number?>) {
    val values = value.map { v ->
        Value.newBuilder().apply {
            if (v != null) numberValue = v.toDouble()
            else nullValue = NullValue.NULL_VALUE
        }.build()
    }

    fields.put(key, Value.newBuilder().setListValue(ListValue.newBuilder().addAllValues(values).build()).build())
}

@JvmName("putStructList")
fun StructKt.Dsl.put(key: String, value: List<Struct?>) {
    val values = value.map { v ->
        Value.newBuilder().apply {
            if (v != null) structValue = v
            else nullValue = NullValue.NULL_VALUE
        }.build()
    }

    fields.put(key, Value.newBuilder().setListValue(ListValue.newBuilder().addAllValues(values).build()).build())
}

fun StructKt.Dsl.put(key: String, struct: Struct) {
    val value = Value.newBuilder().setStructValue(struct).build()
    fields.put(key, value)
}
