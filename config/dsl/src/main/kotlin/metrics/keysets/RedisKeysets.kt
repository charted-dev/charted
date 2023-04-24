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

package org.noelware.charted.configuration.kotlin.dsl.metrics.keysets

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
public enum class RedisKeysets {
    @SerialName("charted_redis_total_connections_received")
    TotalConnectionsReceived,

    @SerialName("charted_redis_total_commands_processed")
    TotalCommandsProcessed,

    @SerialName("charted_redis_total_network_output")
    TotalNetworkOutput,

    @SerialName("charted_redis_total_network_input")
    TotalNetworkInput,

    @SerialName("charted_redis_allocator")
    Allocator,

    @SerialName("*")
    Wildcard,

    @SerialName("charted_redis_version")
    Version,

    @SerialName("charted_redis_uptime")
    Uptime,

    @SerialName("charted_redis_mode")
    Mode,

    @SerialName("charted_redis_ping")
    Ping;

    public object EnumSet: org.noelware.charted.configuration.kotlin.dsl.enumSets.EnumSet<RedisKeysets>(RedisKeysets::class) {
        override val wildcard: RedisKeysets
            get() = Wildcard
    }
}
