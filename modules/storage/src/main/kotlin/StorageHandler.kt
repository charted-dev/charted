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

import org.noelware.remi.core.Object
import java.io.InputStream

/**
 * Represents a proxy over the [StorageTrailer][org.noelware.remi.core.StorageTrailer] interface for
 * tracing and extensibility.
 */
interface StorageHandler {
    /**
     * Initializes the storage handler, and calls [StorageTrailer#init][org.noelware.remi.core.StorageTrailer.init] afterwards.
     */
    suspend fun init()

    /**
     * Opens a file and returns the [InputStream], if the file exists. Calls the [StorageTrailer#open][org.noelware.remi.core.StorageTrailer.open]
     * method internally.
     *
     * @param path The path to locate
     * @return [InputStream] if file exists, otherwise null.
     */
    suspend fun open(path: String): InputStream?

    /**
     * Uploads a file to the storage trailer (i.e, Amazon S3). Calls the [StorageTrailer#upload][org.noelware.remi.core.StorageTrailer.upload]
     * method internally.
     *
     * @param path The path to upload the stream to
     * @param is The input stream to upload.
     * @param contentType content type for metadata
     * @return boolean for successful or failure indication
     */
    suspend fun upload(path: String, `is`: InputStream, contentType: String): Boolean

    /**
     * Checks if the file exists on the trailer itself. Calls the [StorageTrailer#exists][org.noelware.remi.core.StorageTrailer.exists]
     * method internally.
     *
     * @param path The path to locate
     * @return boolean to indicate if it exists
     */
    suspend fun exists(path: String): Boolean

    /**
     * Deletes a file on the trailer. Calls the [StorageTrailer#delete][org.noelware.remi.core.StorageTrailer.delete] method
     * internally.
     *
     * @param path The path to locate
     * @return boolean to indicate if the file was deleted or not
     */
    suspend fun delete(path: String): Boolean

    /**
     * Recursively collects all the files available in the trailer. Calls the [StorageTrailer#listAll][org.noelware.remi.core.StorageTrailer.listAll]
     * method internally.
     */
    suspend fun list(): List<Object>

    /**
     * Unlike [open], which returns a [InputStream], [get] returns all the metadata (and including input stream)
     * from the path specified. Calls the [StorageTrailer#fetch][org.noelware.remi.core.StorageTrailer.fetch] method
     * internally.
     *
     * @param path The path to locate
     * @return the [Object] metadata if the file was found, otherwise null.
     */
    suspend fun get(path: String): Object?
}
