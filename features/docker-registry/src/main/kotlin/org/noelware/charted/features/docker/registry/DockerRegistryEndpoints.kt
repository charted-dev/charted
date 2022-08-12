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

package org.noelware.charted.features.docker.registry

import dev.floofy.utils.exposed.asyncTransaction
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import kotlinx.serialization.json.addJsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import kotlinx.serialization.json.putJsonArray
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.data.Config
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.tables.UserTable
import org.noelware.charted.features.docker.registry.builders.DockerRegistryHeadersBuilder
import org.noelware.charted.features.docker.registry.servicetokens.ServiceTokenManager
import org.noelware.ktor.endpoints.*
import org.springframework.security.crypto.argon2.Argon2PasswordEncoder
import java.util.Base64

// we have to prefix it with `/v2` so it can work with `helm push`/`helm pull`.
class DockerRegistryEndpoints(
    private val serviceTokens: ServiceTokenManager,
    private val argon2: Argon2PasswordEncoder,
    private val registry: DockerRegistry,
    private val config: Config
): AbstractEndpoint("/v2/") {
    @Get
    suspend fun main(call: ApplicationCall) {
        val authHeader = call.request.header(HttpHeaders.Authorization)
        if (authHeader == null) {
            val serverUrl = if (config.baseUrl != null) config.baseUrl else "http${if (config.ssl != null) "s" else ""}://${config.server.host}:${config.server.port}"
            call.response.header("WWW-Authenticate", "Bearer realm=\"$serverUrl/v2/token\",service=\"container_registry\",scope=\"*\"")

            throw RegistryException(RegistryErrorCode.UNAUTHORIZED)
        }

        call.doRespond(data = "")
    }

    @Get("/token")
    suspend fun token(call: ApplicationCall) {
        val auth = call.request.header(HttpHeaders.Authorization)
        if (auth == null) {
            val serverUrl = if (config.baseUrl != null) config.baseUrl else "http${if (config.ssl != null) "s" else ""}://${config.server.host}:${config.server.port}"
            call.response.header("WWW-Authenticate", "Bearer realm=\"$serverUrl/v2/token\",service=\"container_registry\",scope=\"*\"")

            throw RegistryException(RegistryErrorCode.UNAUTHORIZED)
        }

        val split = auth.split(" ", limit = 2)
        if (split.size < 2) {
            throw RegistryException(RegistryErrorCode.UNSUPPORTED)
        }

        val prefix = split.first()
        val base64 = split.last()

        if (prefix != "Basic") {
            throw RegistryException(RegistryErrorCode.UNSUPPORTED)
        }

        val decoded = Base64.getDecoder().decode(base64)
        val (username, password) = String(decoded).split(":", limit = 2)

        val user = asyncTransaction(ChartedScope) {
            UserEntity.find { UserTable.username eq username }.firstOrNull()
        } ?: return call.doRespond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                putJsonArray("errors") {
                    addJsonObject {
                        put("code", "UNKNOWN_USER")
                        put("message", "Unknown user with username [$username]")
                    }
                }
            }
        )

        if (!argon2.matches(password, user.password)) {
            throw RegistryException(RegistryErrorCode.UNAUTHORIZED)
        }

        val token = serviceTokens.createServiceToken(user.id.value)
        return call.doRespond(
            data = buildJsonObject {
                put("token", token.token)
            }
        )
    }

    @Get("/_catalog")
    suspend fun catalog(call: ApplicationCall) {}

    @Get("/{name}/blobs/{digest}")
    suspend fun blob(call: ApplicationCall) {}

    @Get("/{name}/manifests/{ref}")
    suspend fun manifestReference(call: ApplicationCall) {}

    @Post("/{name}/blobs/upload")
    suspend fun uploadBlob(call: ApplicationCall) {}

    @Patch("/{name}/blobs/uploads/{ref}")
    suspend fun patchBlobReference(call: ApplicationCall) {}

    @Put("/{name}/blobs/uploads/{ref}")
    suspend fun putBlobReference(call: ApplicationCall) {}

    @Put("/{name}/manifests/{ref}")
    suspend fun putManifestReference(call: ApplicationCall) {}

    @Get("/{name}/tags/list")
    suspend fun tags(call: ApplicationCall) {}

    @Delete("/{name}/manifests/{ref}")
    suspend fun deleteManifestReference(call: ApplicationCall) {}

    @Delete("/{name}/blobs/{digest}")
    suspend fun deleteBlob(call: ApplicationCall) {}

    private val ApplicationCall.authToken: String
        get() {
            val authHeader = request.header(HttpHeaders.Authorization)
            if (authHeader == null) {
                val serverUrl = if (config.baseUrl != null) config.baseUrl else "http${if (config.ssl != null) "s" else ""}://${config.server.host}:${config.server.port}"
                response.header("WWW-Authenticate", "Bearer realm=\"$serverUrl/v2/token\",service=\"container_registry\",scope=\"*\"")

                throw RegistryException(RegistryErrorCode.UNAUTHORIZED)
            }

            val data = authHeader.split(" ", limit = 2)
            if (data.size < 2 || data.size > 2) {
                throw RegistryException(RegistryErrorCode.UNAUTHORIZED)
            }

            return data.last()
        }
}

/*
        val authHeader = call.request.header(HttpHeaders.Authorization)
        if (authHeader == null) {
            val serverUrl = if (config.baseUrl != null) config.baseUrl else "http${if (config.ssl != null) "s" else ""}://${config.server.host}:${config.server.port}"
            call.response.header("WWW-Authenticate", "Bearer realm=\"$serverUrl/v2/token\",service=\"container_registry\",scope=\"*\"")

            throw RegistryException(RegistryErrorCode.UNAUTHORIZED)
        }
 */

// Helper method to add the headers we need.
suspend inline fun <reified T: Any> ApplicationCall.doRespond(
    status: HttpStatusCode = HttpStatusCode.OK,
    data: T,
    builder: DockerRegistryHeadersBuilder.() -> Unit = {}
) {
    val headers = DockerRegistryHeadersBuilder().apply(builder).build()
    if (!isHandled) {
        if (headers.range != null) {
            response.header("Range", headers.range)
        }

        if (headers.uploadUUID != null) {
            response.header("Docker-Upload-Uuid", headers.uploadUUID)
        }

        if (headers.contentDigest != null) {
            response.header("Docker-Content-Digest", headers.contentDigest)
            response.header("ETag", "\"${headers.contentDigest}\"")
        }

        response.header("Docker-Distribution-Api-Version", "registry/2.0")
    }

    respond(status, data)
}
