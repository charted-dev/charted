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

package org.noelware.charted.analytics

import org.noelware.charted.analytics.protobufs.v1.*
import java.time.Instant

object AnalyticsService: AnalyticsGrpcKt.AnalyticsCoroutineImplBase() {
    override suspend fun connectionAck(request: ConnectionAckRequest): ConnectionAckResponse = ConnectionAckResponse.newBuilder()
        .setConnected(true)
        .setInstanceUUID(fetchInstanceUUID().toString())
        .build()

    override suspend fun retrieveStats(request: ReceiveStatsRequest): ReceiveStatsResponse {
        val response = ReceiveStatsResponse.newBuilder().apply {
            product = "charted-server"
            version = "1.0.0"
            commitSha = "12345678"
            buildDate = Instant.now().toString()
            buildFlavour = BuildFlavour.GIT
        }

        return response.build()
    }
}
