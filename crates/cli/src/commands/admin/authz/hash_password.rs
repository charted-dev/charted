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

use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use charted_core::ARGON2;
use eyre::bail;
use std::io::{self, BufRead};
use tracing::warn;

/// Generates a Argon2 password that is compatible with the `static` authentication backend.
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// The password to hash (can be empty if `-x`/`--stdin` is provided)
    password: Option<String>,

    /// get the password from standard input
    #[arg(long, short = 'x')]
    stdin: bool,
}

pub fn run(Args { password, stdin }: Args) -> eyre::Result<()> {
    if !stdin && password.is_none() {
        bail!("`password` field is missing");
    }

    if stdin && password.is_some() {
        warn!("password field will be skipped due to `--stdin`/`-x` being passed in");
    }

    let password = match (stdin, password) {
        (true, _) => {
            let mut line = String::new();
            let mut stdin = io::stdin().lock();

            stdin.read_line(&mut line)?;
            line.trim().to_owned()
        }

        (false, Some(password)) => password,
        (false, None) => unreachable!(),
    };

    let salt = SaltString::generate(&mut OsRng);
    let hash = match ARGON2.hash_password(password.as_bytes(), &salt) {
        Ok(v) => v,
        Err(err) => bail!("failed to hash password: {}", err),
    };

    println!("{hash}");
    Ok(())
}
