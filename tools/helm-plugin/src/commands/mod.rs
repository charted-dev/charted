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

pub mod login;
pub mod logout;
pub mod me;
pub mod push;
pub mod version;

use clap::Subcommand;

use crate::commands::version::*;
use crate::error::Error;
use crate::settings::Settings;

use self::me::Me;

pub trait Execute {
    fn execute(self, settings: &Settings) -> Result<(), Error>;
}

#[async_trait]
pub trait AsyncExecute {
    async fn execute(self, settings: &Settings) -> Result<(), Error>;
}

#[derive(Debug, Clone, Subcommand)]
pub enum Subcommands {
    Version(Version),
    Me(Me),
}

pub async fn execute(subcommand: &Subcommands, settings: &Settings) -> Result<(), Error> {
    match subcommand {
        Subcommands::Version(version) => version.execute(settings).await,
        Subcommands::Me(me) => me.execute(settings).await,
    }
}
