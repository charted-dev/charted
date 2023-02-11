/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.server.endpoints.v1.api.users

import dev.floofy.utils.koin.inject
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.apache.commons.validator.routines.EmailValidator
import org.noelware.charted.ChartedInfo
import org.noelware.charted.StringOverflowException
import org.noelware.charted.ValidationException
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.extensions.regexp.toPasswordRegex
import org.noelware.charted.types.helm.RepoType

@Serializable
data class NewUserBody(
    val username: String,
    val password: String,
    val email: String
) {
    init {
        val validator: EmailValidator by inject()
        if (!validator.isValid(email)) {
            throw ValidationException("body.email", "Email [$email] was not a valid email.")
        }

        if (username.length > 32) {
            throw StringOverflowException("body.username", 32)
        }

        if (!username.toNameRegex().matches()) {
            throw ValidationException("body.username", "Username can only contain letters, digits, dashes, or underscores.")
        }

        if (!password.toPasswordRegex().matches()) {
            throw ValidationException("body.password", "Password can only contain letters, digits, and special characters.")
        }
    }
}

@Serializable
data class LoginBody(
    val username: String? = null,
    val email: String? = null,
    val password: String
) {
    init {
        if (username == null && email == null) {
            throw ValidationException("body.username|email", "You must use `username` or `email` when logging in.")
        }

        if (username != null && email != null) {
            throw ValidationException("body.username|email", "`username` and `email` are mutually exclusive")
        }

        val validator: EmailValidator by inject()
        if (email != null && !validator.isValid(email)) {
            throw ValidationException("body.email", "Invalid email address.")
        }
    }
}

@Serializable
data class UpdateUserBody(
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
            throw StringOverflowException("body.description", 240)
        }

        if (password != null && !password.toPasswordRegex().matches()) {
            throw ValidationException("body.password", "New user password can only contain letters, digits, and special characters.")
        }

        if (username != null) {
            if (username.length > 32) {
                throw StringOverflowException("body.username", 32)
            }

            if (!username.toNameRegex().matches()) {
                throw ValidationException("body.username", "Username can only contain letters, digits, dashes, or underscores.")
            }
        }

        if (email != null && !emailValidator.isValid(email)) {
            throw ValidationException("body.email", "The email address you used was not a valid one.")
        }

        if (name != null && name.length > 64) {
            throw ValidationException("body.name", "Can't set display name with over 64 characters.")
        }
    }
}

@Serializable
data class MainUserResponse(
    val message: String = "Welcome to the Users API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/users"
)

@Serializable
data class CreateRepositoryBody(
    val description: String? = null,
    val private: Boolean = false,
    val name: String,
    val type: RepoType
) {
    init {
        if (description != null && description.length > 240) {
            throw StringOverflowException("body.description", 240)
        }

        if (name.length > 24) {
            throw StringOverflowException("body.name", 32)
        }

        if (!name.toNameRegex(true, 24).matches()) {
            throw ValidationException("body.name", "Repository name can only contain alphabet characters, digits, underscores, and dashes.")
        }
    }
}
