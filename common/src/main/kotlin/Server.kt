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

package org.noelware.charted

import io.ktor.server.application.*
import java.io.Closeable

/**
 * Represents a blue-print of creating the API server for charted.
 */
public interface Server: Closeable {
    /**
     * Checks if the server has started or not.
     */
    public val started: Boolean

    /**
     * Extension function to tailor the application module for this [Server]
     * instance.
     */
    public fun Application.module()

    /**
     * Starts the server, this will be a no-op if [started] was already
     * set to `true`.
     */
    public fun start()
}