/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

import java.time.Duration;
import java.util.Map;
import java.util.concurrent.TimeUnit;
import kotlin.Unit;
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthenticationStrategy;
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.ElasticsearchConfig;
import org.slf4j.LoggerFactory;
import org.testcontainers.containers.GenericContainer;
import org.testcontainers.containers.Network;
import org.testcontainers.containers.output.Slf4jLogConsumer;
import org.testcontainers.containers.wait.strategy.HttpWaitStrategy;
import org.testcontainers.utility.DockerImageName;
import org.testcontainers.utility.MountableFile;

public class ElasticsearchContainer extends GenericContainer<ElasticsearchContainer> {
    private static final String ELASTICSEARCH_IMAGE_VERSION = "8.6.2";
    private final boolean isSslEnabled;
    private final Network NETWORK = Network.newNetwork();

    public ElasticsearchContainer() {
        this(false);
    }

    @SuppressWarnings("resource")
    public ElasticsearchContainer(boolean ssl) {
        super(DockerImageName.parse("docker.elastic.co/elasticsearch/elasticsearch")
                .withTag(ELASTICSEARCH_IMAGE_VERSION));

        this.isSslEnabled = ssl;
        withLogConsumer(new Slf4jLogConsumer(LoggerFactory.getLogger(ElasticsearchContainer.class)));
        withExposedPorts(9200);
        withNetwork(NETWORK);
        withEnv(Map.of(
                "discovery.type", "single-node",
                "ES_JAVA_OPTS", "-Xms1024m -Xmx2048m -Dfile.encoding=UTF-8",
                "ELASTIC_PASSWORD", "changeme"));

        withCopyFileToContainer(
                MountableFile.forClasspathResource("/elasticsearch/elasticsearch.yml"),
                "/usr/share/elasticsearch/config/elasticsearch.yml");

        setWaitStrategy(new HttpWaitStrategy()
                .forPort(9200)
                .forPath("/")
                .forStatusCode(200)
                .allowInsecure()
                .withBasicCredentials("elastic", "changeme")
                .withReadTimeout(Duration.of(2, TimeUnit.MINUTES.toChronoUnit())));
    }

    public Network getUsedNetwork() {
        return NETWORK;
    }

    public ElasticsearchConfig getConfiguration() {
        return ElasticsearchConfig.invoke((builder) -> {
            builder.node(getHost(), getMappedPort(9200));
            builder.auth(new AuthenticationStrategy.Basic("elastic", "changeme"));

            if (isSslEnabled) {
                builder.ssl(ssl -> {
                    ssl.setValidateHostnames(false);
                    return Unit.INSTANCE;
                });
            }

            return Unit.INSTANCE;
        });
    }
}
