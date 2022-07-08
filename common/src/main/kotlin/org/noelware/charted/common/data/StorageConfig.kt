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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.common.data

import org.noelware.remi.filesystem.FilesystemStorageConfig
import org.noelware.remi.minio.MinIOStorageConfig
import org.noelware.remi.s3.S3StorageConfig

/**
 * Represents the storage configuration for charted-server to use.
 */
@kotlinx.serialization.Serializable
data class StorageConfig(
    /**
     * Configures the filesystem as a storage source!
     */
    val filesystem: FilesystemStorageConfig? = null,

    /**
     * Configures a MinIO server as a storage source.
     */
    val minio: MinIOStorageConfig? = null,

    /**
     * Alias for [filesystem].
     */
    val fs: FilesystemStorageConfig? = null,

    /**
     * Configures Amazon S3 or a S3-compatible server to use for storing data.
     */
    val s3: S3StorageConfig? = null
)
