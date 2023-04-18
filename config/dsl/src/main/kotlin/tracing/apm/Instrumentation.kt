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

package org.noelware.charted.configuration.kotlin.dsl.tracing.apm

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
public enum class Instrumentation {
    /**
     * Enables instrumentation on Elasticsearch's Java RestClient, which charted-server does use if you enable
     * Elasticsearch for searching.
     */
    @SerialName("elasticsearch_restclient")
    ElasticsearchRESTClient,

    /**
     * Enables instrumentation on Java's thread exception handler.
     */
    @SerialName("exception_handler")
    ExceptionHandler,

    /**
     * This will enable instrumentation on Apache's HTTP client. As charted-server doesn't use Apache's HTTP client
     * for sending requests, other components (like Elasticsearch) do, so it's worth tracing, if enabled.
     */
    @SerialName("apache_httpclient")
    ApacheHttpClient,

    /**
     * Enables instrumentation on the JDK's HTTP client, which charted-server does use to send
     * requests to.
     */
    @SerialName("jdk_httpclient")
    JDKHttpClient,

    /**
     * Enables instrumentation on the Redis client that charted-server uses.
     */
    @SerialName("lettuce")
    Lettuce,

    /**
     * Enables instrumentation on Java 8's executor, which kotlinx.coroutines is configured
     * to be used for, so it will auto-instrument all coroutines, if enabled.
     */
    @SerialName("executor")
    Executor,

    /**
     * This will enable instrumentation on AWS' SDK components, charted-server does connect to AWS S3
     * with [Remi's storage-s3](https://github.com/Noelware/remi/tree/master/support/s3), so if you do
     * use the S3 storage configuration, it's probably worth tracing.
     */
    @SerialName("aws_sdk")
    AWSSDK,

    /**
     * Enables instrumentation on the gRPC client for Noelware Analytics or charted's email microservice.
     */
    @SerialName("grpc")
    GRPC,

    /**
     * Enables instrumentation on SQL queries that were executed.
     */
    @SerialName("jdbc")
    JDBC,

    /**
     * Enables all the instrumentations that charted-server supports.
     */
    @SerialName("*")
    Wildcard;

    public object EnumSet: org.noelware.charted.configuration.kotlin.dsl.enumSets.EnumSet<Instrumentation>(Instrumentation::class) {
        override val wildcard: Instrumentation
            get() = Wildcard
    }
}
