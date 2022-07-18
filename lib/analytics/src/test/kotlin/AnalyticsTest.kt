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

package org.noelware.charted.analytics.tests

import io.grpc.inprocess.InProcessChannelBuilder
import io.grpc.inprocess.InProcessServerBuilder
import io.grpc.testing.GrpcCleanupRule
import org.junit.Rule
import org.junit.Test
import org.mockito.AdditionalAnswers
import org.mockito.kotlin.mock
import org.noelware.charted.analytics.protobufs.v1.*
import org.noelware.charted.common.SetOnceGetValue
import java.time.Instant
import kotlin.test.BeforeTest
import kotlin.test.assertIs
import kotlin.test.assertTrue

class AnalyticsTest: AbstractAnalyticsTest() {
    @get:Rule
    val cleanup = GrpcCleanupRule()

    private val _client = SetOnceGetValue<AnalyticsGrpc.AnalyticsBlockingStub>()
    private val serviceMock = mock<AnalyticsGrpcKt.AnalyticsCoroutineImplBase>(
        defaultAnswer = AdditionalAnswers.delegatesTo(object:
                AnalyticsGrpcKt.AnalyticsCoroutineImplBase() {
                override suspend fun connectionAck(request: ConnectionAckRequest): ConnectionAckResponse = ConnectionAckResponse.newBuilder()
                    .setConnected(true)
                    .setInstanceUUID(TEST_ANALYTICS_INSTANCE_UUID)
                    .build()

                override suspend fun retrieveStats(request: ReceiveStatsRequest): ReceiveStatsResponse = ReceiveStatsResponse.newBuilder()
                    .setProduct("charted-server")
                    .setVersion("1.2.3-nightly")
                    .setCommitSha("d3a6gz9s")
                    .setBuildDate(Instant.now().toString())
                    .setBuildFlavour(BuildFlavour.GIT)
                    .build()
            })
    )

    @BeforeTest
    fun setup() {
        val serverName = InProcessServerBuilder.generateName()
        cleanup.register(InProcessServerBuilder.forName(serverName).directExecutor().addService(serviceMock).build().start())

        val channel = cleanup.register(
            InProcessChannelBuilder.forName(serverName).directExecutor().build()
        )

        _client.value = AnalyticsGrpc.newBlockingStub(channel)
    }

    @Test
    fun `can we connect`() {
        val res = _client.value.connectionAck(ConnectionAckRequest.newBuilder().build())
        assertTrue(res.connected)
        assertIs<String>(TEST_ANALYTICS_INSTANCE_UUID)
    }
}
