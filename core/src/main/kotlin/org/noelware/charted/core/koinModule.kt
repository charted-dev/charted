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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.core

import io.ktor.client.*
import io.ktor.client.engine.okhttp.*
import io.ktor.client.plugins.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.serialization.kotlinx.json.*
import io.sentry.Sentry
import kotlinx.serialization.json.Json
import org.koin.dsl.module
import org.noelware.charted.core.http.LoggingInterceptor
import org.noelware.charted.core.http.SentryInterceptor

val chartedModule = module {
    single {
        Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }
    }

    single {
        val httpClient = HttpClient(OkHttp) {
            engine {
                config {
                    followRedirects(true)
                }

                addInterceptor(LoggingInterceptor)

                if (Sentry.isEnabled()) {
                    addInterceptor(SentryInterceptor)
                }
            }

            install(ContentNegotiation) {
                json(get())
            }

            install(UserAgent) {
                agent = "Noelware/charted-server (v${ChartedInfo.version}; https://github.com/charted-dev/charted)"
            }
        }
    }
}

/*
        val httpClient = HttpClient(OkHttp) {
            engine {
                config {
                    followRedirects(true)
                    addInterceptor(LogInterceptor())

                    if (Sentry.isEnabled()) {
                        addInterceptor(SentryInterceptor())
                    }
                }
            }

            install(WebSockets)

            install(ContentNegotiation) {
                this.json(json)
            }

            install(UserAgent) {
                agent = "Nino/DiscordBot (+https://github.com/NinoDiscord/Nino; v${NinoInfo.VERSION})"
            }
        }
 */
