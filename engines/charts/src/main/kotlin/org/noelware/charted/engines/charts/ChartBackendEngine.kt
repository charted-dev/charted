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

package org.noelware.charted.engines.charts

import dev.floofy.utils.slf4j.logging
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.noelware.charted.core.StorageWrapper
import org.noelware.remi.core.figureContentType
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream

/**
 * Represents the [chart repository](https://helm.sh/docs/topics/chart_repository/#helm) backend that is used
 * to store repositories and can be fetched.
 */
class ChartBackendEngine(private val storage: StorageWrapper) {
    private val log by logging<ChartBackendEngine>()

    suspend fun upload(call: ApplicationCall, ownerId: String, projectId: String) {
        log.debug("Checking if we can receive a multipart upload!")
        val multipart = call.receiveMultipart()

        val parts = multipart.readAllParts()
        log.debug("Received ${parts.size} parts.")

        val firstPart = parts.firstOrNull() ?: error("There can be only one file part or there was none!")
        if (firstPart !is PartData.FileItem) {
            error("The multipart was not a file, must be a file!")
        }

        if (firstPart.originalFileName == null)
            error("There should be a file name in the content disposition header.")

        val inputStream = firstPart.streamProvider()

        // Create a clone of `inputStream` since it'll be exhausted
        // when we use getTarContents
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        val newStream = ByteArrayInputStream(data)

        // `inputStream` is now exhausted.
        val contentType = storage.trailer.figureContentType(inputStream)
        if (!contentType.startsWith("application/gzip") || !contentType.startsWith("application/tar"))
            error("The file was not a tarball.")

        // Now, let's upload it.
        storage.upload(
            "./$ownerId/$projectId/tarballs/${firstPart.originalFileName}",
            newStream,
            contentType
        )

        firstPart.dispose()
    }

    suspend fun uploadIndexYaml(call: ApplicationCall, ownerId: String) {
        log.debug("Checking if we can receive a multipart upload!")
        val multipart = call.receiveMultipart()

        val parts = multipart.readAllParts()
        log.debug("Received ${parts.size} parts.")

        val firstPart = parts.firstOrNull() ?: error("There can be only one file part or there was none!")
        if (firstPart !is PartData.FileItem) {
            error("The multipart was not a file, must be a file!")
        }

        if (firstPart.originalFileName == null)
            error("There should be a file name in the content disposition header.")

        if (firstPart.originalFileName != "index.yaml")
            error("When using PUT /repositories/:id/index.yaml, you must set the file name to \"index.yaml\"!")

        val inputStream = firstPart.streamProvider()

        // Create a clone of `inputStream` since it'll be exhausted
        // when we use getTarContents
        val baos = ByteArrayOutputStream()
        withContext(Dispatchers.IO) {
            inputStream.transferTo(baos)
        }

        val data = baos.toByteArray()
        val newStream = ByteArrayInputStream(data)

        // `inputStream` is now exhausted.
        val contentType = storage.trailer.figureContentType(inputStream)
        if (!contentType.startsWith("application/yaml"))
            error("The file was not a YAML-formatted document.")

        // Now, let's upload it.
        storage.upload(
            "./$ownerId/index.yaml",
            newStream,
            contentType
        )

        firstPart.dispose()
    }
}
