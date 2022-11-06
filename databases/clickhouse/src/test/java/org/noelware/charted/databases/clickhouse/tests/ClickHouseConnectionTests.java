package org.noelware.charted.databases.clickhouse.tests;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;
import org.noelware.charted.common.lazy.Lazy;
import org.noelware.charted.configuration.kotlin.dsl.ClickHouseConfig;
import org.noelware.charted.databases.clickhouse.ClickHouseConnection;
import org.noelware.charted.databases.clickhouse.DefaultClickHouseConnection;
import org.testcontainers.containers.ClickHouseContainer;
import org.testcontainers.junit.jupiter.Container;
import org.testcontainers.junit.jupiter.Testcontainers;
import org.testcontainers.utility.DockerImageName;

import java.io.IOException;

@Testcontainers(disabledWithoutDocker = true)
public class ClickHouseConnectionTests {
    @Container
    private final ClickHouseContainer container = new ClickHouseContainer(DockerImageName.parse("clickhouse/clickhouse-server").withTag("22.6.9.11-alpine"));
    private final Lazy<ClickHouseConnection> connection = Lazy.create(() -> {
        final ClickHouseConfig config = new ClickHouseConfig(
                "",
                null,
                null,
                container.getHost(),
                container.getMappedPort(8123)
        );

        return new DefaultClickHouseConnection(config);
    });

    @Test
    public void test_canWeConnectToClickHouse() throws IOException {
        final ClickHouseConnection conn = connection.get();
        assertFalse(conn.getClosed());
        assertEquals(0, conn.getCalls());

        // Connect to ClickHouse
        assertDoesNotThrow(conn::connect);
        conn.close();
    }
}
