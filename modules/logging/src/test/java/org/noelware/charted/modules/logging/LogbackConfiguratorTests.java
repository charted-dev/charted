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
import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import ch.qos.logback.classic.Level;
import ch.qos.logback.classic.Logger;
import ch.qos.logback.classic.LoggerContext;
import java.io.File;
import java.nio.file.Files;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.junit.jupiter.api.io.TempDir;
import uk.org.webcompere.systemstubs.environment.EnvironmentVariables;
import uk.org.webcompere.systemstubs.jupiter.SystemStubsExtension;
import uk.org.webcompere.systemstubs.properties.SystemProperties;

@ExtendWith(SystemStubsExtension.class)
public class LogbackConfiguratorTests {
    private LoggerContext loggerContext;

    @TempDir
    private File tmpDir;

    @BeforeEach
    public void beforeEach() {
        assert loggerContext == null : "#afterEach was not called to destroy the LoggerContext";

        loggerContext = new LoggerContext();
        loggerContext.reset();
    }

    @AfterEach
    public void afterEach() {
        assert loggerContext != null : "#beforeEach was not called to create the LoggerContext";

        loggerContext = null;
    }

    @DisplayName("Should use the default configuration when applied")
    @Test
    public void test0() {
        final LogbackConfigurator configurator = new LogbackConfigurator();
        configurator.configure(loggerContext);

        assertThat(loggerContext.getProperty("charted.log.level")).isNotNull().isEqualTo("INFO");

        assertThat(loggerContext.getProperty("charted.console.json"))
                .isNotNull()
                .isEqualTo("yes");
    }

    @DisplayName("Should use the properties file from `-Dorg.noelware.charted.logback.config`")
    @Test
    public void test1() throws Exception {
        new SystemProperties()
                .set("org.noelware.charted.logback.config", format("%s/dir", tmpDir))
                .execute(() -> {
                    assertDoesNotThrow(() -> {
                        final File newTmpDir = new File(tmpDir, "dir");
                        assertTrue(newTmpDir.mkdirs(), "unable to create directory");
                    });

                    // now, we attempt to load it
                    final LogbackConfigurator configurator = new LogbackConfigurator();
                    assertThatExceptionOfType(IllegalStateException.class)
                            .isThrownBy(() -> configurator.configure(loggerContext))
                            .withMessageContaining("was not a file");
                });

        loggerContext.reset();
        new SystemProperties()
                .set("org.noelware.charted.logback.config", format("%s/logback.properties", tmpDir))
                .execute(() -> {
                    assertDoesNotThrow(() -> {
                        final File logbackProps = new File(tmpDir, "logback.properties");
                        Files.writeString(
                                logbackProps.toPath(),
                                """
                    charted.log.level=TRACE
                    charted.appenders=logstash,sentry
                    """);
                    });

                    // now, we attempt to load it
                    final LogbackConfigurator configurator = new LogbackConfigurator();
                    assertDoesNotThrow(() -> configurator.configure(loggerContext));

                    assertThat(loggerContext.getProperty("charted.log.level"))
                            .isNotNull()
                            .isEqualTo(Level.TRACE.toString());

                    assertThat(loggerContext.getProperty("charted.appenders"))
                            .isNotNull()
                            .isEqualTo("logstash,sentry");

                    final Logger logger = loggerContext.getLogger("woof.net");
                    assertThat(logger).isNotNull();
                    assertThat(logger.getName()).isNotNull().isEqualTo("woof.net");
                });
    }

    @DisplayName("Should use the properties file from `CHARTED_LOGBACK_CONFIG_FILE`")
    @Test
    public void test2() throws Exception {
        new EnvironmentVariables()
                .set("CHARTED_LOGBACK_CONFIG_FILE", format("%s/dir", tmpDir))
                .execute(() -> {
                    assertDoesNotThrow(() -> {
                        final File newTmpDir = new File(tmpDir, "dir");
                        assertThat(newTmpDir.mkdirs()).isTrue();
                    });

                    // now, we attempt to load it
                    final LogbackConfigurator configurator = new LogbackConfigurator();
                    assertThatExceptionOfType(IllegalStateException.class)
                            .isThrownBy(() -> configurator.configure(loggerContext))
                            .withMessageContaining("was not a file");
                });

        loggerContext.reset();
        new EnvironmentVariables()
                .set("CHARTED_LOGBACK_CONFIG_FILE", format("%s/logback.properties", tmpDir))
                .execute(() -> {
                    assertDoesNotThrow(() -> {
                        final File logbackProps = new File(tmpDir, "logback.properties");
                        Files.writeString(
                                logbackProps.toPath(),
                                """
                    charted.log.level=TRACE
                    charted.appenders=logstash,sentry
                    """);
                    });

                    // now, we attempt to load it
                    final LogbackConfigurator configurator = new LogbackConfigurator();
                    assertDoesNotThrow(() -> configurator.configure(loggerContext));

                    assertThat(loggerContext.getProperty("charted.log.level"))
                            .isNotNull()
                            .isEqualTo(Level.TRACE.toString());

                    assertThat(loggerContext.getProperty("charted.appenders"))
                            .isNotNull()
                            .isEqualTo("logstash,sentry");

                    final Logger logger = loggerContext.getLogger("woof.net");
                    assertThat(logger).isNotNull();
                    assertThat(logger.getName()).isEqualTo("woof.net");
                });
    }
}
