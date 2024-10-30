// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

#![allow(unused)]

use charted_config::features::totp::Config;
use charted_core::BoxedFuture;
use charted_features::Feature;

/// Creates the TOTP feature that is opaque as a [`Feature`].
pub fn new(config: &Config) -> impl Feature {
    TotpFeature {
        secret: config.secret.clone(),
    }
}

struct TotpFeature {
    secret: String,
}

impl Feature for TotpFeature {
    fn extends_db<'feat, 'a>(&'feat self, _pool: &'a charted_database::DbPool) -> BoxedFuture<'a, eyre::Result<()>>
    where
        'a: 'feat,
    {
        Box::pin(async move {
            let conn = _pool.get()?;

            Ok(())
        })
    }
}
