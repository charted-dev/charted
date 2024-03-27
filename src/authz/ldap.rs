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

use charted_config::sessions::ldap::Config;
use charted_entities::User;
use futures_util::Future;
use ldap3::LdapConnSettings;
use reqwest::Url;

#[derive(Clone)]
pub struct Backend {
    config: Config,
}

impl Backend {
    pub fn new(config: Config) -> Backend {
        Backend { config }
    }

    /// Spawns a new LDAP connection to the server and spawns the Tokio task associated with it,
    /// does the stuff you need in the function body, and destroys the connection altogether.
    pub async fn new_connection<F, Fut, Res, Err: From<ldap3::LdapError>>(&self, fn_: F) -> Result<Res, Err>
    where
        F: FnOnce(ldap3::Ldap) -> Fut + Send,
        Fut: Future<Output = eyre::Result<Res, Err>> + Send,
    {
        let (conn, ldap) = ldap3::LdapConnAsync::from_url_with_settings(
            LdapConnSettings::new()
                .set_conn_timeout(*self.config.conn_timeout)
                .set_starttls(self.config.starttls)
                .set_no_tls_verify(self.config.insecure_skip_tls_verify),
            &Url::parse(self.config.host.as_str()).unwrap(),
        )
        .await?;

        tokio::spawn(async move {
            if let Err(e) = conn.drive().await {
                error!(error = %e, "unable to drive LDAP connection to completion");
            }
        });

        fn_(ldap).await
    }
}

#[async_trait]
impl super::Backend for Backend {
    async fn authenticate(&self, user: User, password: String) -> Result<(), super::Error> {
        let bind_dn = self.config.bind_dn.clone();

        self.new_connection(|mut conn| async move {
            match conn
                .simple_bind(bind_dn.replace("%u", &user.username).as_str(), password.as_str())
                .await
            {
                Ok(res) if res.rc == 0 => Ok(()),
                Ok(res) if res.rc == 6 => Err(super::Error::InvalidPassword),
                Ok(res) => Err(eyre!("received unexpected error from LDAP service: {res:?}").into()),
                Err(e) => Err(e.into()),
            }
        })
        .await
    }
}
