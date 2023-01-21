package org.noelware.charted.modules.search.meilisearch

import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.noelware.charted.configuration.kotlin.dsl.search.MeilisearchConfig

class DefaultMeilisearchModule(
    private val config: MeilisearchConfig,
    private val httpClient: HttpClient,
    private val json: Json = Json
): MeilisearchModule {
    private val _serverVersion: SetOnce<String> = SetOnce()
    private val log by logging<DefaultMeilisearchModule>()

    override val serverVersion: String
        get() = _serverVersion.value

    override suspend fun init() {
        log.info("initializing Meilisearch module...")

        // Check if we can ping the Meilisearch server or not
        val resp = get("/version")
        _serverVersion.value = json.decodeFromString(JsonObject.serializer(), resp.bodyAsText())["pkgVersion"]!!.jsonPrimitive.content

        log.info("Using Meilisearch v$serverVersion! Initializing all indexes...")
    }

    private suspend fun get(url: String, block: HttpRequestBuilder.() -> Unit = {}): HttpResponse = httpClient.get("${config.endpoint}$url") {
        if (config.masterKey != null) {
            header("Authorization", "Bearer ${config.masterKey}")
        }

        block()
    }
}
