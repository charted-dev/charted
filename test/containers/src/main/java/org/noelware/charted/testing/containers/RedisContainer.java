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

import kotlin.Unit;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.RandomStringGenerator;
import org.noelware.charted.common.lazy.Lazy;
import org.noelware.charted.configuration.kotlin.dsl.RedisConfig;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.containers.Network;
import org.testcontainers.containers.wait.strategy.LogMessageWaitStrategy;
import org.testcontainers.utility.DockerImageName;

/**
 * Represents a generic Redis container that uses Bitnami's Redis distribution or
 * the official one located at <a href="https://hub.docker.com/_/redis">hub.docker.com/_/redis</a>.
 */
public class RedisContainer extends GenericContainer<RedisContainer> {
    private static final String OFFICIAL_REDIS = "redis";
    private static final String BITNAMI_REDIS = "bitnami/redis";
    public static final String VERSION = "7.0.7";
    private static final Lazy<String> PASSWORD = Lazy.create(() -> RandomStringGenerator.generate(8));

    private final boolean isUsingBitnami;
    private final boolean authEnabled;
    private final Network network;

    /**
     * Constructs a new {@link RedisContainer} with no network to be used
     * for other containers to interact with it, it uses the official Docker
     * image instead of Bitnami's distribution, and authentication is disabled.
     */
    public RedisContainer() {
        this(false, false, null);
    }

    /**
     * Constructs a new {@link RedisContainer} with no network to be used for other containers
     * to interact with this container.
     *
     * @param bitnami Whether if the container should use Bitnami's Redis distribution.
     */
    public RedisContainer(boolean bitnami) {
        this(bitnami, false, null);
    }

    /**
     * Constructs a new {@link RedisContainer} with an optional network.
     * @param bitnami Whether if the container should use Bitnami's Redis distribution.
     * @param network The {@link Network} to attach to when interacting with other containers, can be <code>null</code>.
     */
    @SuppressWarnings("resource")
    public RedisContainer(boolean bitnami, boolean auth, @Nullable Network network) {
        super(DockerImageName.parse(bitnami ? BITNAMI_REDIS : OFFICIAL_REDIS).withTag(VERSION));

        this.isUsingBitnami = bitnami;
        this.authEnabled = auth;
        this.network = network;

        if (authEnabled) {
            final String password = PASSWORD.get();
            withEnv("REDIS_PASSWORD", password);
        } else {
            if (bitnami) withEnv("ALLOW_EMPTY_PASSWORD", "yes");
        }

        withExposedPorts(6379);
        setWaitStrategy(new LogMessageWaitStrategy().withTimes(1).withRegEx("* Ready to accept connections"));
    }

    @Nullable
    public String getPassword() {
        return authEnabled ? PASSWORD.get() : null;
    }

    public RedisConfig getConfiguration() {
        return RedisConfig.invoke(builder -> {
            builder.setHost(getHost());
            builder.setPort(getMappedPort(6379));

            if (authEnabled) {
                builder.setPassword(getPassword());
            }

            return Unit.INSTANCE;
        });
    }
}
