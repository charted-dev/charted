package org.noelware.charted.modules.sessions.openid

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class WellKnownOpenIDConfig(
    val issuer: String,

    @SerialName("authorization_endpoint")
    val authorizationEndpoint: String,

    @SerialName("token_endpoint")
    val tokenEndpoint: String,

    @SerialName("revocation_endpoint")
    val revocationEndpoint: String,

    @SerialName("introspection_endpoint")
    val introspectionEndpoint: String,

    @SerialName("userinfo_endpoint")
    val userInfoEndpoint: String,

    @SerialName("jwks_uri")
    val jwksUri: String,

    @SerialName("scopes_supported")
    val supportedScopes: List<String>,

    @SerialName("response_types_supported")
    val supportedResponseTypes: List<String>,

    @SerialName("response_modes_supported")
    val supportedResponseModes: List<String>,

    @SerialName("grant_types_supported")
    val supportedGrantTypes: List<String>,

    @SerialName("token_endpoint_auth_methods_supported")
    val supportedTokenEndpointAuthMethods: List<String>,

    @SerialName("subject_types_supported")
    val supportedSubjectTypes: List<String>,

    @SerialName("id_token_signing_alg_values_supported")
    val supportedIdTokenSigningAlgorithmValues: List<String>,

    @SerialName("claim_types_supported")
    val supportedClaimTypes: List<String>,

    @SerialName("claims_supported")
    val supportedClaims: List<String>,

    @SerialName("code_challenge_methods_supported")
    val supportedCodeChallengeMethods: List<String>
)
