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

package org.noelware.charted;

import static java.lang.String.format;

public class StringOverflowException extends ValidationException {
    private final int maxLength;
    private final int len;

    public StringOverflowException(String path, int maxLength, int len) {
        super(
                path,
                format(
                        "String has exceeded to %d characters, need less than %d characters",
                        maxLength, len - maxLength));

        this.maxLength = maxLength;
        this.len = len;
    }

    /** @return Length of the data scalar that was over the {@link #maxLength()} */
    public int length() {
        return len;
    }

    /** @return Maximum length to complete the overflowed data scalar */
    public int maxLength() {
        return maxLength;
    }
}
