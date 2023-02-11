/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.common;

import java.util.Locale;
import java.util.regex.Pattern;
import org.jetbrains.annotations.NotNull;

/**
 * Represents a value that can be represented as a "byte" in a registered space. This is mainly used for
 * parsing purposes.
 */
public class ByteSizeValue {
    private static final Long BYTE = 1L;
    private static final Long KILOBYTE = BYTE * 1024;
    private static final Long MEGABYTE = KILOBYTE * 1024;
    private static final Long GIGABYTE = MEGABYTE * 1024;
    private static final Long TERABYTE = GIGABYTE * 1024;
    private static final Pattern REGEX = Pattern.compile(
            "^(-?(?:\\d+)?\\.?\\d+) *(b|mb|gb|tb|kb|bytes|kilobytes|megabytes|gigabytes|terabytes)?$",
            Pattern.CASE_INSENSITIVE);

    private ByteSizeValue() {
        /* no instance, just static :3 */
    }

    public static Long fromString(@NotNull String value) {
        final var matcher = REGEX.matcher(value);
        if (!matcher.matches()) throw new IllegalArgumentException("Invalid value [%s]".formatted(value));

        final var valueAsFloat = Float.parseFloat(matcher.group(1));
        var type = matcher.group(2);
        if (type == null) {
            type = "bytes";
        }

        return switch (type.toLowerCase(Locale.ROOT)) {
            case "terabytes", "tb" -> (long) (valueAsFloat * TERABYTE);
            case "gigabytes", "gb" -> (long) (valueAsFloat * GIGABYTE);
            case "megabytes", "mb" -> (long) (valueAsFloat * MEGABYTE);
            case "kilobytes", "kb" -> (long) (valueAsFloat * KILOBYTE);
            case "byte", "bytes", "b" -> (long) (valueAsFloat * BYTE);
            default -> throw new IllegalStateException(String.format("Unexpected value: %s", type));
        };
    }

    public static String fromLong(Long value) {
        return fromLong(value, false);
    }

    public static String fromLong(Long value, boolean longSize) {
        if (longSize) {
            QuadConsumer<Long, Long, Long, String, String> pluralize = (b, absolute, n, name) -> {
                final boolean isPlural = absolute >= (n * 1.5);
                final String suffix = isPlural ? "s" : "";

                return String.format("%d %s%s", Math.round((double) (b / n)), name, suffix);
            };

            final long result = Math.abs(value);
            if (result >= TERABYTE) return pluralize.accept(value, result, TERABYTE, "terabyte");
            if (result >= GIGABYTE) return pluralize.accept(value, result, GIGABYTE, "gigabyte");
            if (result >= MEGABYTE) return pluralize.accept(value, result, MEGABYTE, "megabyte");
            if (result >= KILOBYTE) return pluralize.accept(value, result, KILOBYTE, "kilobyte");

            return pluralize.accept(value, result, BYTE, "byte");
        } else {
            final long result = Math.abs(value);
            if (result >= TERABYTE) return String.format("%dtb", Math.round((double) (result / TERABYTE)));
            if (result >= GIGABYTE) return String.format("%dgb", Math.round((double) (result / GIGABYTE)));
            if (result >= MEGABYTE) return String.format("%dmb", Math.round((double) (result / MEGABYTE)));
            if (result >= KILOBYTE) return String.format("%dkb", Math.round((double) (result / KILOBYTE)));

            return String.format("%db", Math.round(result));
        }
    }
}
