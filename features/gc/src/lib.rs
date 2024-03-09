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

mod action;
mod actions;
mod lexer;
mod parser;

/// Represents the garbage collection feature, which allows the API server
/// to create a cron scheduler where its purpose is to clean unneccessary
/// objects from the datastore or from the main database.
///
/// ## Why?
///
/// As the server grows and you use it more, there is unnecessary Helm charts, users, organizations, etc that might keep using more disk. The way is to manually remove them yourself, but that can be a tedious task!
///
/// This implements:
///
/// * REST handler to run the garbage collector and grab metrics about previous iterations (i.e, total disk saved, etc.)
/// * Command line interface (`charted gc`) to run and grab metrics.
///
/// ## Configuration
///
/// This can be configured in `./config/charted.toml` with the `[features.gc]` TOML table:
///
/// ```toml filename=./config/charted.toml
/// [features.gc]
/// cron = "@daily" # runs at 00:00 - this is the base cron schedule, it'll be the default if none were specified.
///
/// # Specify a constraint that the garbage collector will use to determine
/// # how a entity should be garbage collected.
/// [[features.gc.constraint]]
/// entity = "Repository"
/// constraint = "updated_at >= 30d"
/// description = "Delete all repositories that haven't been updated in 30 days"
/// actions = [
///     # delete it from the database
///     "delete",
///
///     # send the email to the owner and the team members
///     "email"
/// ]
/// ```
///
/// The garbage collector will:
///
/// * Run a cron job (specified in `gc.cron` or `gc.constraint[].cron`) to check if the `constraint` is true, then will run the following actions:
///   * Deletes the repository from the database
///   * Sends a email that a repository was deleted (if the emails service is enabled, this will be nop if not enabled)
///
/// ## Using via `charted` crate
/// You can get the garbage collection feature if it was ever enabled:
/// ```rust,ignore
/// # use charted::Instance;
/// #
/// let instance = Instance::get();
/// if instance.features.enabled::<GarbageCollection>() {
///     let gc = instance.features.downcast::<GarbageCollection>().unwrap();
///     // do whatever
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GarbageCollection;
