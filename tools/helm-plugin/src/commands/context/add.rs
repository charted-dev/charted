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

use url::Url;

use crate::auth::{Auth, Context, EnvVarKind, Registry, Type};
use std::{
    io::{self, IsTerminal, Read},
    path::PathBuf,
    process::exit,
};

/// Adds a context to the `auth.yaml` configuration
#[derive(Debug, Clone, clap::Parser)]
pub struct Cmd {
    /// name of the context identifier
    context: Context,

    /// url to the registry this context points to
    registry: Url,

    /// authentication type to provide
    ty: Option<Type>,

    /// Reads the contents from the standard input buffer rather than asking prompts.
    #[arg(long, short = 'x')]
    stdin: bool,

    /// Location to an `auth.yaml` file that represents the authentication file
    /// to authenticate between charted instances
    ///
    /// ## Default Locations
    /// | OS               | Location                                                                                                  |
    /// | :--------------- | :-------------------------------------------------------------------------------------------------------- |
    /// | Windows          | `C:\Users\<username>\AppData\Local\Noelware\charted-server\auth.yaml`                                     |
    /// | macOS            | `/Users/<username>/Library/Application Support/Noelware/charted-server/auth.yaml`                         |
    /// | Linux            | `$XDG_CONFIG_DIR/Noelware/charted-server/auth.yaml` or `$HOME/.config/Noelware/charted-server/auth.yaml` |
    #[arg(long, short = 'a', env = "CHARTED_AUTH_YAML_LOCATION")]
    auth: Option<PathBuf>,
}

pub fn run(
    Cmd {
        context,
        ty,
        auth: auth_file,
        registry,
        stdin,
    }: Cmd,
) -> eyre::Result<()> {
    let mut auth = Auth::load(auth_file.as_ref())?;
    if auth.contexts.contains_key(&context) {
        error!("not adding existing context `{context}`");
        exit(1);
    }

    let registry = match ty.unwrap_or_default() {
        Type::None => Registry {
            registry,
            auth: Type::None,
        },

        Type::Basic { .. } => {
            if !in_tty() {
                if !stdin {
                    error!("please use `--stdin`/`-x` in a no TTY context");
                    exit(1);
                }

                let mut buf = String::new();
                io::stdin().lock().read_to_string(&mut buf)?;

                if let Some((username, password)) = buf.split_once(':') {
                    if password.contains(':') {
                        error!("expected no more than one colon in buffer [{buf}]");
                        exit(1);
                    }

                    Registry {
                        registry: registry.clone(),
                        auth: Type::Basic {
                            username: username.to_owned(),
                            password: password.to_owned(),
                        },
                    }
                } else {
                    error!("expected 'username:password' for Basic authentication, received buffer: {buf}");
                    exit(1);
                };
            }

            let username = inquire::prompt_text("Enter username")?;
            let password = inquire::prompt_secret("Enter password")?;

            Registry {
                registry,
                auth: Type::Basic { username, password },
            }
        }

        Type::ApiKey(_) => {
            if !in_tty() {
                if !stdin {
                    error!("please use `--stdin`/`-x` in a no TTY context");
                    exit(1);
                }

                let mut buf = String::new();
                io::stdin().lock().read_to_string(&mut buf)?;

                Registry {
                    registry,
                    auth: Type::ApiKey(buf),
                }
            } else {
                Registry {
                    registry,
                    auth: Type::ApiKey(inquire::prompt_secret("Enter API key")?),
                }
            }
        }

        Type::EnvironmentVariable { .. } => {
            if !in_tty() {
                if !stdin {
                    error!("please use `--stdin`/`-x` in a no TTY context");
                    exit(1);
                }

                let mut buf = String::new();
                io::stdin().lock().read_to_string(&mut buf)?;

                if let Some((ty, value)) = buf.split_once(':') {
                    if value.contains(':') {
                        error!("expected no more than one colon in buffer [{buf}]");
                        exit(1);
                    }

                    let kind = match ty {
                        "apikey" => EnvVarKind::ApiKey,
                        "bearer" => EnvVarKind::Bearer,
                        s => {
                            error!("unexpected value: [{s}], wanted [apikey/bearer]");
                            exit(1);
                        }
                    };

                    Registry {
                        registry: registry.clone(),
                        auth: Type::EnvironmentVariable {
                            kind,
                            env: value.to_owned(),
                        },
                    }
                } else {
                    error!("expected '[apikey or bearer]:name' for environment variable authentication, received buffer: {buf}");
                    exit(1);
                }
            } else {
                Registry {
                    registry,
                    auth: Type::EnvironmentVariable {
                        kind: match inquire::prompt_text(
                            "Enter what authentication type it should use (bearer/apikey):",
                        )?
                        .as_str()
                        {
                            "bearer" => EnvVarKind::Bearer,
                            "apikey" => EnvVarKind::ApiKey,
                            s => return Err(eyre!("expected 'bearer' or 'apikey', received {s}")),
                        },

                        env: inquire::prompt_text("Enter the environment variable name to use")?,
                    },
                }
            }
        }

        Type::Session { .. } => {
            if !in_tty() {
                if !stdin {
                    error!("please use `--stdin`/`-x` in a no TTY context");
                    exit(1);
                }

                let mut buf = String::new();
                io::stdin().lock().read_to_string(&mut buf)?;

                Registry {
                    registry,
                    auth: Type::Session {
                        access: buf,
                        refresh: None,
                    },
                }
            } else {
                Registry {
                    registry,
                    auth: Type::Session {
                        access: inquire::prompt_secret("Enter API key:")?,
                        refresh: None,
                    },
                }
            }
        }
    };

    auth.contexts.insert(context, registry);
    auth.sync(auth_file.as_ref())?;

    Ok(())
}

// it is used but clippy is dumb ...sometimes
#[allow(dead_code)]
fn in_tty() -> bool {
    io::stderr().is_terminal()
}
