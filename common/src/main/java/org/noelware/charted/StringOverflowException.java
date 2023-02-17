/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
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

public class StringOverflowException extends ValidationException {
    public StringOverflowException(String path, int maxSize, int length) {
        super(
                path,
                "String overflowed from %d characters, exceeded %d characters".formatted(maxSize, length - maxSize));
    }

    public StringOverflowException(String path, int maxSize) {
        super(
                path,
                String.format(
                        "String overflowed from %d characters, exceeded %d characters",
                        maxSize, path.length() - maxSize));
    }
}
