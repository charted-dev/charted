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

package org.noelware.charted.modules.storage

import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import org.noelware.charted.configuration.kotlin.dsl.storage.StorageConfig
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.remi.core.Blob
import org.noelware.remi.core.StorageService
import org.noelware.remi.core.UploadRequest
import org.noelware.remi.support.azure.AzureBlobStorageService
import org.noelware.remi.support.filesystem.FilesystemStorageService
import org.noelware.remi.support.gcs.GoogleCloudStorageService
import org.noelware.remi.support.s3.AmazonS3StorageService
import java.io.InputStream

class DefaultStorageHandler(private val config: StorageConfig) : StorageHandler {
    private val _service: SetOnce<StorageService<*>> = SetOnce()
    private val log by logging<DefaultStorageHandler>()

    override val service: StorageService<*>
        get() = _service.value

    override fun init() {
        if (_service.wasSet()) {
            log.warn("#init was already called!")
            return
        }

        log.info("Determining which storage service to use!")
        _service.value = when {
            config.filesystem != null -> FilesystemStorageService(config.filesystem!!.toRemiConfig())
            config.azure != null -> AzureBlobStorageService(config.azure!!.toRemiConfig())
            config.gcs != null -> GoogleCloudStorageService(config.gcs!!.toRemiConfig())
            config.s3 != null -> AmazonS3StorageService(config.s3!!.toRemiConfig())
            else -> {
                log.warn("Using filesystem storage service by default!")
                FilesystemStorageService("./data")
            }
        }

        log.info("Configured to use storage trailer ${service.name()}")
        try {
            service.init()
        } catch (e: Exception) {
            ifSentryEnabled { Sentry.captureException(e) }
            throw e
        }
    }

    override fun open(path: String): InputStream? = service.open(path)
    override fun exists(path: String): Boolean = service.exists(path)
    override fun delete(path: String): Boolean = service.delete(path)
    override fun list(): List<Blob> = service.blobs()
    override fun blob(path: String): Blob? = service.blob(path)
    override fun upload(path: String, `is`: InputStream, contentType: String) = service.upload(
        UploadRequest.builder()
            .withContentType(contentType)
            .withInputStream(`is`)
            .withPath(path)
            .build(),
    )
}
