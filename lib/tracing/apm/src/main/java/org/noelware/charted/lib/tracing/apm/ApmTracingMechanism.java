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

package org.noelware.charted.lib.tracing.apm;

import co.elastic.apm.attach.ElasticApmAttacher;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.nio.file.Files;
import java.util.HashMap;
import java.util.Map;
import java.util.Properties;
import java.util.stream.Collectors;
import org.jetbrains.annotations.NotNull;
import org.noelware.charted.lib.tracing.TraceContext;
import org.noelware.charted.lib.tracing.TracerMechanism;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ApmTracingMechanism implements TracerMechanism {
    private final Logger log = LoggerFactory.getLogger(ApmTracingMechanism.class);

    @NotNull
    @Override
    public TraceContext createContext(@NotNull String name, @NotNull Map<String, String> attributes) {
        return new ApmTraceContext(name, attributes);
    }

    @Override
    public void init() {
        log.info("Enabling Elastic APM...");

        var apmProperties = System.getProperty("org.noelware.charted.apm.properties", "./config/apm.properties");

        log.info("Loading APM properties if file exists [{}]", apmProperties);
        var file = new File(apmProperties);
        if (!file.exists()) {
            log.warn("File [{}] doesn't exist, not enabling Elastic APM!", file);
            return;
        }

        if (Files.isSymbolicLink(file.toPath())) {
            try {
                var resolved = Files.readSymbolicLink(file.toPath());
                log.info("Path [{}] was a symbolic link that resolved to {}", file.toPath(), resolved);

                file = resolved.toFile();
            } catch (IOException e) {
                log.error("Not loading APM due to conflicts with symbolic link [{}] due to {}", file.toPath(), e);
                return;
            }
        }

        var properties = new Properties();
        try (var inputStream = new FileInputStream(file)) {
            properties.load(inputStream);

            var config = properties.entrySet().stream()
                    .collect(Collectors.toMap(
                            e -> String.valueOf(e.getKey()),
                            e -> String.valueOf(e.getValue()),
                            (prev, next) -> next,
                            HashMap::new));

            ElasticApmAttacher.attach(config);
        } catch (IOException e) {
            log.error("Received IO exception when fetching input stream, APM will not be enabled.", e);
        }
    }
}
