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

package org.noelware.charted.modules.tracing.opentelemetry;

import io.opentelemetry.api.OpenTelemetry;
import java.io.File;
import java.io.IOException;
import java.util.Objects;
import java.util.concurrent.atomic.AtomicBoolean;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.configuration.kotlin.dsl.tracing.TracingConfig;
import org.noelware.charted.modules.tracing.Tracer;
import org.noelware.charted.modules.tracing.Transaction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class OpenTelemetryTracer implements Tracer {
    private final AtomicBoolean initialized = new AtomicBoolean(false);
    private final Logger LOG = LoggerFactory.getLogger(getClass());

    private final TracingConfig.OpenTelemetry settings;
    private OpenTelemetry openTelemetry;

    public OpenTelemetryTracer(TracingConfig.OpenTelemetry settings) {
        this.settings = Objects.requireNonNull(settings, "OpenTelemetry settings shouldn't be null.");
    }

    @Override
    public @NotNull Transaction createTransaction(@NotNull String name, @Nullable String operation) {
        throw new RuntimeException("#createTransaction is not available at the moment.");
    }

    @Override
    public @NotNull Transaction createTransaction(@NotNull String name) {
        throw new RuntimeException("#createTransaction is not available at the moment.");
    }

    @Override
    public void init() {
        if (!initialized.compareAndSet(false, true)) return;

        LOG.info("Now initializing OpenTelemetry tracing...");

        // First, we need to create a temporary file which the OpenTelemetry
        // Java Agent will initialize the options from the settings provided.
        File tempPropertiesFile;
        try {
            tempPropertiesFile = File.createTempFile("opentelemetry-javaagent", ".tmp");
        } catch (IOException ioe) {
            LOG.error("Unable to create temp properties file for OpenTelemetry", ioe);
            return;
        }

        LOG.debug("Created temporary properties file [{}]", tempPropertiesFile);
    }

    @Override
    public void close() {}
}
