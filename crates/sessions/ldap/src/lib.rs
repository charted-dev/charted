// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use charted_config::ldap::Config;
use eyre::{Context, Result};
use ldap3::{Ldap, LdapConnAsync};
use std::sync::Arc;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct LDAPSessionProvider {
    ldap: Ldap,
    _handle: Arc<JoinHandle<()>>,
}

impl LDAPSessionProvider {
    pub async fn new(config: Config) -> Result<LDAPSessionProvider> {
        let (conn, ldap) = LdapConnAsync::new(&config.host)
            .await
            .context("unable to connect to ldap server")?;

        Ok(LDAPSessionProvider {
            _handle: Arc::new(ldap3::drive!(conn)),
            ldap,
        })
    }

    pub async fn terminate(mut self) -> Result<()> {
        self.ldap
            .unbind()
            .await
            .context("unable to terminate connection to ldap server")
    }
}
