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

package org.noelware.charted.server.endpoints

import dev.floofy.utils.slf4j.logging
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import kotlinx.serialization.json.addJsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import org.noelware.charted.common.extensions.measureTime
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.server.utils.createOutgoingContentWithBytes
import org.noelware.remi.filesystem.FilesystemStorageConfig
import org.noelware.remi.filesystem.FilesystemStorageTrailer

suspend fun Routing.proxyStorageTrailer(storage: StorageWrapper) {
    val log by logging("org.noelware.charted.server.endpoints.CdnEndpointsKt\$proxyStorageTrailer")
    log.debug("Configuring CDN endpoints from storage trailer...")

    val contents = storage.trailer.listAll()
    val cdnPrefix = "/cdn" // TODO: add it back?
    log.measureTime("Took %T to configure the storage trailer proxy.") {
        for (content in contents) {
            val prefix = if (storage.trailer is FilesystemStorageTrailer) {
                content
                    .path
                    .replace("file://", "")
                    .replace((storage.trailer.config as FilesystemStorageConfig).directory, "")
            } else {
                content.path
                    .replace("s3://", "")
                    .replace("minio://", "")
            }

            log.debug("|- GET $cdnPrefix$prefix")
            get("$cdnPrefix$prefix") {
                val stream = storage.trailer.fetch(if (storage.trailer is FilesystemStorageTrailer) "./$prefix" else prefix)
                    ?: return@get call.respond(
                        HttpStatusCode.NotFound,
                        buildJsonObject {
                            put("success", false)
                            putJsonArray("errors") {
                                addJsonObject {
                                    put("code", "UNKNOWN_FILE")
                                    put("message", "File was not found or was deleted, sorry!")
                                }
                            }
                        }
                    )

                if (stream.createdAt != null) {
                    call.response.header("File-Created-At", stream.createdAt.toString())
                }

                call.response.header("File-Last-Modified", stream.lastModifiedAt.toString())
                call.response.header("ETag", stream.etag)

                val data = stream.inputStream!!.readBytes()
                val contentType = ContentType.parse(stream.contentType)
                call.respond(
                    createOutgoingContentWithBytes(
                        data,
                        contentType = contentType
                    )
                )
            }
        }
    }
}
