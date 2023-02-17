/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.databases.clickhouse.tests;

import static java.lang.String.format;
import static org.junit.jupiter.api.Assertions.*;

import java.io.IOException;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.*;
import org.junit.jupiter.api.condition.DisabledOnOs;
import org.noelware.charted.common.lazy.Lazy;
import org.noelware.charted.databases.clickhouse.ClickHouseConnection;
import org.noelware.charted.databases.clickhouse.DefaultClickHouseConnection;
import org.noelware.charted.testing.containers.ClickHouseContainer;
import org.slf4j.LoggerFactory;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.containers.output.Slf4jLogConsumer;
import org.testcontainers.junit.jupiter.Container;
import org.testcontainers.junit.jupiter.Testcontainers;
import org.testcontainers.utility.DockerImageName;

@Testcontainers(disabledWithoutDocker = true)
public class ClickHouseConnectionTests {
    @Container
    private static final ClickHouseContainer container = new ClickHouseContainer();

    private static final Lazy<ClickHouseConnection> connection =
            Lazy.create(() -> new DefaultClickHouseConnection(container.getConfiguration()));

    @AfterAll
    public static void closeDown() throws IOException {
        final ClickHouseConnection conn = connection.get();
        conn.close();
    }

    @DisplayName("Can we connect to the ClickHouse cluster?")
    @Test
    public void test0() {
        final ClickHouseConnection conn = connection.get();
        assertFalse(conn.getClosed());
        assertEquals(0, conn.getCalls());

        // Connect to ClickHouse
        assertDoesNotThrow(conn::connect);
        assertEquals("22.12.1.1752", conn.getServerVersion());
        assertFalse(conn.getClosed());
        assertEquals(2, conn.getCalls());
    }

    @DisplayName("Can we list all the default database tables?")
    @Test
    public void test1() throws SQLException {
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
                fail(e);
                return null;
            }
        });

        assertEquals(3, conn.getCalls());
        assertEquals(4, databases.size());
        assertEquals(List.of("INFORMATION_SCHEMA", "default", "information_schema", "system"), databases);
    }

    @Test
    @SuppressWarnings("resource")
    @DisplayName("Can we run all pending migrations on this ClickHouse cluster?")
    @DisabledOnOs(architectures = {"aarch64", "arm64"})
    public void test2() {
        final ClickHouseConnection conn = connection.get();
        assertDoesNotThrow(() -> conn.create((connection) -> {
            try {
                final Statement stmt = connection.createStatement();
                stmt.execute("CREATE DATABASE charted;");

                return null;
            } catch (SQLException e) {
                fail(e);
                return null;
            }
        }));

        assertEquals(4, conn.getCalls());

        // We need to run the latest migrations binary (which is available at cr.noelware.cloud or ghcr.io)
        final GenericContainer<?> migrationsContainer = new GenericContainer<>(
                        DockerImageName.parse("ghcr.io/charted-dev/charted:nightly"))
                .withLogConsumer(new Slf4jLogConsumer(LoggerFactory.getLogger("org.noelware.charted.docker")))
                .withEnv(Map.of("CHARTED_ENABLE_WELCOME_PROMPT", "no"))
                .withCommand(
                        "charted",
                        "ch-migrations",
                        format("--hosts=%s", format("%s:%d", container.getHost(), container.getMappedPort(9000))),
                        "--table=migrations",
                        "--timeout=5s",
                        "--database=charted");

        migrationsContainer.start();
        migrationsContainer.close();

        assertDoesNotThrow(() -> {
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
                    return fail(e);
                }
            });

            assertEquals(5, conn.getCalls());
            assertEquals(5, databases.size());
            assertEquals(
                    List.of("INFORMATION_SCHEMA", "charted", "default", "information_schema", "system"), databases);
        });
    }
}
