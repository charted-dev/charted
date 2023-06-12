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

import io.swagger.v3.oas.models.media.StringSchema

public class NameSchema: StringSchema() {
    init {
        description("Valid UTF-8 string that is used to point to a user, repository, or organization resource. Mainly used for `idOrName` path parameters in REST controllers to help identify which resource to locate from a valid `Snowflake` identifier, or a Name to point to a resource.")
        pattern("^([A-z]|-|_|\\d{0,9}){0,32}")
    }
}

/**
 * Represents a [Name], that is used to point to a user, repository, or organization
 * entity. This is mainly used for the `{idOrName}` path parameter that REST controllers
 * used to safely identify a resource from its name or Snowflake counterpart.
 */
public object Name
