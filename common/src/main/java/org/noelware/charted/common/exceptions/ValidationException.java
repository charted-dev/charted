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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.common.exceptions;

/** Represents a validation exception within a Kotlinx Serialization object. */
public class ValidationException extends RuntimeException {
    private final String path;
    private final String message;

    public ValidationException(String path, String message) {
        super(String.format("[%s] %s", path, message));

        this.message = message;
        this.path = path;
    }

    public ValidationException(String path, String message, Exception cause) {
        super(String.format("[%s] %s", path, message), cause);

        this.message = message;
        this.path = path;
    }

    public String getValidationMessage() {
        return message;
    }

    public String getPath() {
        return path;
    }
}
