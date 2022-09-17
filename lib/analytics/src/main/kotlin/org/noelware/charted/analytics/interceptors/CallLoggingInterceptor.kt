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

package org.noelware.charted.analytics.interceptors

import dev.floofy.utils.slf4j.logging
import io.grpc.*
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.analytics.asString
import org.noelware.charted.common.extensions.doFormatTime
import java.util.concurrent.atomic.AtomicLong

class CallLoggingInterceptor: ServerInterceptor {
    private val stopwatches: MutableMap<String, StopWatch> = mutableMapOf()
    private val _calls: AtomicLong = AtomicLong(0L)
    private val log by logging<CallLoggingInterceptor>()

    /**
     * Returns all the gRPC calls that have been called.
     */
    val calls: Long
        get() = _calls.get()

    override fun <ReqT: Any, RespT: Any> interceptCall(
        call: ServerCall<ReqT, RespT>,
        headers: Metadata,
        next: ServerCallHandler<ReqT, RespT>
    ): ServerCall.Listener<ReqT> {
        val listener = object: ForwardingServerCall.SimpleForwardingServerCall<ReqT, RespT>(call) {
            override fun sendMessage(message: RespT) {
                stopwatches[call.methodDescriptor.fullMethodName] = StopWatch.createStarted()
                super.sendMessage(message)
            }

            override fun close(status: Status, trailers: Metadata) {
                val stopwatch = stopwatches[call.methodDescriptor.fullMethodName]
                stopwatch?.stop()
                _calls.incrementAndGet()

                val contentType = headers.get(Metadata.Key.of("content-type", Metadata.ASCII_STRING_MARSHALLER))
                val userAgent = headers.get(Metadata.Key.of("user-agent", Metadata.ASCII_STRING_MARSHALLER))
                val message = buildString {
                    append("Finished gRPC request [${call.methodDescriptor.fullMethodName} (${call.methodDescriptor.serviceName ?: "<unknown service>"})]")
                    append("with status ${status.code.asString()}")

                    if (stopwatch != null) {
                        append(" [${stopwatch.doFormatTime()}")
                    }

                    if (userAgent != null) append(", $userAgent")
                    if (contentType != null) append(", $contentType")
                    append(']')
                }

                if (!status.isOk && status.code != Status.Code.CANCELLED) {
                    val exception = status.asException(headers)
                    log.error(message, exception.cause!!)
                } else {
                    log.info(message)
                }

                super.close(status, trailers)
            }
        }

        return createCallListener(listener, headers, next)
    }

    companion object {
        fun <ReqT: Any, RespT: Any> createCallListener(
            listener: ForwardingServerCall.SimpleForwardingServerCall<ReqT, RespT>,
            headers: Metadata,
            next: ServerCallHandler<ReqT, RespT>
        ): ForwardingServerCallListener.SimpleForwardingServerCallListener<ReqT> =
            object: ForwardingServerCallListener.SimpleForwardingServerCallListener<ReqT>(next.startCall(listener, headers)) {}
    }
}
