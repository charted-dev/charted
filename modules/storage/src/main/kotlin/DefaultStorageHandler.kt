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

package org.noelware.charted.modules.storage

import co.elastic.apm.api.Traced
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.noelware.charted.common.SetOnce
import org.noelware.charted.configuration.kotlin.dsl.StorageConfig
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.remi.core.Object
import org.noelware.remi.core.StorageTrailer
import org.noelware.remi.filesystem.FilesystemStorageTrailer
import org.noelware.remi.minio.MinIOStorageTrailer
import org.noelware.remi.s3.S3StorageTrailer
import java.io.InputStream

class DefaultStorageHandler(private val config: StorageConfig): StorageHandler {
    private val _trailer: SetOnce<StorageTrailer<*>> = SetOnce()
    private val log by logging<DefaultStorageHandler>()

    /**
     * Returns the underlying [storage trailer][StorageTrailer] itself. It is not recommended to
     * use this directly.
     */
    override val trailer: StorageTrailer<*>
        get() = _trailer.value

    /**
     * Initializes the storage handler, and calls [StorageTrailer#init][org.noelware.remi.core.StorageTrailer.init] afterwards.
     */
    @Traced
    override suspend fun init() {
        if (_trailer.wasSet()) {
            log.warn("#init was called more than once! this might be a bug")
            return
        }

        log.info("Determining which storage trailer to use...")
        _trailer.value = when {
            config.filesystem != null -> FilesystemStorageTrailer(config.filesystem!!)
            config.minio != null -> MinIOStorageTrailer(config.minio!!)
            config.s3 != null -> S3StorageTrailer(config.s3!!)
            else -> throw IllegalStateException("Unable to load storage handler due to invalid configuration")
        }

        log.info("Using storage trailer ${trailer.name}!")
        try {
            trailer.init()
        } catch (e: NotImplementedError) {
            // skip this for now
        } catch (e: Exception) {
            ifSentryEnabled { Sentry.captureException(e) }
            throw e
        }
    }

    /**
     * Opens a file and returns the [InputStream], if the file exists. Calls the [StorageTrailer#open][org.noelware.remi.core.StorageTrailer.open]
     * method internally.
     *
     * @param path The path to locate
     * @return [InputStream] if file exists, otherwise null.
     */
    override suspend fun open(path: String): InputStream? = trailer.open(path)

    /**
     * Uploads a file to the storage trailer (i.e, Amazon S3). Calls the [StorageTrailer#upload][org.noelware.remi.core.StorageTrailer.upload]
     * method internally.
     *
     * @param path The path to upload the stream to
     * @param is The input stream to upload.
     * @param contentType content type for metadata
     * @return boolean for successful or failure indication
     */
    override suspend fun upload(path: String, `is`: InputStream, contentType: String): Boolean {
        val size = withContext(Dispatchers.IO) {
            `is`.available()
        }

        if (size == 0) throw IllegalStateException("Input stream is 0 bytes, make sure you didn't consume it before using #upload")
        return trailer.upload(path, `is`, contentType)
    }

    /**
     * Checks if the file exists on the trailer itself. Calls the [StorageTrailer#exists][org.noelware.remi.core.StorageTrailer.exists]
     * method internally.
     *
     * @param path The path to locate
     * @return boolean to indicate if it exists
     */
    override suspend fun exists(path: String): Boolean = trailer.exists(path)

    /**
     * Deletes a file on the trailer. Calls the [StorageTrailer#delete][org.noelware.remi.core.StorageTrailer.delete] method
     * internally.
     *
     * @param path The path to locate
     * @return boolean to indicate if the file was deleted or not
     */
    override suspend fun delete(path: String): Boolean = trailer.delete(path)

    /**
     * Recursively collects all the files available in the trailer. Calls the [StorageTrailer#listAll][org.noelware.remi.core.StorageTrailer.listAll]
     * method internally.
     */
    override suspend fun list(): List<Object> = trailer.listAll(true)

    /**
     * Unlike [open], which returns a [InputStream], [get] returns all the metadata (and including input stream)
     * from the path specified. Calls the [StorageTrailer#fetch][org.noelware.remi.core.StorageTrailer.fetch] method
     * internally.
     *
     * @param path The path to locate
     * @return the [Object] metadata if the file was found, otherwise null.
     */
    override suspend fun get(path: String): Object? = trailer.fetch(path)
}
