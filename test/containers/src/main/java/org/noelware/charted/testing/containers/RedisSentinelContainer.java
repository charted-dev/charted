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

import java.util.Map;
import java.util.Objects;
import kotlin.Unit;
import org.noelware.charted.RandomStringGenerator;
import org.noelware.charted.common.lazy.Lazy;
import org.noelware.charted.configuration.kotlin.dsl.RedisConfig;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.containers.Network;
import org.testcontainers.utility.DockerImageName;

public class RedisSentinelContainer extends GenericContainer<RedisSentinelContainer> {
    private static final Lazy<String> PASSWORD = Lazy.create(() -> RandomStringGenerator.generate(8));

    private final RedisContainer masterContainer;
    private final boolean authEnabled;
    private final Network network = Network.newNetwork();

    @SuppressWarnings("resource")
    public RedisSentinelContainer(boolean authEnabled) {
        super(DockerImageName.parse("bitnami/redis-sentinel").withTag(RedisContainer.VERSION));

        this.masterContainer = new RedisContainer(true, authEnabled, network);
        this.authEnabled = authEnabled;

        withEnv("REDIS_MASTER_HOST", masterContainer.getHost());
        if (authEnabled) {
            withEnv(Map.of(
                    "REDIS_SENTINEL_PASSWORD", PASSWORD.get(),
                    "REDIS_MASTER_PASSWORD", Objects.requireNonNull(masterContainer.getPassword())));
        }

        withExposedPorts(26379);
    }

    public RedisConfig getConfiguration() {
        return RedisConfig.invoke((builder) -> {
            builder.addSentinel(getHost(), getMappedPort(26379));
            builder.setMasterName("mymaster");

            if (authEnabled) {
                builder.setPassword(masterContainer.getPassword());
            }

            return Unit.INSTANCE;
        });
    }
}
