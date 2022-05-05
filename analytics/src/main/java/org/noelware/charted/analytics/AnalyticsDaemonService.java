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

package org.noelware.charted.analytics;

import io.grpc.stub.StreamObserver;
import net.devh.boot.grpc.server.service.GrpcService;
import org.noelware.charted.analytics.protobufs.v1.AnalyticsGrpc;
import org.noelware.charted.analytics.protobufs.v1.ConnectionAckEvent;
import org.noelware.charted.analytics.protobufs.v1.ConnectionAckResponse;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/** This is the main implementation of the gRPC stub. */
@GrpcService
public class AnalyticsDaemonService extends AnalyticsGrpc.AnalyticsImplBase {
  private final Logger log = LoggerFactory.getLogger(AnalyticsDaemonService.class);

  @Override
  public void connectionAck(
      ConnectionAckEvent request, StreamObserver<ConnectionAckResponse> responseObserver) {
    log.debug("Received connection ack from request!");

    var resp = ConnectionAckResponse.newBuilder().setConnected(true).setPing(0.1f).build();

    responseObserver.onNext(resp);
    responseObserver.onCompleted();
  }
}
