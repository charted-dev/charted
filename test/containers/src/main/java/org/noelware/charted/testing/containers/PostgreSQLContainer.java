/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

import java.time.Duration;
import java.util.Map;
import kotlin.Unit;
import org.noelware.charted.configuration.kotlin.dsl.DatabaseConfig;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.containers.wait.strategy.LogMessageWaitStrategy;
import org.testcontainers.containers.wait.strategy.Wait;
import org.testcontainers.utility.DockerImageName;

public class PostgreSQLContainer extends GenericContainer<PostgreSQLContainer> {
    private static final String ALPINE_VERSION = "15-alpine3.16";

    @SuppressWarnings("resource")
    public PostgreSQLContainer() {
        super(DockerImageName.parse("postgres").withTag(ALPINE_VERSION));

        withExposedPorts(5432);
        withEnv(Map.of(
                "POSTGRES_USER", "charted",
                "POSTGRES_PASSWORD", "charted",
                "POSTGRES_DB", "charted"));

        setWaitStrategy(Wait.forListeningPort().withStartupTimeout(Duration.ofMinutes(1)));
    }

    public DatabaseConfig getConfiguration() {
        return DatabaseConfig.invoke((builder) -> {
            builder.setUsername("charted");
            builder.setPassword("charted");
            builder.setDatabase("charted");
            builder.setHost(getHost());
            builder.setPort(getMappedPort(5432));

            return Unit.INSTANCE;
        });
    }
}
