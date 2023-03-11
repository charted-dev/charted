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

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.Serializable
import org.noelware.charted.common.Buildable

/**
 * Represents the configuration to proxy the storage handler's contents to a mounted endpoint
 * with the [prefix].
 *
 * @param enabled If the storage handler proxy is enabled
 * @param prefix  prefix to mount all the storage handler's contents towards
 */
@Serializable
public data class CdnConfig(
    val enabled: Boolean = false,
    val prefix: String = "/cdn"
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: Buildable<CdnConfig> {
        /** prefix to mount all the storage handler's contents towards */
        public var prefix: String = "/cdn"

        override fun build(): CdnConfig = CdnConfig(true, prefix)
    }
}
