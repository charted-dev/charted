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

package org.noelware.charted.gradle.plugins.golang;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Locale;
import java.util.Map;
import org.jetbrains.annotations.Nullable;

/**
 * Represents a wrapper for running the <code>go env</code> command and
 * returning the results.
 */
public class GoEnv {
    /**
     * Returns the Go environment that is executed on the local system's <code>go compiler</code>.
     * @param execOperations The operations to use to execute the command.
     * @throws IOException If anything happens when closing the {@link ByteArrayOutputStream stdout}.
     * @return Immutable map of the environment, returns null if we can't fetch it from the local
     *         system's Go compiler.
     */
    @Nullable
    public static Map<String, String> getGoEnvironment(ExecOperationsLike execOperations) throws IOException {
        final var env = new HashMap<String, String>();
        try (final var stdout = new ByteArrayOutputStream()) {
            final var result = execOperations.exec(spec -> {
                spec.commandLine("go");
                spec.args("env");
                spec.setStandardOutput(stdout);
                spec.setIgnoreExitValue(true);
            });

            if (result.getExitValue() != 0) {
                return null;
            }

            final var data = stdout.toString().split("\n\r?");
            for (String line : data) {
                final var l = line.split("=", 2);
                if (l.length != 2) throw new IllegalArgumentException();

                final var name = l[0].toLowerCase(Locale.ROOT);
                final var value = l[1].replaceFirst("\"", "").replace("\"", "");

                env.putIfAbsent(name, value);
            }
        }

        return Collections.unmodifiableMap(env);
    }
}
