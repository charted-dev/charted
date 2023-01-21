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

use std::{
    fs::{create_dir, File},
    io::ErrorKind,
    path::{Path, PathBuf},
};

use std::io::Write as _;

use serde_json::Value;

use crate::{api::client::Client, error::Error, keychain::Keychain};

pub const DEFAULT_SERVER_URL: &str = "https://charts.noelware.org";

/// Represents all the global settings that are available to all commands. Though, this is only
/// contains relevant settings that most commands will use.
#[derive(Debug, Clone)]
pub struct Settings {
    server_url: String,
    keychain: Keychain,
    client: Client,
}

impl Settings {
    pub fn new(verbose: bool, server_url: Option<String>) -> Result<Settings, Error> {
        let url = server_url
            .as_ref()
            .unwrap_or(&DEFAULT_SERVER_URL.to_string())
            .to_string();

        Ok(Settings {
            server_url: url.clone(),
            keychain: Keychain::new(url.clone()).map_err(|e| Error::Unknown(Box::new(e)))?,
            client: Client::new(&url, verbose),
        })
    }

    pub fn server(&self) -> &String {
        &self.server_url
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn keychain(&self) -> &Keychain {
        &self.keychain
    }

    pub fn get_unencrypted_key(&self) -> Result<Option<String>, Error> {
        match self.keychain.get_api_key() {
            Ok(Some(key)) => Ok(Some(key)),
            Ok(None) => {
                let servers_file = self.servers_file()?;
                let contents: Value = serde_json::from_reader(&servers_file)?;

                let unencrypted = &contents[self.server_url.clone()];
                if unencrypted.is_null() {
                    return Ok(None);
                }

                Ok(Some(unencrypted.as_str().unwrap().to_string()))
            }

            Err(e) => Err(e),
        }
    }

    fn servers_file(&self) -> Result<File, Error> {
        let data_dir = dirs::data_dir().unwrap();
        let charted_data_dir = {
            let mut p = PathBuf::new();
            p.push(data_dir);
            p.push(format!("{}charted", std::path::MAIN_SEPARATOR_STR));

            p
        };

        let servers_path = format!(
            "{}{}servers.yaml",
            charted_data_dir.display(),
            std::path::MAIN_SEPARATOR_STR
        );

        match File::open(servers_path.clone()) {
            Ok(file) => Ok(file),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    create_dir(charted_data_dir).map_err(Error::IoError)?;

                    let path = Path::new(&servers_path);
                    if !path.exists() {
                        let mut file = File::create_new(servers_path.clone())?;
                        writeln!(file, "{}", format!("{}: null", self.server_url))?;

                        Ok(file)
                    } else {
                        Err(Error::IoError(std::io::Error::new(
                            ErrorKind::Other,
                            "path exists but dir didn't?",
                        )))
                    }
                } else {
                    Err(Error::IoError(e))
                }
            }
        }
    }
}
