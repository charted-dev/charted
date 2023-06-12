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

package org.noelware.charted.models

import io.swagger.v3.oas.annotations.media.Schema
import io.swagger.v3.oas.models.media.IntegerSchema
import org.noelware.charted.snowflake.Snowflake

public class SnowflakeSchema: IntegerSchema() {
    init {
        description("A unique identifier that points to a User, Repository, or Organization resource")
        minLength(15)
        format("int64")
    }
}

/**
 * Union discriminated type that can resolve Snowflake parameters (valid [Long], int64 values),
 * or a [Name] that can safely point to a user, organization, or repository.
 */
@Schema(
    description = "Union discriminated type that can resolve a valid `Snowflake`, or a `Name` that can safely point to a user, organization, or repository",
    oneOf = [Snowflake::class, Name::class],
)
public object NameOrSnowflake
