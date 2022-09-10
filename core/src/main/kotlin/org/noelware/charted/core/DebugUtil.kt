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

package org.noelware.charted.core

import dev.floofy.utils.koin.injectOrNull
import org.noelware.charted.configuration.dsl.Config

/**
 * Represents utilities for checking if the server is in debugging, or in development
 * mode. You can configure this in the configuration by placing the `debug` variable to **true**:
 *
 * ```yaml
 * debug: true
 * ```
 *
 * or, you can use a system property with the `-Dorg.noelware.charted.debug.mode` set to `yes`, `true`,
 * `1`, or `si` and debug mode would be enabled:
 *
 * ```sh
 * CHARTED_JAVA_OPTS="-Dorg.noelware.charted.debug.mode=yes" ./bin/charted-server
 * ```
 *
 * You can also set `CHARTED_DEBUG` to `true` to get the same result.
 */
object DebugUtils {
    private const val SYSTEM_PROPERTY = "org.noelware.charted.debug.mode"
    private val _config: Config? by injectOrNull()

    /**
     * Returns if the server is running in debug mode. You can check the KDoc in the main
     * object class for more information.
     *
     * If Koin hasn't been initialized yet, it'll check only the system properties, otherwise
     * it'll check both the configuration and the system properties, or you can pass in
     * the configuration yourself.
     */
    fun isDebugEnabled(config: Config? = null): Boolean =
        (config ?: _config)?.debug == true || System.getProperty(SYSTEM_PROPERTY, "0").matches("^(yes|true|1|si|si*)$".toRegex())
}
