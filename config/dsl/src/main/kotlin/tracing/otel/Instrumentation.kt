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

package org.noelware.charted.configuration.kotlin.dsl.tracing.otel

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
public enum class Instrumentation {
    @SerialName("otel_api")
    OpenTelemetryAPI,

    @SerialName("apache_httpclient")
    ApacheHttpClient,

    @SerialName("java_httpclient")
    JavaHttpClient,

    @SerialName("java_executor")
    JavaExecutor,

    @SerialName("elasticsearch")
    Elasticsearch,

    @SerialName("kotlinx_coroutines")
    Coroutines,

    @SerialName("azure_sdk")
    AzureSDK,

    @SerialName("hikaricp")
    HikariCP,

    @SerialName("lettuce")
    Lettuce,

    @SerialName("logback")
    Logback,

    @SerialName("aws_sdk")
    AWSSDK,

    @SerialName("grpc")
    GRPC,

    @SerialName("jdbc")
    JDBC,

    @SerialName("ktor")
    Ktor,

    /**
     * Enables all the supported instrumentations for charted-server with OpenTelemetry.
     */
    @SerialName("*")
    Wildcard;

    public object EnumSet: org.noelware.charted.configuration.kotlin.dsl.enumSets.EnumSet<Instrumentation>(Instrumentation::class) {
        override val wildcard: Instrumentation
            get() = Wildcard
    }
}

public val Instrumentation.otelValue: List<String>
    get() = when (this) {
        Instrumentation.OpenTelemetryAPI -> listOf("opentelemetry-api")
        Instrumentation.ApacheHttpClient -> listOf("apache-httpclient")
        Instrumentation.JavaHttpClient -> listOf("java-http-client")
        Instrumentation.Elasticsearch -> listOf("elasticsearch-transport", "elasticsearch-rest")
        Instrumentation.JavaExecutor -> listOf("executor")
        Instrumentation.Coroutines -> listOf("kotlinx-coroutines")
        Instrumentation.HikariCP -> listOf("hikaricp")
        Instrumentation.AzureSDK -> listOf("azure-core")
        Instrumentation.Wildcard -> listOf()
        Instrumentation.Lettuce -> listOf("lettuce")
        Instrumentation.Logback -> listOf("logback-appender", "logback-mdc")
        Instrumentation.AWSSDK -> listOf("aws-sdk")
        Instrumentation.JDBC -> listOf("jdbc")
        Instrumentation.Ktor -> listOf("ktor")
        Instrumentation.GRPC -> listOf("grpc")
    }
