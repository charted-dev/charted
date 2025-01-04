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

use charted_authz::{Authenticator, InvalidPassword};
use charted_config::sessions::ldap::Config;
use charted_core::BoxedFuture;
use charted_types::User;
use eyre::eyre;
use ldap3::{Ldap, LdapConnAsync, LdapConnSettings};
use std::future::Future;
use tracing::error;
use url::Url;

#[derive(Clone)]
pub struct Backend {
    config: Config,
}

impl Backend {
    pub fn new(config: Config) -> Backend {
        Backend { config }
    }

    fn ldap_settings(&self) -> LdapConnSettings {
        LdapConnSettings::new()
            .set_conn_timeout(*self.config.conn_timeout)
            .set_starttls(self.config.starttls)
            .set_no_tls_verify(self.config.insecure_skip_tls_verify)
    }

    /// Spawns a new LDAP connection to the server and spawns the Tokio task associated with it,
    /// does the stuff you need in the function body, and destroys the connection altogether.
    pub async fn conn<F, Fut, Res, Err>(&self, f: F) -> Result<Res, Err>
    where
        F: FnOnce(Ldap) -> Fut + Send,
        Fut: Future<Output = Result<Res, Err>>,
        Err: From<ldap3::LdapError>,
    {
        let (conn, ldap) =
            LdapConnAsync::from_url_with_settings(self.ldap_settings(), &Url::parse(&self.config.host).unwrap())
                .await?;

        tokio::spawn(async move {
            if let Err(e) = conn.drive().await {
                error!(error = %e, "failed to drive LDAP connection to completion");
                sentry::capture_error(&e);
            }
        });

        f(ldap).await
    }
}

impl Authenticator for Backend {
    fn authenticate<'u>(&'u self, user: &'u User, password: String) -> BoxedFuture<'u, eyre::Result<()>> {
        Box::pin(async move {
            let bind_dn = self.config.bind_dn.clone();
            self.conn(|mut conn| async move {
                match conn
                    .simple_bind(bind_dn.replace("%u", &user.username).as_str(), password.as_str())
                    .await
                {
                    Ok(res) if res.rc == 0 => Ok(()),
                    Ok(res) if res.rc == 6 => Err(InvalidPassword.into()),
                    Ok(res) => Err(eyre!("received unexpected error from LDAP service: {res:?}")),
                    Err(e) => Err(e.into()),
                }
            })
            .await
        })
    }
}
