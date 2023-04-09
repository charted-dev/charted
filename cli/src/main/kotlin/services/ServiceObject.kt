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

package org.noelware.charted.cli.services

import java.io.File

/**
 * Represents an object to handle service information.
 */
interface ServiceObject {
    /**
     * Information about a given process, if it has started or `null`
     * if it hasn't been started
     */
    val info: ProcessHandle.Info?

    /**
     * Service name
     */
    val name: String

    /**
     * Downloads this [service object][ServiceObject] into the specified
     * [path].
     *
     * @param path The path to save it to
     * @return [Boolean] to indicate if it was downloaded successfully.
     */
    fun download(path: File): Boolean
}

/**
 * Checks if this [service object][ServiceObject] has already started
 * or not by the charted CLI.
 */
val ServiceObject.wasStarted: Boolean
    get() = info != null
