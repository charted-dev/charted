package org.noelware.charted.gradle.plugins.golang;

import org.gradle.process.ExecOperations;
import org.jetbrains.annotations.Nullable;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

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
            for (String line: data) {
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
