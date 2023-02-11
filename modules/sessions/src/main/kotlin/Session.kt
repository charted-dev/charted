/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.modules.sessions

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.serializers.UUIDSerializer
import java.util.UUID

/**
 * Represents a session token object. This is how sessions are stored in the Redis table.
 * @param refreshToken The token for refreshing this session to get a new one. The web UI uses this
 *                     token to refresh your session when it expires. This token only lasts for 1 week,
 *                     and a new session will need to be created.
 * @param accessToken  The token for accessing the API server as the user. This is dangerous to someone who
 *                     knows your credentials.
 * @param sessionID    Unique identifier to identify this session.
 * @param user         The user who owns this session.
 */
@Serializable
data class Session(
    @SerialName("refresh_token")
    val refreshToken: String,

    @SerialName("access_token")
    val accessToken: String,

    @Serializable(with = UUIDSerializer::class)
    @SerialName("session_id")
    val sessionID: UUID,
    val userID: Long
) {
    fun toJsonObject(showToken: Boolean = false): JsonObject = buildJsonObject {
        if (showToken) {
            put("refresh_token", refreshToken)
            put("access_token", accessToken)
        }

        put("session_id", sessionID.toString())
        put("user_id", userID)
    }
}
