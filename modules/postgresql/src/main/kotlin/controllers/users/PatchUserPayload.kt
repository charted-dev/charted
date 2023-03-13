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

import com.fasterxml.jackson.annotation.JsonProperty
import dev.floofy.utils.koin.inject
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.apache.commons.validator.routines.EmailValidator
import org.noelware.charted.StringOverflowException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.extensions.regexp.matchesNameAndIdRegex
import org.noelware.charted.common.extensions.regexp.matchesPasswordRegex

@Serializable
data class PatchUserPayload(
    @JsonProperty("gravatar_email")
    @SerialName("gravatar_email")
    val gravatarEmail: String? = null,
    val description: String? = null,
    val username: String? = null,
    val password: String? = null,
    val email: String? = null,
    val name: String? = null
) {
    init {
        val emailValidator: EmailValidator by inject()
        if (gravatarEmail != null && !emailValidator.isValid(gravatarEmail)) {
            throw ValidationException("body.gravatar_email", "The gravatar email provided was not a valid email.")
        }

        if (description != null && description.length > 240) {
            throw StringOverflowException("body.description", description.length, 240)
        }

        if (password != null && !password.matchesPasswordRegex()) {
            throw ValidationException("body.password", "New user password can only contain letters, digits, and special characters.")
        }

        if (username != null) {
            if (username.length > 32) {
                throw StringOverflowException("body.username", username.length, 32)
            }

            if (!username.matchesNameAndIdRegex()) {
                throw ValidationException("body.username", "Username can only contain letters, digits, dashes, or underscores.")
            }
        }

        if (email != null && !emailValidator.isValid(email)) {
            throw ValidationException("body.email", "Email was not valid")
        }

        if (name != null && name.length > 64) {
            throw StringOverflowException("body.name", name.length, 64)
        }
    }
}
