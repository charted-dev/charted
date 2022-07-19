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

package org.noelware.charted.testing.containers;

import org.junit.AfterClass;
import org.junit.BeforeClass;
import org.noelware.charted.common.SetOnceGetValue;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.testcontainers.containers.CassandraContainer;
import org.testcontainers.containers.wait.CassandraQueryWaitStrategy;
import org.testcontainers.utility.DockerImageName;

/**
 * @since 18.07.22
 * @author Noel <cutie@floofy.dev>
 */
public class AbstractCassandraContainerTests {
    private static final SetOnceGetValue<CassandraContainer<?>> container = new SetOnceGetValue<>();
    private static final Logger log =
            LoggerFactory.getLogger(AbstractCassandraContainerTests.class);

    public static CassandraContainer<?> getContainer() {
        return container.getValue();
    }

    @BeforeClass
    public static void start() {
        log.info("Starting Cassandra container...");

        var image = DockerImageName.parse("cassandra").withTag("4.0.0");
        var cont = new CassandraContainer<>(image);
        cont.setWaitStrategy(new CassandraQueryWaitStrategy());
        cont.start();

        container.setValue(cont);
    }

    @AfterClass
    public static void destroy() {
        if (!container.wasSet())
            throw new IllegalStateException(
                    "Can't call #destroyContainer if the container was never set.");

        var cont = container.getValue();
        cont.stop();
    }
}
