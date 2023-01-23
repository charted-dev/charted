package org.noelware.charted.server.openapi.extensions

import guru.zoroark.tegral.openapi.dsl.OperationDsl
import guru.zoroark.tegral.openapi.dsl.schema
import org.noelware.charted.types.responses.ApiResponse

fun OperationDsl.addSessionResponses(exclude: List<Int> = listOf()) {
    if (!exclude.contains(400)) {
        400 response {
            description = "If the session token or API key couldn't be validated or if the Authorization header was malformed"
            "application/json" content {
                schema<ApiResponse.Err>()
            }
        }
    }

    if (!exclude.contains(401)) {
        401 response {
            description = "If the request couldn't be authorized to perform this action due to an expired access/refresh session token, or an invalid password (in Basic authentication)"
            "application/json" content {
                schema<ApiResponse.Err>()
            }
        }
    }

    if (!exclude.contains(403)) {
        403 response {
            description = """
            |• If the given API key has missing scopes that this route requires,
            |• If the username and password didn't match in Basic authentication,
            |• If there was no `Authorization` header present in the request
            """.trimMargin("|")

            "application/json" content {
                schema<ApiResponse.Err>()
            }
        }
    }

    if (!exclude.contains(406)) {
        406 response {
            description = """If the Authorization request header couldn't be accepted due to:
            • If the header wasn't formed as base64 encoded 'username:password' (in Basic authentication),
            • Unknown JWT exception had occurred (in Session authentication),
            • The request header didn't follow the '[Type] [Token]' scheme
                • `Type` is "Basic", "ApiKey", or "Bearer"
                • `Token` is the actual token or base64-encoded of 'username:password' if `Type` is Basic
            """.trimIndent()

            "application/json" content {
                schema<ApiResponse.Err>()
            }
        }
    }

    500 response {
        description = "Internal Server Error (it can happen)"
        "application/json" content {
            schema<ApiResponse.Err>()
        }
    }
}
