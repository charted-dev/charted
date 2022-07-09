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

package org.noelware.charted.common.data

import kotlinx.serialization.SerialName
import org.noelware.charted.common.exceptions.ValidationException

/**
 * Represents the configuration if we should proxy the storage trailer to the API itself
 * rather than a base URL that is probably preconfigured, this is useful if you want to have
 * a separate domain for your contents or just proxy it on the REST API.
 */
@kotlinx.serialization.Serializable
data class CdnConfig(
    /**
     * Represents the base URL to use. This the base URL that the chart configuration
     * can use for avatars, tarballs, and metadata.
     */
    @SerialName("base_url")
    val baseUrl: String? = null,

    /**
     * If the contents of your data in the storage trailer should be proxied via a REST API endpoint or not.
     * (defaults to `/cdn` if true)
     */
    @SerialName("proxy_contents")
    val proxyContents: Boolean = true,

    /**
     * The proxy prefix to use when configuring the CDN proxy.
     */
    @SerialName("proxy_prefix")
    val proxyPrefix: String = "/cdn"
) {
    init {
        if (!proxyPrefix.startsWith('/')) {
            throw ValidationException("config.cdn.proxy_prefix", "The proxy prefix must start with a leading slash.")
        }
    }
}

fun formatCdnUrl(
    config: Config,
    url: String
): String {
    if (config.cdn.baseUrl != null) {
        return "${config.cdn.baseUrl}$url"
    }

    return "${config.cdn.proxyPrefix}$url"
}
