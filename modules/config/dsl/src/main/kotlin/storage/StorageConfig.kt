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

package org.noelware.charted.configuration.kotlin.dsl.storage

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.storage.azure.AzureAuthenticationHost
import org.noelware.remi.support.s3.AmazonS3StorageConfig as RemiS3StorageConfig

@Serializable
public data class StorageConfig(
    val filesystem: FilesystemStorageConfig? = null,
    val azure: AzureBlobStorageConfig? = null,
    val gcs: GoogleCloudStorageConfig? = null,
    val s3: AmazonS3StorageConfig? = null,

    @SerialName("host_alias")
    val hostAlias: String? = null
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder : org.noelware.charted.common.Builder<StorageConfig> {
        private var _filesystem: FilesystemStorageConfig? = null
        private var _azure: AzureBlobStorageConfig? = null
        private var _gcs: GoogleCloudStorageConfig? = null
        private var _s3: AmazonS3StorageConfig? = null

        public var hostAlias: String? = null

        public fun filesystem(directory: String): Builder {
            if (_azure != null || _gcs != null || _s3 != null) {
                throw IllegalStateException("There is already a storage service config defined")
            }

            _filesystem = FilesystemStorageConfig(directory)
            return this
        }

        public fun azure(
            container: String,
            auth: AzureAuthenticationHost,
            endpoint: String? = null
        ): Builder {
            if (_filesystem != null || _gcs != null || _s3 != null) {
                throw IllegalStateException("There is already a storage service config defined")
            }

            _azure = AzureBlobStorageConfig(container, endpoint, auth)
            return this
        }

        public fun gcs(
            projectId: String,
            bucket: String,
            credentialsFile: String? = null
        ): Builder {
            if (_filesystem != null || _azure != null || _s3 != null) {
                throw IllegalStateException("There is already a storage service config defined")
            }

            _gcs = GoogleCloudStorageConfig(credentialsFile, projectId, bucket)
            return this
        }

        public fun s3(builder: RemiS3StorageConfig.Builder.() -> Unit = {}): Builder {
            if (_filesystem != null || _azure != null || _gcs != null) {
                throw IllegalStateException("There is already a storage service config defined")
            }

            _s3 = AmazonS3StorageConfig.fromS3Builder(builder)
            return this
        }

        override fun build(): StorageConfig = StorageConfig(_filesystem, _azure, _gcs, _s3, hostAlias)
    }
}
