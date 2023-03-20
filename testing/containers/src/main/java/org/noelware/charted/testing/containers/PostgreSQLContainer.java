package org.noelware.charted.testing.containers;

import java.time.Duration;
import java.util.Map;
import kotlin.Unit;
import org.noelware.charted.configuration.kotlin.dsl.DatabaseConfig;
import org.testcontainers.containers.GenericContainer;
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
