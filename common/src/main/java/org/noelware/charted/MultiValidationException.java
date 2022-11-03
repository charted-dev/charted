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

package org.noelware.charted;

import java.util.Collections;
import java.util.List;

public class MultiValidationException extends RuntimeException {
    private final List<ValidationException> exceptions;

    public MultiValidationException(List<ValidationException> exceptions) {
        super();

        this.exceptions = exceptions;
    }

    @Override
    public String getMessage() {
        return doFormat(exceptions);
    }

    public List<ValidationException> exceptions() {
        return Collections.unmodifiableList(exceptions);
    }

    private String doFormat(List<ValidationException> exceptions) {
        final var builder = new StringBuilder();
        for (ValidationException ex : exceptions) {
            // [body.username] Username "noel" didn't follow validations.
            builder.append("[").append(ex.getPath()).append("]");
            builder.append(" ").append(ex.getValidationMessage()).append('\n');
        }

        return builder.toString();
    }
}
