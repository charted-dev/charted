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

package org.noelware.charted.logback;

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

public class LogbackConfigurator extends ContextAwareBase implements Configurator {
    private static final String SYSTEM_PROPERTY_KEY = "org.noelware.charted.logback.config";
    private static final String ENVIRONMENT_VARIABLE_KEY = "CHARTED_LOGBACK_CONFIG_PATH";

    @Override
    public ExecutionStatus configure(LoggerContext context) {
        // Set the logger context to the one we have right now.
        setContext(context);

        // priority: environment variable > system property > classpath > default
        final String envVar = System.getenv(ENVIRONMENT_VARIABLE_KEY);
        if (resolveFileFromPath(context, envVar)) return process0(context);

        final String sysProp = System.getProperty(SYSTEM_PROPERTY_KEY);
        if (resolveFileFromPath(context, sysProp)) return process0(context);

        // Check if we can find it in the ./config directory, since the Docker image
        // and archives provide a default one
        final File configDir = new File("./config");
        if (configDir.exists()) {
            if (resolveFileFromPath(context, new File(configDir, "logback.properties").getAbsolutePath())) {
                return process0(context);
            }
        }

        // If we can resolve it in the root directory of the project
        if (resolveFileFromPath(context, "./logback.properties")) return process0(context);
        final URL resourcePath = getClass().getResource("/config/logback.properties");
        if (resourcePath != null) {
            final InputStream is = getClass().getResourceAsStream("/config/logback.properties");
            applyProperties(context, is);

            return process0(context);
        }

        context.putProperty("charted.log.level", "INFO");
        return process0(context);
    }

    private boolean resolveFileFromPath(LoggerContext context, String filePath) {
        if (filePath != null) {
            File file = new File(filePath);
            if (!file.exists()) {
                addError("Path [%s] doesn't exist".formatted(file.toString()));
                throw new IllegalStateException(
                        "Path [%s] doesn't exist. Make sure it exists!".formatted(file.toString()));
            }

            if (!file.isFile()) {
                addError("Path [%s] was not a file".formatted(file.toString()));
                throw new IllegalStateException(
                        "Path [%s] was not a file. Make sure it is a file!".formatted(file.toString()));
            }

            if (Files.isSymbolicLink(file.toPath())) {
                File original = file;
                try {
                    file = Files.readSymbolicLink(file.toPath()).toFile();
                } catch (IOException e) {
                    throw new RuntimeException(
                            "Unable to read symbolic link from path [%s]".formatted(file.toString()), e);
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

    private Configurator.ExecutionStatus process0(LoggerContext context) {
        try {
            final JoranConfigurator configurator = new JoranConfigurator();
            configurator.setContext(context);
            configurator.doConfigure(Objects.requireNonNull(getClass().getResource("/config/logback.xml")));

            return ExecutionStatus.DO_NOT_INVOKE_NEXT_IF_ANY;
        } catch (JoranException e) {
            throw new RuntimeException("Unable to configure Joran:", e);
        }
    }

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
