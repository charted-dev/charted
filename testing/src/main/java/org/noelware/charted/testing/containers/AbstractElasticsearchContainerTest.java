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
 *  Unless required by applicable law or agreed to in writing, software
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
import org.testcontainers.containers.wait.strategy.HttpWaitStrategy;
import org.testcontainers.elasticsearch.ElasticsearchContainer;
import org.testcontainers.utility.DockerImageName;

import java.util.List;

/**
 * This is the main test container suite to test the Elasticsearch library from <code>
 * :lib:elasticsearch</code> or any subproject that requires Elasticsearch to be tested.
 *
 * @since 14.07.22
 * @author Noel <cutie@floofy.dev>
 */
public class AbstractElasticsearchContainerTest {
    private static final SetOnceGetValue<ElasticsearchContainer> container =
            new SetOnceGetValue<>();

    private static final Logger log =
            LoggerFactory.getLogger(AbstractElasticsearchContainerTest.class);

    public static ElasticsearchContainer getContainer() {
        return container.getValue();
    }

    @BeforeClass
    public static void startElasticsearchContainer() {
        log.info("Starting Elasticsearch container...");

        var image =
                DockerImageName.parse("docker.elastic.co/elasticsearch/elasticsearch")
                        .withTag("8.3.0");

        var cont = new ElasticsearchContainer(image);
        cont.setEnv(List.of(
                "ES_JAVA_OPTS=-Xms1024m -Xmx4096m",
                "node.name=charted-es-0",
                "cluster.name=charted-es-cluster",
                "network.host=0.0.0.0"
        ));

        cont.setWaitStrategy(new HttpWaitStrategy().forPort(9200));

        container.setValue(cont);
        cont.start();
    }

    @AfterClass
    public static void destroyContainer() {
        if (!container.wasSet())
            throw new IllegalStateException(
                    "Can't call #destroyContainer if the container was never set.");

        var cont = container.getValue();
        cont.stop();
    }
}
