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
import kotlin.Unit;
import org.noelware.charted.configuration.kotlin.dsl.ClickHouseConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.containers.output.Slf4jLogConsumer;
import org.testcontainers.containers.wait.strategy.HttpWaitStrategy;
import org.testcontainers.utility.DockerImageName;

public class ClickHouseContainer extends GenericContainer<ClickHouseContainer> {
    private static final String CLICKHOUSE_IMAGE_VERSION = "22.12.1.1752-alpine";

    public ClickHouseContainer() {
        super(DockerImageName.parse("clickhouse/clickhouse-server").withTag(CLICKHOUSE_IMAGE_VERSION));

        Logger LOG = LoggerFactory.getLogger(ClickHouseContainer.class);
        LOG.info("Using Docker image [clickhouse/clickhouse-server:{}]", CLICKHOUSE_IMAGE_VERSION);

        addExposedPorts(8123, 9000);
        withLogConsumer(new Slf4jLogConsumer(LOG));
        waitStrategy = new HttpWaitStrategy()
                .forPort(8123)
                .forStatusCode(200)
                .forResponsePredicate("Ok."::equals)
                .withStartupTimeout(Duration.ofMinutes(1));
    }

    /**
     * Returns the {@link ClickHouseConfig} object from the container's information.
     */
    public ClickHouseConfig getConfiguration() {
        return ClickHouseConfig.invoke((builder) -> {
            builder.setHost(getHost());
            builder.setPort(getMappedPort(8123));
            builder.setDatabase("");
            builder.setPassword(null);
            builder.setUsername(null);

            return Unit.INSTANCE;
        });
    }
}
