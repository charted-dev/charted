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

package org.noelware.charted.testing.kubernetes.exceptions;

import java.io.IOException;
import java.io.InputStream;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.common.SetOnceGetValue;

public class GenericStdoutException extends RuntimeException {
    private final SetOnceGetValue<String> stdoutAsString = new SetOnceGetValue<>();
    private final InputStream stdout;

    public GenericStdoutException(String message, InputStream stdout) {
        super(message);

        this.stdout = stdout;
    }

    /**
     * Returns the standard input from the given process.
     */
    @NotNull
    public String stdout() throws IOException {
        if (stdoutAsString.wasSet()) return stdoutAsString.getValue();
        try (stdout) {
            final var bytes = stdout.readAllBytes();
            stdoutAsString.setValue(new String(bytes));
        }

        return stdoutAsString.getValue();
    }
}
