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
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
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

        // The priority on how charted loads the Logback configuration file is:
        // environment variable > system property > classpath > safe default config
        final String environmentVariable = System.getenv(ENVIRONMENT_VARIABLE_KEY);
        if (environmentVariable != null) {
            addInfo("Pulling Logback configuration from path [%s] from environment variable [%s]"
                    .formatted(environmentVariable, ENVIRONMENT_VARIABLE_KEY));

            File file = new File(environmentVariable);
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

            final Properties properties = new Properties();
            try (final FileInputStream is = new FileInputStream(file)) {
                properties.load(is);
            } catch (IOException e) {
                throw new RuntimeException("Unable to create input stream from file:", e);
            }

            for (Map.Entry<Object, Object> entry : properties.entrySet()) {
                final String key = (String) entry.getKey();
                final Object value = entry.getValue();

                context.putProperty(key, value.toString());
            }

            return process0(context);
        }

        final String systemProperty = System.getProperty(SYSTEM_PROPERTY_KEY);
        if (systemProperty != null && !systemProperty.isBlank()) {
            addInfo("Pulling Logback configuration from path [%s] from property key [%s]"
                    .formatted(systemProperty, SYSTEM_PROPERTY_KEY));

            File file = new File(systemProperty);
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

            final Properties properties = new Properties();
            try (final FileInputStream is = new FileInputStream(file)) {
                properties.load(is);
            } catch (IOException e) {
                throw new RuntimeException("Unable to create input stream from file:", e);
            }

            for (Map.Entry<Object, Object> entry : properties.entrySet()) {
                final String key = (String) entry.getKey();
                final Object value = entry.getValue();

                context.putProperty(key, value.toString());
            }

            return process0(context);
        }

        final URL resourcePath = this.getClass().getResource("/config/logback.properties");
        if (resourcePath != null) {
            addInfo("Loading from resource path [%s]".formatted(resourcePath.toString()));

            try (final InputStream stream = this.getClass().getResourceAsStream("/config/logback.properties")) {
                final Properties properties = new Properties();
                properties.load(stream);

                for (Map.Entry<Object, Object> entry : properties.entrySet()) {
                    final String key = (String) entry.getKey();
                    final Object value = entry.getValue();

                    context.putProperty(key, value.toString());
                }
            } catch (IOException e) {
                throw new RuntimeException("Unable to create input stream from file:", e);
            }

            return process0(context);
        }

        // Represents the default configuration if the configurator can't:
        // - Load it from system property
        // - Load it from classpath
        // - Load it from the environment variable
        //
        // The log level will be set to INFO, which will be minimal output
        // instead of the verbose DEBUG/TRACE levels.
        //
        // JSON is set to be the default output, so it'll be easier to used
        // with other services that can check up on the logging system.
        addWarn("Unable to load from file or JAR resources, resulting to safe defaults...");
        context.putProperty("charted.log.level", "info");
        context.putProperty("charted.console.json", "true");
        context.putProperty("charted.appenders", "");

        return process0(context);
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
}
