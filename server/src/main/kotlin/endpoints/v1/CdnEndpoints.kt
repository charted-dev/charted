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

package org.noelware.charted.server.endpoints.v1

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.http.*
import io.ktor.server.response.*
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.createKtorContentWithByteArray
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import org.noelware.remi.support.filesystem.FilesystemStorageService

class CdnEndpoints(private val storage: StorageHandler): AbstractEndpoint("/cdn") {
    @Get("/{params...}")
    suspend fun main(call: ApplicationCall) {
        val paths = (call.parameters.getAll("params") ?: listOf()).joinToString("/")
        val searchPath = if (storage.service is FilesystemStorageService) {
            "./$paths"
        } else {
            paths
        }

        val stream = storage.blob(searchPath)
            ?: return call.respond(HttpStatusCode.NotFound)

        if (stream.createdAt() != null) call.response.header("X-File-Created-At", stream.createdAt()!!.toHttpDateString())
        if (stream.lastModifiedAt() != null) call.response.header("X-File-Last-Modified", stream.lastModifiedAt()!!.toHttpDateString())
        if (stream.etag() != null) call.response.header("Etag", stream.etag()!!)

        val data = stream.inputStream()!!.use { it.readBytes() }
        val contentType = ContentType.parse(stream.contentType() ?: "application/octet-stream")
        call.respond(createKtorContentWithByteArray(data, contentType))
    }
}
