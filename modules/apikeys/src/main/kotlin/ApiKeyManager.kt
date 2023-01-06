/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.modules.apikeys

import org.noelware.charted.databases.postgres.models.ApiKeys
import java.io.Closeable
import kotlin.time.Duration

/**
 * Represents the manager for handling API key expiration dates. Expiration dates should be only inserted
 * if the expiration date is valid.
 */
interface ApiKeyManager : Closeable {
    /**
     * Accepts that the API key will be expiring in the specified [duration][expiresIn] and is sent in Redis.
     * @param apiKey    The API key that is expiring
     * @param expiresIn The duration of when the API key will expire
     */
    suspend fun send(apiKey: ApiKeys, expiresIn: Duration)
}
