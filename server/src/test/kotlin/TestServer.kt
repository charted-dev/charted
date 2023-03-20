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

package org.noelware.charted.server

import com.zaxxer.hikari.HikariDataSource
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import io.ktor.server.application.*
import io.ktor.server.testing.*
import kotlinx.atomicfu.atomic
import kotlinx.coroutines.cancel
import kotlinx.coroutines.runBlocking
import org.koin.core.context.GlobalContext
import org.noelware.charted.ChartedScope
import org.noelware.charted.Server
import org.noelware.charted.common.extensions.closeable.closeQuietly
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.redis.RedisClient
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.server.bootstrap.ConfigureModulesPhase
import org.noelware.charted.server.internal.DefaultServer

typealias ServerTestFunction = suspend ApplicationTestBuilder.() -> Unit

class TestServer(
    private val config: Config,
    private val testFunction: ServerTestFunction,
    private val appModule: (Application.() -> Unit)? = null
): Server by DefaultServer(config) {
    private val _started = atomic(false)
    private val log by logging<TestServer>()

    override val started: Boolean
        get() = _started.value

    override fun start() {
        runBlocking { ConfigureModulesPhase.phaseThrough(config) }

        log.info("Starting test server...")
        testApplication {
            application {
                module()
                appModule?.invoke(this)
            }

            testFunction()
            closeQuietly()
        }
    }

    override fun close() {
        log.warn("Closing server...")

        val koin = GlobalContext.getKoinApplicationOrNull()
        if (koin != null) {
            val sessions: AbstractSessionManager by inject()
            val hikari: HikariDataSource by inject()
            val redis: RedisClient by inject()

            sessions.closeQuietly()
            hikari.closeQuietly()
            redis.closeQuietly()

            runBlocking {
                ChartedScope.cancel()
            }

            koin.close()
        } else {
            log.warn("Koin was not initialized, skipping...")
        }

        log.warn("charted-server has completely shutdown, goodbye! ｡･ﾟﾟ･(థ Д థ。)･ﾟﾟ･｡")
    }
}
