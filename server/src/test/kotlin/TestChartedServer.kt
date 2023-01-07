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

package org.noelware.charted.server.testing

import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.testing.*
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.server.ChartedServer
import org.noelware.charted.server.hasStarted

typealias ServerTestFunction = suspend ApplicationTestBuilder.() -> Unit

/**
 * Creates a new [TestChartedServer] instance with the given [config dsl builder][config] and the test
 * function to run the server in.
 *
 * It is recommended to use the [AbstractChartedServerTest.withServerTest] function so the [config dsl builder][config]
 * can be easily populated with required services.
 *
 * ## Example
 * ```kt
 * @Test
 * fun `do things`(): Unit = TestChartedServer({
 *   /* config dsl builder thing here */
 * }) {
 *   /* actual server tests here */
 * }
 * ```
 */
fun TestChartedServer(config: Config.Builder.() -> Unit = {}, testFunction: ServerTestFunction): TestChartedServer =
    TestChartedServer(Config.Builder().apply(config).build(), testFunction)

class TestChartedServer(
    private val config: Config,
    val testFunction: ServerTestFunction = {}
): ChartedServer {
    override val server: ApplicationEngine
        get() = throw IllegalStateException("Server tests don't expose `ApplicationEngine`")

    override val started: Boolean
        get() = hasStarted.get()

    override fun Application.module() {
    }

    override fun start() {
    }

    override fun close() {
    }
}
