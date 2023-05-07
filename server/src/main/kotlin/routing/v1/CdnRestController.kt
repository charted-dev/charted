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

package org.noelware.charted.server.routing.v1

import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.http.*
import io.ktor.server.plugins.cachingheaders.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.helm.charts.acceptableContentTypes
import org.noelware.charted.modules.storage.StorageModule
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.util.createBodyWithByteArray
import org.noelware.remi.support.filesystem.FilesystemStorageService
import kotlin.time.Duration.Companion.hours

class CdnRestController(
    config: Config,
    private val storage: StorageModule
): RestController("${config.cdn!!.prefix}/{params...}") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(CachingHeaders) {
            options { _, content ->
                val ct = content.contentType?.withoutParameters()
                if (acceptableContentTypes.map(ContentType::parse).any { it == ct }) {
                    return@options CachingOptions(CacheControl.MaxAge(maxAgeSeconds = 1.hours.inWholeSeconds.toInt()))
                }

                when (ct) {
                    ContentType.parse("text/yaml; charset=utf-8"), ContentType.Image.Any -> CachingOptions(CacheControl.MaxAge(maxAgeSeconds = 1.hours.inWholeSeconds.toInt()))
                    else -> null
                }
            }
        }
    }

    override suspend fun call(call: ApplicationCall) {
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
        val contentType = ContentType.parse(
            stream.contentType()
                ?: storage.service.getContentTypeOf(data)
                ?: "application/octet-stream",
        )

        call.respond(createBodyWithByteArray(data, contentType))
    }

    override fun toPathDsl(): PathItem = TODO("Not available to be generated since dynamic prefix")
}
