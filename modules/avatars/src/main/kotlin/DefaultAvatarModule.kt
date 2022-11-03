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

package org.noelware.charted.modules.avatars

import org.noelware.charted.common.CryptographyUtils
import org.noelware.charted.configuration.kotlin.dsl.Config

class DefaultAvatarModule(private val config: Config): AvatarModule {
    override fun charted(id: Long): String {
        val baseUrl = config.baseUrl
            ?: "http${if (config.server.ssl != null) "s" else ""}://${config.server.host}:${if (config.server.ssl != null) config.server.ssl!!.sslPort else config.server.port}"

        return "$baseUrl/users/$id/avatars/current.png"
    }

    override fun identicons(id: Long): String = "https://avatars.dicebear.com/api/identicon/$id.svg"
    override fun gravatar(email: String): String {
        val hash = CryptographyUtils.md5Hex(email)
        return "https://secure.gravatar.com/avatar/$hash.png"
    }
}
