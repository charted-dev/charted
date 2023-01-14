// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022-2023 Noelware <team@noelware.org>
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

use std::sync::Arc;

use keyring::Entry;
use url::Url;

use crate::error::Error;

/// Represents an abstraction over the OS-dependent keychain that is backed by the
/// [`keyring`] crate.
#[derive(Debug, Clone)]
pub struct Keychain {
    inner: Arc<Entry>,
}

impl Keychain {
    pub fn new(server_url: String) -> Result<Keychain, Error> {
        let username = whoami::username();
        let url = Url::parse(&server_url).map_err(|e| Error::Unknown(Box::new(e)))?;

        let host = url
            .host()
            .ok_or(Error::UnableToDetermineURLHost)?
            .to_string();

        debug!("built keychain for target '{}:api:key'", host.clone());
        Ok(Keychain {
            inner: Arc::new(Entry::new_with_target(
                &format!("{}:api:key", host).to_string(),
                "charted-server",
                &username,
            )),
        })
    }

    pub fn get_api_key(&self) -> Result<Option<String>, Error> {
        match self.inner.get_password() {
            Ok(result) => Ok(Some(result)),
            Err(err) => match err {
                keyring::Error::NoEntry => Ok(None),
                _ => Err(Error::Unknown(Box::new(err))),
            },
        }
    }

    pub fn destroy_key(&self) -> Result<(), Error> {
        self.inner
            .delete_password()
            .map_err(|e| Error::Unknown(Box::new(e)))
    }
}
