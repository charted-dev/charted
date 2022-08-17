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
import org.noelware.charted.common.extensions.doFormatTime
import java.util.concurrent.atomic.AtomicLong

class GrpcLoggingInterceptor: ServerInterceptor {
    private val stopwatches = mutableMapOf<String, StopWatch>()
    private val log by logging<GrpcLoggingInterceptor>()
    private val _calls = AtomicLong(0L)
    private val id = AtomicLong(0L)

    val calls: Long
        get() = _calls.get()

    private fun <ReqT: Any, RespT: Any> createCallListener(
        listener: ForwardingServerCall.SimpleForwardingServerCall<ReqT, RespT>,
        headers: Metadata,
        next: ServerCallHandler<ReqT, RespT>
    ): ForwardingServerCallListener.SimpleForwardingServerCallListener<ReqT> =
        object: ForwardingServerCallListener.SimpleForwardingServerCallListener<ReqT>(next.startCall(listener, headers)) {}

    override fun <ReqT: Any, RespT: Any> interceptCall(
        call: ServerCall<ReqT, RespT>,
        headers: Metadata,
        next: ServerCallHandler<ReqT, RespT>
    ): ServerCall.Listener<ReqT> {
        val currId = id.incrementAndGet()
        val listener = object: ForwardingServerCall.SimpleForwardingServerCall<ReqT, RespT>(call) {
            override fun sendMessage(message: RespT) {
                stopwatches["${call.methodDescriptor.fullMethodName}:$currId"] = StopWatch.createStarted()
                super.sendMessage(message)
            }

            override fun close(status: Status, trailers: Metadata) {
                val stopwatch = stopwatches["${call.methodDescriptor.fullMethodName}:$currId"]
                stopwatch?.stop()

                // increment the calls i think
                _calls.incrementAndGet()

                val contentType = headers.get(Metadata.Key.of("content-type", Metadata.ASCII_STRING_MARSHALLER))
                val userAgent = headers.get(Metadata.Key.of("user-agent", Metadata.ASCII_STRING_MARSHALLER))

                // do not log errors if the status code is not `CANCELLED` since it'll
                // probably spam the console.
                if (!status.isOk && status.code != Status.Code.CANCELLED) {
                    val ex = status.asException(headers)

                    // Don't log the exception itself since it's just:
                    // io.grpc.StatusException: ...
                    //     [insert some good stacktrace here mmmm]
                    // Caused by: ...
                    val message = buildString {
                        appendLine("Finished request execution with ID [#$currId]")
                        append("[status=${status.code}, authority=${call.authority}, method-descriptor=${call.methodDescriptor.fullMethodName}")
                        if (userAgent != null) {
                            append(", user-agent=$userAgent")
                        }

                        if (contentType != null) {
                            append(", content-type=$contentType")
                        }

                        append("]")
                    }

                    log.error(message, ex.cause)
                } else {
                    val message = buildString {
                        appendLine(
                            if (stopwatch != null) {
                                "Finished request execution with ID [#$currId] in ${stopwatch.doFormatTime()}"
                            } else {
                                "Finished request execution with ID [#$currId]"
                            }
                        )

                        append("infos: ")
                        append("[method-descriptor=${call.methodDescriptor.fullMethodName}")

                        if (userAgent != null) {
                            append(", user-agent=$userAgent")
                        }

                        if (contentType != null) {
                            append(", content-type=$contentType")
                        }

                        append("]")
                    }

                    log.info(message)
                }

                super.close(status, trailers)
            }
        }

        return createCallListener(listener, headers, next)
    }
}

/*
        val listener = object: ForwardingServerCall.SimpleForwardingServerCall<ReqT, RespT>(call) {
            private val stopwatch: StopWatch = StopWatch.create()
            private val log by logging<SimpleForwardingServerCall<ReqT, RespT>>()

            override fun sendMessage(message: RespT) {
                log.info("Sending message [$message] to clients!")
                // stopwatch.start()

                super.sendMessage(message)
            }

            override fun close(status: Status, trailers: Metadata) {
                // stopwatch.stop()
                log.info("Took ${stopwatch.formatTime()} to send message. [$status, $trailers]")

                super.close(status, trailers)
            }
        }

        return object: ForwardingServerCallListener.SimpleForwardingServerCallListener<ReqT>(next.startCall(listener, headers)) {}
    }
 */
