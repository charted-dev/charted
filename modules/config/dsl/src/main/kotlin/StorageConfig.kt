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

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.remi.filesystem.FilesystemStorageConfig
import org.noelware.remi.minio.MinIOStorageConfig
import org.noelware.remi.s3.S3StorageConfig

@Serializable
data class StorageConfig(
    val filesystem: FilesystemStorageConfig? = null,

    @SerialName("host_alias")
    val hostAlias: String? = null,
    val minio: MinIOStorageConfig? = null,
    val s3: S3StorageConfig? = null
) {
    class Builder: org.noelware.charted.common.Builder<StorageConfig> {
        private var _filesystem: FilesystemStorageConfig? = null
        private var _minio: MinIOStorageConfig? = null
        private var _s3: S3StorageConfig? = null

        var hostAlias: String? = null

        infix fun filesystem(directory: String): Builder {
            if (_s3 != null || _minio != null) {
                throw IllegalStateException("Can't use 'filesystem' storage trailer since Amazon S3 or Minio was configured before 'filesystem $directory'")
            }

            _filesystem = FilesystemStorageConfig(directory)
            return this
        }

        fun minio(block: MinIOStorageConfig.() -> Unit = {}): Builder {
            if (_s3 != null || _filesystem != null) {
                throw IllegalStateException("Can't use 'minio' storage trailer since Amazon S3 or local disk was configured before 'minio {}'")
            }

            _minio = MinIOStorageConfig().apply(block)
            return this
        }

        fun s3(block: S3StorageConfig.() -> Unit = {}): Builder {
            if (_minio != null || _filesystem != null) {
                throw IllegalStateException("Can't use 's3' storage trailer since Minio or the local disk was configured before 's3 {}'")
            }

            _s3 = S3StorageConfig().apply(block)
            return this
        }

        override fun build(): StorageConfig = StorageConfig(_filesystem, hostAlias, _minio, _s3)
    }
}
