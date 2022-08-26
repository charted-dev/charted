/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
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

package org.noelware.charted.tests.cassandra;

import static org.junit.jupiter.api.Assertions.*;

import java.util.List;
import org.junit.Test;
import org.noelware.charted.common.Provider;
import org.noelware.charted.common.data.CassandraConfig;
import org.noelware.charted.common.lazy.Lazy;
import org.noelware.charted.common.lazy.LazyImpl;
import org.noelware.charted.database.cassandra.CassandraConnection;
import org.testcontainers.containers.CassandraContainer;
import org.testcontainers.junit.jupiter.Container;
import org.testcontainers.junit.jupiter.Testcontainers;
import org.testcontainers.utility.DockerImageName;

@Testcontainers(disabledWithoutDocker = true)
public class CassandraTests {
    @Container
    private final CassandraContainer<?> container =
            new CassandraContainer<>(DockerImageName.parse("cassandra").withTag("4.0"));

    private final Lazy<CassandraConfig> configuration = new LazyImpl<>(
            () -> new CassandraConfig(null, null, "", List.of(container.getHost()), container.getMappedPort(9042)));

    private void createConnection(Provider.Param1<CassandraConnection> provider) {
        if (!container.isRunning()) container.start();
        try (final var connection = new CassandraConnection(configuration.get())) {
            assertFalse(connection.getClosed());
            assertThrows(IllegalStateException.class, connection::getServerVersion);

            connection.connect();
            provider.get(connection);
        }
    }

    @Test
    public void assertIsRunning() {
        createConnection((connection) -> assertEquals("4.0.6", connection.getServerVersion()));
    }

    @Test
    public void testCrudOperations() {
        createConnection((connection) -> {
            assertDoesNotThrow(() -> {
                connection.sql(
                        "CREATE KEYSPACE testdb WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 2};");
            });

            var keyspaces =
                    connection.sql("select * from system_schema.keyspaces").all();

            assertNotEquals(0, keyspaces.size());

            var notSystemKs = keyspaces.stream()
                    .filter(it -> !it.getString("keyspace_name").startsWith("system"))
                    .count();

            assertEquals(1, notSystemKs);
        });
    }
}
