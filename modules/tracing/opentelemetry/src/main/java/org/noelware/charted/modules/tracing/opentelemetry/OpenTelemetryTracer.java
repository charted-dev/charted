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

import io.opentelemetry.api.common.Attributes;
import io.opentelemetry.sdk.resources.Resource;
import io.opentelemetry.semconv.resource.attributes.ResourceAttributes;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.configuration.kotlin.dsl.tracing.TracingConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class OpenTelemetryTracer {
    private final TracingConfig.OpenTelemetry settings;
    private final Logger LOG = LoggerFactory.getLogger(getClass());

    public OpenTelemetryTracer(@NotNull TracingConfig.OpenTelemetry settings) {
        this.settings = settings;
    }

    public void init() {
        LOG.info("Creating manual instrumentation");
        final Resource resource = Resource.getDefault()
                .merge(Resource.create(Attributes.of(ResourceAttributes.SERVICE_NAME, "charted-server")));
    }

    File createTempPropertiesFile(@Nullable File tempdir) throws IOException {
        final File tmpDir = tempdir == null ? new File(System.getProperty("java.io.tmpdir")) : tempdir;
        final File tempProps = Files.createTempFile(tmpDir.toPath(), "charted-opentelemetry-properties", ".tmp")
                .toFile();

        return tempProps;
    }
}
