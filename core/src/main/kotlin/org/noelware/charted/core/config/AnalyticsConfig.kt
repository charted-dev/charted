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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core.config

/**
 * Represents the configuration to configure the Analytics daemon to be connected with
 * [analytics.noelware.org](https://analytics.noelware.org). Once the REST server has been bootstrapped,
 * in a background thread, the daemon of the gRPC server that will be connected.
 *
 * It will try to check if the server is healthy with the `ConnectionAck` RPC, and it will be responded
 * if the daemon has been configured.
 *
 * ## How can we verify if it's Noelware?
 * We will send the following headers:
 *
 * - `X-Analytics-Timestamp` - Represents the timestamp of when the request was being sent
 * - `X-Analytics-Signature` - SHA256 signature with the secret key configured.
 * - `X-Analytics-Id` - The [analyticsId] to compare the hash with.
 *
 * Then, you compare the hashes that was generated from the Analytics server to us.
 *
 * @param port The port of the gRPC server that it should use.
 * @param analyticsId The
 */
@kotlinx.serialization.Serializable
data class AnalyticsConfig(
    val port: Long = 9987,
    val analyticsId: String,
    val signatureSecret: String
)
