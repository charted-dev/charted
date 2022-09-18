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

package org.noelware.charted.lib.avatars.fetcher

import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import org.noelware.charted.common.CryptoUtils
import org.noelware.charted.lib.avatars.AvatarFetcher

class GravatarAvatarFetcher(private val httpClient: HttpClient): AvatarFetcher {
    // seed = gravatar email from database
    override suspend fun fetch(seed: String): ByteArray {
        val hash = CryptoUtils.md5Hex(seed)
        val resp = httpClient.get("https://secure.gravatar.com/avatar/$hash")

        return resp.body()
    }
}
