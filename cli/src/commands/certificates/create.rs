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

use charted_common::cli::Execute;
use eyre::{Context, Result};
use std::path::PathBuf;

/// Generates a self-signed SSL certificate that charted-server can use
/// for TLS support.
#[derive(Debug, Clone, clap::Parser)]
pub struct Create {
    /// path to the certificates that it should be created in, by default,
    /// this will create them in `./certs`.
    #[arg(env = "CHARTED_SSL_CERTS")]
    ssl_dir: Option<PathBuf>,

    /// passphrase for the SSL certificate
    #[arg()]
    passphrase: Option<String>,
}

impl Execute for Create {
    fn execute(&self) -> Result<()> {
        let _certs = self.ssl_dir.clone().unwrap_or_else(|| PathBuf::from("./certs"));

        Ok(())
    }
}
