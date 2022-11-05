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

package org.noelware.charted.modules.redis.metrics

import kotlinx.serialization.Serializable

/**
 * Represents the collected statistics the metrics collector use.
 * @param totalConnectionsReceived Returns how many total connections the server has received
 * @param totalCommandsProcessed   Returns how many Redis commands were processed
 * @param totalNetworkOutput       Returns the total network output (in bytes) the server has sent out to us or any other client
 * @param totalNetworkInput        Returns the total network input (in bytes) we or other clients had sent out to the server
 * @param allocator                The allocator that the Redis server uses
 * @param version                  Redis server version
 * @param uptime                   The uptime (in milliseconds) how long the server has been up for
 * @param mode                     The server mode it is in ("standalone" or "clustered")
 * @param ping                     Returns the latency (in nanoseconds) from the server to us.
 */
@Serializable
data class RedisServerStats(
    val totalNetworkInput: Long,
    val totalNetworkOutput: Long,
    val totalCommandsProcessed: Long,
    val totalConnectionsReceived: Long,
    val allocator: String,
    val uptime: Long,
    val version: String,
    val mode: String,
    val ping: Long
)
