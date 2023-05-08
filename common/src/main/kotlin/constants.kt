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

/**
 * The epoch (in milliseconds) for the Snowflake generator, it will always
 * be March 1st, 2023
 */
public const val SNOWFLAKE_EPOCH: Long = 1677654000000

/**
 * Regular expression to evaluate booleans with human-input like `yes`.
 */
public val TRUE_BOOL_REGEX: Regex = "^(yes|true|1|enabled|enable|si*)$".toRegex()

/**
 * Checks if we are in debug-mode or not.
 *
 * Priority is:
 *  - `CHARTED_DEBUG` environment variable
 *  - `-Dorg.noelware.charted.debug` system property
 */
public fun isDebugEnabled(): Boolean {
    val environmentVariable = System.getenv("CHARTED_DEBUG")
    if (environmentVariable != null) return environmentVariable matches TRUE_BOOL_REGEX

    val systemProperty: String = System.getProperty("org.noelware.charted.debug", "false")
    return systemProperty matches TRUE_BOOL_REGEX
}
