package org.noelware.charted.testing.containers;

import java.time.Duration;
import kotlin.Unit;
import org.jetbrains.annotations.Nullable;
import org.noelware.charted.common.lazy.Lazy;
import org.noelware.charted.configuration.kotlin.dsl.RedisConfig;
import org.noelware.charted.utils.RandomStringGeneratorKt;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.containers.Network;
import org.testcontainers.containers.wait.strategy.Wait;
import org.testcontainers.utility.DockerImageName;

/**
 * Represents a generic Redis container that uses Bitnami's Redis distribution or
 * the official one located at <a href="https://hub.docker.com/_/redis">hub.docker.com/_/redis</a>.
 */
public class RedisContainer extends GenericContainer<RedisContainer> {
    private static final String OFFICIAL_REDIS = "redis";
    private static final String BITNAMI_REDIS = "bitnami/redis";
    public static final String VERSION = "7.0.8";
    private static final Lazy<String> PASSWORD = Lazy.create(() -> RandomStringGeneratorKt.randomString(16));

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
        setWaitStrategy(Wait.forListeningPort().withStartupTimeout(Duration.ofMinutes(1)));
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
