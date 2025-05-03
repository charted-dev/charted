// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Env;
use charted_types::Ulid;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};
use serde::Deserialize;

/// The `iss` field value.
///
/// <https://www.rfc-editor.org/rfc/rfc7519#section-4.1.1>
pub const ISS_VALUE: &str = "Noelware";

/// The `aud` field value.
///
/// <https://www.rfc-editor.org/rfc/rfc7519#section-4.1.3>
pub const AUD_VALUE: &str = "charted-server";

/// JWT algorithm to use.
pub const ALGORITHM: Algorithm = Algorithm::HS512;

/// JWT claim for the expiration.
///
/// <https://www.rfc-editor.org/rfc/rfc7519#section-4.1.4>
pub const EXP: &str = "exp";

/// JWT claim for the audience, which will always be [`JWT_AUD_VALUE`] if
/// it was issued by us.
///
/// <https://www.rfc-editor.org/rfc/rfc7519#section-4.1.3>
pub const AUD: &str = "aud";

/// JWT claim for the issuer, which will always be [`JWT_ISS_VALUE`] if it
/// was issued by us.
///
/// <https://www.rfc-editor.org/rfc/rfc7519#section-4.1.1>
pub const ISS: &str = "iss";

/// Claim name for a user ID.
pub const UID: &str = "uid";

/// Claim name for a session ID.
pub const SID: &str = "sid";

/// JWT claims that should be present.
#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub sid: Ulid,
    pub uid: Ulid,
}

/// Decodes a JWT token.
///
/// This will add extra validation to ensure that the JWT token was created by the server.
pub fn decode_jwt(env: &Env, token: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    let key = DecodingKey::from_secret(env.config.jwt_secret_key.as_ref());
    let validation = create_jwt_validation();

    decode(token, &key, &validation).inspect_err(|e| {
        error!(error = %e, "failed to decode JWT token");
        sentry::capture_error(e);
    })
}

/// Creates a [`Validation`] to add more validation towards JWT deconstruction.
fn create_jwt_validation() -> Validation {
    let mut validation = Validation::new(ALGORITHM);
    validation.set_issuer(&[ISS_VALUE]);
    validation.set_audience(&[AUD_VALUE]);
    validation.set_required_spec_claims(&[EXP, ISS, AUD, UID, SID]);

    validation
}
