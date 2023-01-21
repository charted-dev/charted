package org.noelware.charted.testing.modules.meilisearch

import io.ktor.client.*
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.Test
import org.noelware.charted.modules.search.meilisearch.DefaultMeilisearchModule
import org.noelware.charted.modules.search.meilisearch.MeilisearchModule
import org.noelware.charted.testing.containers.MeilisearchContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals

@Testcontainers(disabledWithoutDocker = true)
class MeilisearchTests {
    @Container
    private val container: MeilisearchContainer = MeilisearchContainer()

    private suspend fun withMeilisearch(block: MeilisearchModule.() -> Unit = {}) {
        val module = DefaultMeilisearchModule(container.configuration, HttpClient {})
        module.init()
        module.block()
    }

    @Test
    fun `can we connect meilisearch`(): Unit = runBlocking {
        withMeilisearch {
        }
    }
}
