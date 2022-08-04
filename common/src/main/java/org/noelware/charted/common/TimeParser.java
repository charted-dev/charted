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

package org.noelware.charted.common;

import java.util.Locale;
import java.util.regex.Pattern;
import org.jetbrains.annotations.NotNull;

/**
 * This is a Java port of the NPM package: `ms` by Vercel
 *
 * @since 04.07.22
 * @author Noel
 * @credit https://github.com/vercel/ms/blob/master/src/index.ts
 */
public class TimeParser {
    private static final Long SECONDS = 1000L;
    private static final Long MINUTES = SECONDS * 60L;
    private static final Long HOURS = MINUTES * 60L;
    private static final Long DAYS = HOURS * 24L;
    private static final Long WEEKS = DAYS * 7;
    private static final Double YEARS = DAYS * 365.25;
    private static final Pattern REGEX = Pattern.compile(
            "^(-?(?:\\d+)?\\.?\\d+)"
                    + " *(milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|years?|yrs?|y)?$",
            Pattern.CASE_INSENSITIVE);

    private TimeParser() {}

    public static Long fromString(@NotNull String value) {
        var matcher = REGEX.matcher(value);
        if (!matcher.matches()) throw new IllegalStateException(String.format("Invalid value [%s]", value));

        var valueAsFloat = Float.parseFloat(matcher.group(1));
        var type = matcher.group(2);
        if (type == null) {
            type = "ms";
        }

        return switch (type.toLowerCase(Locale.ROOT)) {
            case "years", "year", "yrs", "yr", "y" -> (long) (valueAsFloat * YEARS);
            case "weeks", "week", "w" -> (long) (valueAsFloat * WEEKS);
            case "days", "day", "d" -> (long) (valueAsFloat * DAYS);
            case "minutes", "minute", "mins", "min", "m" -> (long) (valueAsFloat * MINUTES);
            case "seconds", "second", "secs", "sec", "s" -> (long) (valueAsFloat * SECONDS);
            case "milliseconds", "millisecond", "msecs", "msec", "ms" -> (long) valueAsFloat;
            default -> throw new IllegalStateException(String.format("Unexpected value: %s", type));
        };
    }

    public static String fromLong(Long value, boolean longSize) {
        if (longSize) {
            QuadConsumer<Long, Long, Long, String, String> _pluralize = (ms, msAbs, n, name) -> {
                var isPlural = msAbs >= (n * 1.5);
                var suffix = isPlural ? "s" : "";

                return String.format("%d %s%s", Math.round((double) (ms / n)), name, suffix);
            };

            var msAbs = Math.abs(value);
            if (msAbs >= YEARS) return _pluralize.accept(value, msAbs, YEARS.longValue(), "year");
            if (msAbs >= WEEKS) return _pluralize.accept(value, msAbs, WEEKS, "week");
            if (msAbs >= DAYS) return _pluralize.accept(value, msAbs, DAYS, "day");
            if (msAbs >= HOURS) return _pluralize.accept(value, msAbs, HOURS, "hour");
            if (msAbs >= MINUTES) return _pluralize.accept(value, msAbs, MINUTES, "minute");
            if (msAbs >= SECONDS) return _pluralize.accept(value, msAbs, SECONDS, "second");
        } else {
            var msAbs = Math.abs(value);
            if (msAbs >= YEARS) return String.format("%dy", Math.round(value / YEARS));
            if (msAbs >= WEEKS) return String.format("%dw", Math.round((double) (value / WEEKS)));
            if (msAbs >= DAYS) return String.format("%dd", Math.round((double) (value / DAYS)));
            if (msAbs >= HOURS) return String.format("%dh", Math.round((double) (value / HOURS)));
            if (msAbs >= MINUTES) return String.format("%dmin", Math.round((double) (value / MINUTES)));
            if (msAbs >= SECONDS) return String.format("%ds", Math.round((double) (value / SECONDS)));
        }

        return String.format("%dms", value);
    }
}
