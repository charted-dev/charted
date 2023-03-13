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

import org.jetbrains.annotations.Nullable;

/**
 * Represents a {@link RuntimeException} of a validation error from any source.
 */
public class ValidationException extends RuntimeException {
    private final String codeToUse;
    private final String message;
    private final String path;

    public ValidationException(String path, String message) {
        this(path, message, null);
    }

    public ValidationException(String path, String message, @Nullable String codeToUse) {
        super(format("[%s] %s", path, message));

        this.codeToUse = codeToUse;
        this.message = message;
        this.path = path;
    }

    /** @return API error code to use */
    @Nullable
    public String codeToUse() {
        return codeToUse;
    }

    /** @return validation message */
    public String validationMessage() {
        return message;
    }

    /** @return validation path */
    public String path() {
        return path;
    }
}
