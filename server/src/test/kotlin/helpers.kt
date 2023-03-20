package org.noelware.charted.server

import io.ktor.client.*
import io.ktor.server.testing.*
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.serialization.kotlinx.json.*

fun ApplicationTestBuilder.createHttpClient(): HttpClient = createClient {
    install(ContentNegotiation) {
        json()
    }
}
