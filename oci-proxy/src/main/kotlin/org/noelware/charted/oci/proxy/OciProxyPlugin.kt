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

package org.noelware.charted.oci.proxy

import io.ktor.client.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.util.*
import kotlinx.serialization.json.Json
import org.noelware.charted.common.config.AuthStrategy
import org.noelware.charted.common.config.NoAuthStrategy
import org.noelware.charted.common.config.OciProxyConfig
import kotlin.properties.Delegates

class OciProxyConfiguration {
    private var authStrategy: AuthStrategy = NoAuthStrategy()

    var httpClient: HttpClient by Delegates.notNull()
    var json: Json = Json
    var host: String = "localhost"
    var port: Int = 5000
    var ssl: Boolean = false

    fun fromConfig(config: OciProxyConfig) {
        this.authStrategy = config.auth
        this.host = config.host
        this.port = config.port
        this.ssl = config.ssl
    }

    fun <T: AuthStrategy> withAuthStrategy(strategy: T) {
        authStrategy = strategy
    }

    fun build(): Triple<OciProxyConfig, HttpClient, Json> =
        Triple(OciProxyConfig(authStrategy, port, host), httpClient, json)
}

val reverseProxyKey = AttributeKey<DockerRegistryReverseProxy>("Docker Registry Proxy")
val OciProxyPlugin = createApplicationPlugin("ChartedOciProxyPlugin", ::OciProxyConfiguration) {
    val (proxyConfig, httpClient, json) = pluginConfig.build()
    val proxy = DockerRegistryReverseProxy(proxyConfig, httpClient, json)

    onCall { call ->
        // Check if the URI path contains /v2/...
        if (call.request.uri.contains("/v2/")) {
            val shouldContinue = proxy.reroute(call)
            if (!shouldContinue) return@onCall
        }
    }
}
