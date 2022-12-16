package org.noelware.charted.databases.clickhouse.tests;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;
import org.noelware.charted.common.lazy.Lazy;
import org.noelware.charted.configuration.kotlin.dsl.ClickHouseConfig;
import org.noelware.charted.databases.clickhouse.ClickHouseConnection;
import org.noelware.charted.databases.clickhouse.DefaultClickHouseConnection;
import org.slf4j.LoggerFactory;
import org.testcontainers.containers.ClickHouseContainer;
import org.testcontainers.containers.output.Slf4jLogConsumer;
import org.testcontainers.junit.jupiter.Container;
import org.testcontainers.junit.jupiter.Testcontainers;
import org.testcontainers.utility.DockerImageName;

import java.io.IOException;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.List;

@Testcontainers(disabledWithoutDocker = true)
public class ClickHouseConnectionTests {
    @Container
    private static final ClickHouseContainer container = new ClickHouseContainer(DockerImageName.parse("clickhouse/clickhouse-server").withTag("22.6.9.11-alpine")).withLogConsumer(new Slf4jLogConsumer(LoggerFactory.getLogger("com.clickhouse.docker")));
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
    public void test_canWeConnectToClickHouse() {
        final ClickHouseConnection conn = connection.get();
        assertFalse(conn.getClosed());
        assertEquals(0, conn.getCalls());

        // Connect to ClickHouse
        assertDoesNotThrow(conn::connect);
        assertEquals("22.6.9.11", conn.getServerVersion());
        assertFalse(conn.getClosed());
        assertEquals(2, conn.getCalls());
    }

    @Test
    public void test_canWeListTables() throws SQLException {
        final ClickHouseConnection conn = connection.get();
        final ArrayList<String> databases = new ArrayList<>();

        conn.create((connection) -> {
            try {
                final Statement stmt = connection.createStatement();
                final ResultSet rs = stmt.executeQuery("SELECT * FROM system.databases;");
                while (rs.next()) {
                    final String name = rs.getString("name");
                    databases.add(name);
                }

                return null;
            } catch (SQLException e) {
                throw new RuntimeException(e);
            }
        });

        assertEquals(3, conn.getCalls());
        assertEquals(4, databases.size());
        assertEquals(List.of("INFORMATION_SCHEMA", "default", "information_schema", "system"), databases);
    }
}
