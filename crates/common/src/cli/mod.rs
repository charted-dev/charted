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

use async_trait::async_trait;
use eyre::Result;

/// Generic trait to implement when implementing CLI logic for commands. This
/// is an indicator that this command is only synchronous.
pub trait Execute {
    fn execute(&self) -> Result<()>;
}

/// Generic trait to implement for asynchronous execution of the logic of CLI commands.
#[async_trait]
pub trait AsyncExecute {
    async fn execute(&self) -> Result<()>;
}
