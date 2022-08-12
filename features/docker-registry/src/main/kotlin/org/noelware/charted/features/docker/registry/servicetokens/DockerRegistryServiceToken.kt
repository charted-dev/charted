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

package org.noelware.charted.features.docker.registry.servicetokens

/**
 * Represents a service token for using the Docker Registry. This is needed, so we don't use
 * the session tokens (since it can do any REST operation on a user, so let's just keep it safe
 * and only let it use the `/v2/` endpoints.)
 */
@kotlinx.serialization.Serializable
data class DockerRegistryServiceToken(
    val userID: Long,
    val token: String
)
