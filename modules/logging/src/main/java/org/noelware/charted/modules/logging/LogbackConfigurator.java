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

package org.noelware.charted.modules.logging;

import static java.lang.String.format;

import ch.qos.logback.classic.LoggerContext;
import ch.qos.logback.classic.joran.JoranConfigurator;
import ch.qos.logback.classic.spi.Configurator;
import ch.qos.logback.core.joran.spi.JoranException;
import ch.qos.logback.core.spi.ContextAwareBase;
import java.io.*;
import java.net.URL;
import java.nio.file.Files;
import java.util.Map;
import java.util.Objects;
import java.util.Properties;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * {@link Configurator} that configures Logback with the XML configuration from different
 * places:
 *
 * <ul>
 *     <li><code>CHARTED_LOGBACK_CONFIG_FILE</code> environment variable</li>
 *     <li><code>org.noelware.charted.logback.config</code> system property</li>
 *     <li>in ./config/logback.properties or ./logback.properties</li>
 *     <li>in classpath (/config/logback.properties)</li>
 * </ul>
 *
 * If the clauses weren't met, then the configurator will provide some defaults:
 * <ul>
 *     <li><code>charted.log.level</code> is set to INFO</li>
 *     <li>JSON logging is enabled, not prettified console output</li>
 * </ul>
 */
public class LogbackConfigurator extends ContextAwareBase implements Configurator {
    private static final String ENVIRONMENT_VARIABLE_NAME = "CHARTED_LOGBACK_CONFIG_FILE";
    private static final String SYSTEM_PROPERTY_NAME = "org.noelware.charted.logback.config";

    @Override
    public ExecutionStatus configure(LoggerContext context) {
        // Set the current context we received as the current one
        setContext(context);

        // PRIORITY: env > system props > ./config/logback.properties > ./config/logback.properties > classpath >
        // default
        if (resolveFileFromPath(context, System.getenv(ENVIRONMENT_VARIABLE_NAME))) return process0(context);
        if (resolveFileFromPath(context, System.getProperty(SYSTEM_PROPERTY_NAME))) return process0(context);

        final File configDir = new File("./config");
        if (configDir.exists() && configDir.isDirectory()) {
            if (resolveFileFromPath(context, new File(configDir, "logback.properties").getAbsolutePath()))
                return process0(context);
        }

        final File logbackFile = new File("./logback.properties");
        if (logbackFile.exists() && resolveFileFromPath(context, logbackFile.getAbsolutePath()))
            return process0(context);

        final URL resourcePath = getClass().getResource("/config/logback.properties");
        if (resourcePath != null) {
            final InputStream is = getClass().getResourceAsStream("/config/logback.properties");
            applyProperties(context, is);

            return process0(context);
        }

        context.putProperty("charted.log.level", "INFO");
        context.putProperty("charted.console.json", "yes");

        return process0(context);
    }

    /**
     * Processes the given {@link LoggerContext} with Joran, which processes the <code>/config/logback.xml</code>
     * file in the server JAR.
     *
     * @param context Logger context that was previously configured from {@link #configure(LoggerContext)}
     * @return the {@link Configurator.ExecutionStatus} when loading /config/logback.xml
     */
    private Configurator.ExecutionStatus process0(LoggerContext context) {
        try {
            final JoranConfigurator configurator = new JoranConfigurator();
            configurator.setContext(context);
            configurator.doConfigure(Objects.requireNonNull(getClass().getResource("/config/logback.xml")));

            return ExecutionStatus.DO_NOT_INVOKE_NEXT_IF_ANY;
        } catch (JoranException e) {
            throw new RuntimeException("Unable to configure Joran with /config/logback.xml:", e);
        }
    }

    /**
     * Resolves a file from the given <code>filePath</code>
     * @param context the {@link LoggerContext} to use, cannot be null.
     * @param filePath file path to look for, can be null.
     * @return boolean whether if it was resolved correctly or not
     */
    private boolean resolveFileFromPath(@NotNull LoggerContext context, @Nullable String filePath) {
        Objects.requireNonNull(context, "received null LoggerContext");
        if (filePath != null) {
            File file = new File(filePath);

            // If the file didn't exist, poll to the next option (if any)
            if (!file.exists()) return false;
            if (!file.isFile())
                throw new IllegalStateException(format("Path [%s] was not a file", file.getAbsolutePath()));

            if (Files.isSymbolicLink(file.toPath())) {
                File original = file;
                try {
                    file = Files.readSymbolicLink(file.toPath()).toFile();
                } catch (IOException e) {
                    throw new RuntimeException(
                            format(
                                    "Received I/O exception when reading symbolic link for path [%s]",
                                    file.getAbsolutePath()),
                            e);
                }

                addInfo("Resolved path [%s] ~> [%s]".formatted(original, file));
            }

            try {
                applyProperties(context, new FileInputStream(file));
            } catch (FileNotFoundException e) {
                throw new RuntimeException(e);
            }

            return true;
        }

        return false;
    }

    /**
     * Loads the properties from the {@link InputStream} and applies all the key-value
     * pairs in the {@link LoggerContext} so that Joran can configure Logback successfully.
     *
     * @param context {@link LoggerContext} to apply properties to, cannot be null
     * @param stream {@link InputStream} to use when loading the properties, cannot be null.
     */
    private void applyProperties(LoggerContext context, InputStream stream) {
        try (stream) {
            final Properties properties = new Properties();
            properties.load(stream);

            for (Map.Entry<Object, Object> entry : properties.entrySet()) {
                final String key = (String) entry.getKey();
                final Object value = entry.getValue();

                context.putProperty(key, value.toString());
            }
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
