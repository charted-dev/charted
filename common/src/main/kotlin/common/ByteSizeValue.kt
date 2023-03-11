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

package org.noelware.charted.common

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlin.math.abs
import kotlin.math.roundToLong

/**
 * Represents a "byte" value (i.e, `1mb`, `2GB`, `31TB`)
 */
@Serializable(with = ByteSizeValue.Companion.Serializer::class)
public class ByteSizeValue(public val value: Long) {
    init {
        if (value < -1) throw IllegalStateException("Can't use under -1 as a byte value")
    }

    @Suppress("DuplicatedCode")
    public fun toString(long: Boolean): String {
        fun pluralize(b: Long, absolute: Long, n: Long, name: String): String {
            val isPlural = absolute >= (n * 1.5)
            val suffix = if (isPlural) "s" else ""

            return "${b / n} $name$suffix"
        }

        val res = abs(value)
        if (long) {
            if (res >= Unit.tera) return pluralize(value, res, Unit.tera, "terabyte")
            if (res >= Unit.giga) return pluralize(value, res, Unit.giga, "gigabyte")
            if (res >= Unit.mega) return pluralize(value, res, Unit.mega, "megabyte")
            if (res >= Unit.kilo) return pluralize(value, res, Unit.kilo, "kilobyte")

            return pluralize(value, res, Unit.byte, "byte")
        } else {
            if (res >= Unit.tera) return "${res}TB"
            if (res >= Unit.giga) return "${res}GB"
            if (res >= Unit.mega) return "${res}MB"
            if (res >= Unit.kilo) return "${res}KB"

            return "${res}B"
        }
    }

    override fun toString(): String = toString(false)
    public companion object {
        private val regex: Regex = """^(-?(?:\d+)?\.?\d+) *(b|mb|gb|tb|kb|B|KB|MB|GB|TB|bytes|kilobytes|megabytes|gigabytes|terabytes)?$""".toRegex()

        @JvmStatic
        public fun ofString(value: String): ByteSizeValue {
            val matcher = regex.toPattern().matcher(value)
            if (!matcher.matches()) throw IllegalStateException("Unable to parse '$value' as byte value")

            val float = java.lang.Float.parseFloat(matcher.group(1) ?: error("Unable to get first group"))
            val type = if (matcher.group(2) == null) "bytes" else matcher.group(2)

            return when (type.lowercase()) {
                "terabytes", "terabyte", "tb", "t" -> ByteSizeValue(float.roundToLong() * Unit.tera)
                "gigabytes", "gigabyte", "giga", "gb", "g" -> ByteSizeValue(float.roundToLong() * Unit.giga)
                "megabytes", "megabyte", "mb", "m" -> ByteSizeValue(float.roundToLong() * Unit.mega)
                "kilobytes", "kilobyte", "kb", "k" -> ByteSizeValue(float.roundToLong() * Unit.kilo)
                "bytes", "byte", "b" -> ByteSizeValue(float.roundToLong())
                else -> throw IllegalStateException("Unexpected value [$type]")
            }
        }

        public object Serializer: KSerializer<ByteSizeValue> {
            override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.ByteSizeValue", PrimitiveKind.STRING)
            override fun serialize(encoder: Encoder, value: ByteSizeValue) {
                encoder.encodeString(value.toString())
            }

            override fun deserialize(decoder: Decoder): ByteSizeValue = ofString(decoder.decodeString())
        }
    }

    /**
     * Unit for the [ByteSizeValue] to perform conversion
     */
    public enum class Unit {
        TERABYTE {
            override fun toTerabyte(value: Long): Long = value
            override fun toGigabyte(value: Long): Long = value / (giga / tera)
            override fun toMegabyte(value: Long): Long = preventOverflow(value, tera / mega, Long.MAX_VALUE / (tera / mega))
            override fun toKilobyte(value: Long): Long = preventOverflow(value, tera / kilo, Long.MAX_VALUE / (tera / kilo))
            override fun toByte(value: Long): Long = preventOverflow(value, tera / byte, Long.MAX_VALUE / (tera / byte))
        },
        GIGABYTE {
            override fun toTerabyte(value: Long): Long = value / (tera / giga)
            override fun toGigabyte(value: Long): Long = value
            override fun toMegabyte(value: Long): Long = preventOverflow(value, giga / mega, Long.MAX_VALUE / (giga / mega))
            override fun toKilobyte(value: Long): Long = preventOverflow(value, giga / kilo, Long.MAX_VALUE / (giga / kilo))
            override fun toByte(value: Long): Long = preventOverflow(value, giga / byte, Long.MAX_VALUE / (giga / byte))
        },
        MEGABYTE {
            override fun toTerabyte(value: Long): Long = value / (tera / mega)
            override fun toGigabyte(value: Long): Long = value / (giga / mega)
            override fun toMegabyte(value: Long): Long = value
            override fun toKilobyte(value: Long): Long = preventOverflow(value, mega / kilo, Long.MAX_VALUE / (mega / kilo))
            override fun toByte(value: Long): Long = preventOverflow(value, mega / byte, Long.MAX_VALUE / (mega / byte))
        },
        KILOBYTE {
            override fun toTerabyte(value: Long): Long = value / (tera / kilo)
            override fun toGigabyte(value: Long): Long = value / (giga / kilo)
            override fun toMegabyte(value: Long): Long = value / (mega / kilo)
            override fun toKilobyte(value: Long): Long = value
            override fun toByte(value: Long): Long = preventOverflow(value, kilo / byte, Long.MAX_VALUE / (kilo / byte))
        },
        BYTE {
            override fun toTerabyte(value: Long): Long = value / (tera / byte)
            override fun toGigabyte(value: Long): Long = value / (giga / byte)
            override fun toMegabyte(value: Long): Long = value / (mega / byte)
            override fun toKilobyte(value: Long): Long = value / (kilo / byte)
            override fun toByte(value: Long): Long = value
        };

        public abstract fun toTerabyte(value: Long): Long
        public abstract fun toGigabyte(value: Long): Long
        public abstract fun toMegabyte(value: Long): Long
        public abstract fun toKilobyte(value: Long): Long
        public abstract fun toByte(value: Long): Long

        internal fun preventOverflow(value: Long, max: Long, over: Long): Long = when {
            value > over -> Long.MAX_VALUE
            value < -over -> Long.MIN_VALUE
            else -> value * max
        }

        internal companion object {
            internal val byte: Long = 1
            internal val kilo: Long = byte * 1024L
            internal val mega: Long = kilo * 1024L
            internal val giga: Long = mega * 1024L
            internal val tera: Long = giga * 1024L
        }
    }
}

public val ByteSizeValue.Unit.suffix: String
    get() = when (this) {
        ByteSizeValue.Unit.TERABYTE -> "TB"
        ByteSizeValue.Unit.GIGABYTE -> "GB"
        ByteSizeValue.Unit.MEGABYTE -> "MB"
        ByteSizeValue.Unit.KILOBYTE -> "KB"
        ByteSizeValue.Unit.BYTE -> "B"
    }
