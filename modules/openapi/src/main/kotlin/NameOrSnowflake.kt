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

package org.noelware.charted.modules.openapi

import io.swagger.v3.oas.annotations.media.Schema

/**
 * Represents a dummy class to use for resolving "idOrName" parameters
 * with this class.
 */
@Schema(
    description = "Represents a value that handles Name and Snowflake parameters",
    oneOf = [Name::class, Long::class],
)
@Deprecated("Replaced with [org.noelware.charted.modules.NameOrSnowflake]", replaceWith = ReplaceWith("NameOrSnowflake", "org.noelware.charted.models.NameOrSnowflake"))
class NameOrSnowflake private constructor()

/**
 * Dummy object to represent a Name that is a valid user, repository, or organization
 * name to be queried by the API server.
 */
@Deprecated("Replaced with [org.noelware.charted.models.Name]", replaceWith = ReplaceWith("Name", "org.noelware.charted.models.Name"))
@Schema(
    description = "Schema to resolve a valid user, repository, or organization successfully.",
    implementation = String::class,
    pattern = """^([A-z]|-|_|\d{0,9}){0,32}""",
)
object Name
