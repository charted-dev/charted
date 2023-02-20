package org.noelware.charted.modules.email

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
    private val channel: ManagedChannel = ManagedChannelBuilder.forTarget(config.emailsEndpoint!!).apply {
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
