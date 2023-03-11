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

package org.noelware.charted.modules.emails

import dev.floofy.utils.kotlin.threading.createThreadFactory
import dev.floofy.utils.slf4j.logging
import io.grpc.ManagedChannel
import io.grpc.ManagedChannelBuilder
import kotlinx.atomicfu.atomic
import org.noelware.charted.ChartedInfo
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.emails.protobufs.v1.*
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit

class DefaultEmailService(config: Config): EmailService {
    private val channel: ManagedChannel = ManagedChannelBuilder.forTarget(config.emailsGrpcEndpoint!!).apply {
        // Use a single thread for this since we wouldn't really use this method much unless
        // we need HA, then we might bump this up.
        executor(Executors.newSingleThreadExecutor(createThreadFactory("EmailsGrpcExecutor")))
        userAgent("Noelware/charted-server (+https://github.com/charted-dev/charted; v${ChartedInfo.version}+${ChartedInfo.commitHash})")
        usePlaintext() // The service doesn't implement TLS, so we will have to use plaintext for now
    }.build()

    private val grpcClient: EmailsGrpcKt.EmailsCoroutineStub = EmailsGrpcKt.EmailsCoroutineStub(channel)
    private val _closed = atomic(false)
    private val log by logging<DefaultEmailService>()

    override suspend fun ping(): Boolean = grpcClient.ping(PingRequest.getDefaultInstance()).pong
    override suspend fun sendEmail(request: SendEmailRequest): SendEmailResponse = grpcClient.send(request)
    override fun close() {
        if (_closed.compareAndSet(expect = false, update = true)) {
            log.info("Closing gRPC client...")
            channel.shutdownNow()

            try {
                channel.awaitTermination(10, TimeUnit.SECONDS)
                log.info("Client connection is closed.")
            } catch (e: InterruptedException) {
                log.warn("Unable to close down client connection due to an interruption, it was not closed cleanly", e)
            }
        }
    }
}
