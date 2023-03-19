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

import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlin.math.abs
import kotlin.math.roundToLong
import kotlin.time.Duration
import kotlin.time.DurationUnit
import kotlin.time.toDuration

/**
 * Represents a slice of a specific point of time. This is represented as "2 seconds" -> 2000.
 */
@Schema(description = "Represents a slice of a specific point of time. This is represented as \"2 seconds\" -> 2000 milliseconds", implementation = Long::class)
@Serializable(with = TimeSpan.Companion.Serializer::class)
public class TimeSpan(public val value: Long) {
    /**
     * Returns a [Duration] of the given time span.
     * @param unit [DurationUnit] to use.
     */
    public fun toDuration(unit: DurationUnit): Duration = value.toDuration(unit)

    /**
     * String representation of this [TimeSpan].
     * @param long If we should use longer statements (i.e, `2 seconds`) if [long] is true, otherwise
     *             statements will be in (`2s`) form.
     */
    @Suppress("DuplicatedCode", "LiftReturnOrAssignment")
    public fun toString(long: Boolean = false): String {
        if (long) {
            fun pluralize(ms: Long, abs: Long, n: Long, name: String): String {
                val isPlural = abs >= (n * 1.5)
                val suffix = if (isPlural) "s" else ""

                return "${ms / n} $name$suffix"
            }

            val msAbs = abs(value)
            if (msAbs >= YEARS) return pluralize(value, msAbs, YEARS.roundToLong(), "year")
            if (msAbs >= MONTH) return pluralize(value, msAbs, MONTH, "month")
            if (msAbs >= WEEKS) return pluralize(value, msAbs, WEEKS, "week")
            if (msAbs >= DAYS) return pluralize(value, msAbs, DAYS, "day")
            if (msAbs >= HOURS) return pluralize(value, msAbs, HOURS, "hour")
            if (msAbs >= MINUTES) return pluralize(value, msAbs, MINUTES, "minute")
            if (msAbs >= SECONDS) {
                return pluralize(value, msAbs, SECONDS, "second")
            } else {
                return "$msAbs milliseconds"
            }
        } else {
            val msAbs = abs(value)
            if (msAbs >= YEARS) return "${(value / YEARS).roundToLong()}y"
            if (msAbs >= MONTH) return "${(value / MONTH).toDouble().roundToLong()}mo"
            if (msAbs >= WEEKS) return "${(value / WEEKS).toDouble().roundToLong()}w"
            if (msAbs >= DAYS) return "${(value / DAYS).toDouble().roundToLong()}d"
            if (msAbs >= HOURS) return "${(value / HOURS).toDouble().roundToLong()}h"
            if (msAbs >= MINUTES) return "${(value / MINUTES).toDouble().roundToLong()}m"
            if (msAbs >= SECONDS) {
                return "${(value / SECONDS).toDouble().roundToLong()}s"
            } else {
                return "${value}ms"
            }
        }
    }

    override fun toString(): String = toString(false)
    public companion object {
        public class Serializer: KSerializer<TimeSpan> {
            override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor("charted.TimeSpan", PrimitiveKind.STRING)
            override fun deserialize(decoder: Decoder): TimeSpan = ofString(decoder.decodeString())
            override fun serialize(encoder: Encoder, value: TimeSpan) {
                encoder.encodeString(value.toString())
            }
        }

        private val REGEX: Regex = """^(-?(?:\d+)?\.?\d+) *(milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|months?|mo|years?|yrs?|y)?$""".toRegex()

        private const val SECONDS: Long = 1000L
        private const val MINUTES: Long = SECONDS * 60
        private const val HOURS: Long = MINUTES * 60
        private const val DAYS: Long = HOURS * 24
        private const val WEEKS: Long = DAYS * 7
        private const val MONTH: Long = WEEKS * 4
        private const val YEARS: Double = DAYS * 365.25

        @JvmStatic
        public fun ofString(value: String): TimeSpan {
            val matcher = REGEX.toPattern().matcher(value)
            if (!matcher.matches()) throw IllegalStateException("Unable to parse '$value' as TimeSpan")

            val float = java.lang.Float.parseFloat(matcher.group(1))
            val type: String = if (matcher.group(2) == null) "ms" else matcher.group(2)

            return when (type.lowercase()) {
                "years", "year", "yrs", "yr", "y" -> TimeSpan((float * YEARS).roundToLong())
                "months", "month", "mo" -> TimeSpan((float * MONTH).roundToLong())
                "weeks", "week", "w" -> TimeSpan((float * WEEKS).roundToLong())
                "days", "day", "d" -> TimeSpan((float * DAYS).roundToLong())
                "hours", "hour", "hrs", "hr", "h" -> TimeSpan((float * HOURS).roundToLong())
                "minutes", "minute", "mins", "min", "m" -> TimeSpan((float * MINUTES).roundToLong())
                "seconds", "second", "secs", "sec", "s" -> TimeSpan((float * SECONDS).roundToLong())
                "milliseconds", "millisecond", "ms", "msec", "msecs" -> TimeSpan(float.roundToLong())
                else -> throw IllegalStateException("Unexpected value [$type]")
            }
        }
    }
}
