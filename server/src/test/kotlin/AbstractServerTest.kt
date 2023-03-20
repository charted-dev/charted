package org.noelware.charted.server

import io.ktor.server.application.*
import org.noelware.charted.common.extensions.reflection.setField
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.Feature
import org.noelware.charted.testing.containers.PostgreSQLContainer
import org.noelware.charted.testing.containers.RedisContainer
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.contracts.ExperimentalContracts
import kotlin.contracts.InvocationKind
import kotlin.contracts.contract

@Testcontainers(disabledWithoutDocker = true)
abstract class AbstractServerTest(
    registrations: Boolean = true,
    isInviteOnly: Boolean = false,
    elasticearch: Boolean = false,
    meilisearch: Boolean = false,
    features: List<Feature> = listOf()
) {
    private val config: Config by lazy {
        Config {
            this.registrations = registrations
            inviteOnly = isInviteOnly

            setField("database", postgresContainer.configuration)
            setField("features", features)
            setField("redis", redisContainer.configuration)

            storage {
                filesystem("./.data")
            }
        }
    }

    @OptIn(ExperimentalContracts::class)
    fun withServer(appModule: (Application.() -> Unit)? = null, testFunction: ServerTestFunction) {
        contract { callsInPlace(testFunction, InvocationKind.EXACTLY_ONCE) }

        val server = TestServer(config, testFunction, appModule)
        server.start()
    }

    companion object {
        @Container
        private val postgresContainer: PostgreSQLContainer = PostgreSQLContainer()

        @Container
        private val redisContainer: RedisContainer = RedisContainer()
    }
}
