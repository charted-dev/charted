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
import org.testcontainers.containers.ClickHouseContainer;
import org.testcontainers.containers.wait.strategy.HttpWaitStrategy;
import org.testcontainers.utility.DockerImageName;

/**
 * This is the main test container suite to test ClickHouse and the server features that it needs.
 *
 * @since 14.07.22
 * @author Noel <cutie@floofy.dev>
 */
public class AbstractClickHouseContainerTest {
    private static final SetOnceGetValue<ClickHouseContainer> container = new SetOnceGetValue<>();
    private static final Logger log =
            LoggerFactory.getLogger(AbstractClickHouseContainerTest.class);

    public static ClickHouseContainer getContainer() {
        return container.getValue();
    }

    public static SetOnceGetValue<ClickHouseContainer> getContainerState() {
        return container;
    }

    @BeforeClass
    public static void startContainer() {
        log.info("Starting ClickHouse container...");

        var image =
                DockerImageName.parse("clickhouse/clickhouse-server")
                        .withTag("22.6.2.12-alpine")
                        .asCompatibleSubstituteFor("yandex/clickhouse-server");

        var cont = new ClickHouseContainer(image);
        cont.setWaitStrategy(new HttpWaitStrategy().forPort(8123));
        container.setValue(cont);

        cont.start();
    }

    @AfterClass
    public static void destroyContainer() {
        if (!container.wasSet())
            throw new IllegalStateException(
                    "Can't call #destroyContainer if the container was never set.");

        log.warn("Closing container...");
        var cont = container.getValue();
        cont.stop();
    }
}
