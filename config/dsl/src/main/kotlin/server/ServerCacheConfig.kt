/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.configuration.kotlin.dsl.server

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.Buildable

@Serializable
public enum class CacheDriver {
    @SerialName("in-memory")
    InMemory,

    @SerialName("redis")
    Redis,

    @SerialName("none")
    None
}

/**
 * Configuration for configuring response cache between objects to help
 * reduce latency between API calls.
 *
 * @param driver Cache driver to use. Using `none` will disable caching and all
 * entries will be fresh, `in-memory` will use Caffeine to store objects, and
 * `redis` will use Redis.
 */
@Serializable
public data class ServerCacheConfig(
    val driver: CacheDriver = CacheDriver.None
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: Buildable<ServerCacheConfig> {
        /**
         * [CacheDriver] to use to find which cache strategy to resolve
         * to for objects.
         *
         * * `in-memory`: Uses Caffeine to store entries.
         * * `redis`: Uses the established Redis connection to store entries.
         * * `none`: Disables caching and all entries will be fresh from PostgreSQL.
         */
        public var driver: CacheDriver = CacheDriver.None

        override fun build(): ServerCacheConfig = ServerCacheConfig(driver)
    }
}
