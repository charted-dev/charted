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

package org.noelware.charted.modules.analytics;

import io.grpc.protobuf.services.ProtoReflectionService;
import java.io.Closeable;
import java.io.IOException;
import java.time.Instant;
import org.noelware.analytics.jvm.server.AnalyticsServer;
import org.noelware.analytics.jvm.server.AnalyticsServerBuilder;
import org.noelware.analytics.jvm.server.extensions.Extension;
import org.noelware.analytics.protobufs.v1.BuildFlavour;
import org.noelware.charted.ChartedInfo;
import org.noelware.charted.common.SetOnce;
import org.noelware.charted.configuration.kotlin.dsl.NoelwareAnalyticsConfig;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * Represents a daemon process that runs the analytics server in the background outside the
 * main API server. If you wish to just run the Analytics Server for only <code>charted-server</code>,
 * you can run the <code>charted analytics start</code> subcommand.
 */
public class AnalyticsDaemon implements Closeable {
    private final SetOnce<AnalyticsServer> server = new SetOnce<>();
    private final Logger LOG = LoggerFactory.getLogger(AnalyticsDaemon.class);
    private final NoelwareAnalyticsConfig config;
    private final Extension<?> extension;

    /**
     * Constructs a new {@link AnalyticsDaemon}.
     * @param config The configuration object for configuring the daemon
     */
    public AnalyticsDaemon(NoelwareAnalyticsConfig config, Extension<?> extension) {
        this.extension = extension;
        this.config = config;
    }

    public void start() throws IOException {
        if (server.wasSet()) {
            LOG.warn("Analytics daemon is already running! Not doing anything...");
            return;
        }

        LOG.info("Starting the protocol server with host [0.0.0.0:{}]", config.getPort());
        final AnalyticsServer server_ = new AnalyticsServerBuilder(config.getPort())
                .withServiceToken(config.getServiceToken())
                .withExtension(extension)
                .withServerMetadata(metadata -> {
                    final ChartedInfo info = ChartedInfo.INSTANCE;
                    final BuildFlavour flavour =
                            switch (info.getDistribution()) {
                                case UNKNOWN, AUR -> BuildFlavour.UNRECOGNIZED;
                                case DOCKER -> BuildFlavour.DOCKER;
                                case RPM -> BuildFlavour.RPM;
                                case DEB -> BuildFlavour.DEB;
                                case GIT -> BuildFlavour.GIT;
                            };

                    metadata.setDistributionType(flavour);
                    metadata.setProductName("charted-server");
                    metadata.setCommitHash(info.getCommitHash());
                    metadata.setBuildDate(Instant.parse(info.getBuildDate()));
                    metadata.setVersion(info.getVersion());
                    metadata.setVendor("Noelware");
                })
                .withServerBuilder(builder -> {
                    builder.addService(ProtoReflectionService.newInstance());
                })
                .build();

        server.setValue(server_);
        server_.start();
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * <p> As noted in {@link AutoCloseable#close()}, cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * <em>mark</em> the {@code Closeable} as closed, prior to throwing
     * the {@code IOException}.
     *
     * @throws IOException if an I/O error occurs
     */
    @Override
    public void close() throws IOException {
        if (!server.wasSet()) return;

        server.getValue().close();
    }
}
