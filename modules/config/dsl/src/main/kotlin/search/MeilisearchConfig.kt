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

package org.noelware.charted.configuration.kotlin.dsl.search

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.serializers.SecretStringSerializer

@Serializable
public data class MeilisearchConfig(
    @Serializable(with = SecretStringSerializer::class)
    @SerialName("master_key")
    val masterKey: String? = null,
    val endpoint: String = "http://localhost:7700"
) {
    public companion object {
        public operator fun invoke(builder: Builder.() -> Unit = {}): MeilisearchConfig = Builder().apply(builder).build()
    }

    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder : org.noelware.charted.common.Builder<MeilisearchConfig> {
        public var masterKey: String? = null
        public var endpoint: String = "http://localhost:7700"

        override fun build(): MeilisearchConfig = MeilisearchConfig(masterKey, endpoint)
    }
}
