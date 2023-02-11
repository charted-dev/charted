/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.testing.containers;

import static java.lang.String.format;

import java.time.Duration;
import java.util.Map;
import kotlin.Unit;
import org.noelware.charted.RandomStringGenerator;
import org.noelware.charted.common.lazy.Lazy;
import org.noelware.charted.configuration.kotlin.dsl.search.MeilisearchConfig;
import org.slf4j.LoggerFactory;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.containers.output.Slf4jLogConsumer;
import org.testcontainers.containers.wait.strategy.Wait;
import org.testcontainers.utility.DockerImageName;

public class MeilisearchContainer extends GenericContainer<MeilisearchContainer> {
    private static final String MEILISEARCH_VERSION = "v0.30.5";
    private final Lazy<String> MASTER_KEY = Lazy.create(() -> RandomStringGenerator.generate(16));

    @SuppressWarnings("resource")
    public MeilisearchContainer() {
        super(DockerImageName.parse("getmeili/meilisearch").withTag(MEILISEARCH_VERSION));

        withLogConsumer(new Slf4jLogConsumer(LoggerFactory.getLogger("com.meilisearch.docker")));
        setWaitStrategy(Wait.forListeningPort().withStartupTimeout(Duration.ofMinutes(1)));
        withExposedPorts(7700);
        withEnv(Map.of(
                // Disable Meilisearch Analytics
                "MEILI_NO_ANALYTICS", "true",

                // Set the environment for Meilisearch to production
                "MEILI_ENV", "production",

                // Setting a master key is required if `MEILI_ENV` is in production
                // https://docs.meilisearch.com/learn/configuration/instance_options.html#environment
                "MEILI_MASTER_KEY", MASTER_KEY.get()));
    }

    public String getMasterKey() {
        return MASTER_KEY.get();
    }

    public MeilisearchConfig getConfiguration() {
        return MeilisearchConfig.invoke((builder) -> {
            builder.setEndpoint(format("http://%s:%d", getHost(), getMappedPort(7700)));
            builder.setMasterKey(MASTER_KEY.get());

            return Unit.INSTANCE;
        });
    }
}
