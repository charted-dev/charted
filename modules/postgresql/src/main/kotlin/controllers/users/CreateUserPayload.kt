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

package org.noelware.charted.modules.postgresql.controllers.users

import dev.floofy.utils.koin.inject
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.serialization.Serializable
import org.apache.commons.validator.routines.EmailValidator
import org.noelware.charted.StringOverflowException
import org.noelware.charted.StringUnderflowException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.extensions.regexp.matchesNameRegex
import org.noelware.charted.common.extensions.regexp.matchesPasswordRegex
import org.noelware.charted.models.Name

@Schema(description = "Payload object to create a new User resource")
@Serializable
data class CreateUserPayload(
    @get:Schema(description = "Password for logging in, if the server is configured to use the local session manager, this is a required field.", pattern = "^.*(?=.{8,})(?=.*[a-zA-Z])(?=.*\\d)?(?=.*[!#\$%&? \"])?.*\$")
    val password: String? = null,

    @get:Schema(description = "Username to identify this user", implementation = Name::class)
    val username: String,

    @get:Schema(description = "Email address to identify this user")
    val email: String
) {
    init {
        val validator: EmailValidator by inject()
        if (!validator.isValid(email)) {
            throw ValidationException("body.email", "Email [$email] was not a valid email.")
        }

        if (username.isBlank()) {
            throw StringUnderflowException("body.username", 0, 32)
        }

        if (username.length > 32) {
            throw StringOverflowException("body.username", 32, username.length)
        }

        if (!username.matchesNameRegex()) {
            throw ValidationException("body.username", "Username can only contain letters, digits, dashes, or underscores.")
        }

        if (password != null && !password.matchesPasswordRegex()) {
            throw ValidationException("body.password", "Password can only contain letters, digits, and special characters.")
        }
    }
}
