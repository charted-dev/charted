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

package org.noelware.charted.gradle.util;

import java.util.Arrays;
import java.util.Objects;
import org.jetbrains.annotations.NotNull;

public class ArrayUtil {
    // no direct construct pls
    private ArrayUtil() {}

    /**
     * Pops the last element out of the originating array and returns a new copied
     * array that goes from last <- first.
     *
     * @param array Array to "pop" its last item from
     * @return New copied array
     */
    public static <T> T[] pop(@NotNull T[] array) {
        Objects.requireNonNull(array, "Array cannot be null");
        return Arrays.copyOf(array, array.length - 1);
    }
}
