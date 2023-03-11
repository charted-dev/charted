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

package org.noelware.charted.configuration.kotlin.dsl.storage

import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.storage.azure.AzureAuthenticationHost
import org.noelware.charted.configuration.kotlin.dsl.storage.azure.toAzureConnectionAuth
import org.noelware.remi.support.azure.AzureBlobStorageConfig as RemiAzureBlobStorageConfig

/**
 * Represents the configuration to use when using the azure storage driver
 * @param container The blob container name
 * @param endpoint  The blob endpoint to use, don't define this if you're going to use [AzureAuthenticationHost.ConnectionStringAuthenticationHost]
 * @param auth      Authentication host for Azure Blob Storage
 */
@Serializable
public data class AzureBlobStorageConfig(
    val container: String,
    val endpoint: String? = null,
    val auth: AzureAuthenticationHost
) {
    public fun toRemiConfig(): RemiAzureBlobStorageConfig = RemiAzureBlobStorageConfig(container, endpoint ?: "", auth.toAzureConnectionAuth())
}
