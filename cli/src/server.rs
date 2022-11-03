// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022 Noelware <team@noelware.org>
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
    borrow::Cow,
    env::var,
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
};

use anyhow::{anyhow, Result};
use tokio::process::{Child, Command};
use which::which;

#[allow(dead_code)]
const CHARTED_LOGBACK_PATH_ENV: &str = "CHARTED_LOGBACK_CONFIG_FILE";

#[allow(dead_code)]
const WINTERFOX_DEDI_NODE_ENV: &str = "WINTERFOX_DEDI_NODE";

#[allow(dead_code)]
const CHARTED_JAVA_OPTS_ENV: &str = "CHARTED_JAVA_OPTS";

/// Represents the server process that is executed when you run `charted server`. This helps
/// keep the lifecycle of the server running until you hit CTRL+C or whenever the server has
/// exited on its own.
///
/// This will require you to have a Java installation in $PATH or in JAVA_HOME since the process
/// will try to determine what Java installation to use and bootstrap the server.
///
/// ```no_run
/// use charted_cli::server::ServerProcess;
/// use anyhow::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///    // This will create a instance of `ServerProcess` where it will only create
///    // the channel to wait for exit statuses and such. You will need to use
///    // proc.start(); to start the process alone.
///    let proc = ServerProcess::create();
///    proc.start()?;
///
///    // This will block the main thread until the process has been exited
///    // or cancelled by the OS, or whatever. Now, we can do a teardown to
///    // release the allocated resources.
///    proc.teardown()?
/// }
/// ```
#[derive(Debug)]
pub struct ServerProcess {
    #[allow(dead_code)]
    child: Child,
}

impl ServerProcess {
    /// Creates a new managed [`ServerProcess`].
    pub fn create() -> Result<ServerProcess> {
        trace!("creating server process...");

        if !Path::new("./libs").exists() {
            return Err(anyhow!("Server installation is corrupted or not a valid installation! This installation \
            is currently missing a \"libs\" directories where all the libraries required for charted-server to run \
            live at. Please download an official build from Noelware's Artifact Registry: \
                -> https://noelware.cloud/download/charted/server \
            This could also be taken into effect if you're running the CLI in the 'bin/' directory, which \
            you should probably do `cd .. && ./bin/charted server` instead. :)"));
        }

        let distribution = match var("CHARTED_DISTRIBUTION") {
            Ok(p) => p,
            Err(_) => "local".into(),
        };

        info!("Using charted-server distribution [{}]", distribution);

        let props = ServerProcess::get_java_properties();
        trace!("resolved properties => {}", props.join(" "));

        let java_dist = ServerProcess::find_java_install()?;
        let proc = Command::new(java_dist)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .args(props)
            .arg(format!(
                "-Dorg.noelware.charted.distribution.type={}",
                distribution
            ))
            .arg("-cp")
            .arg(Path::new("./libs"))
            .arg("org.noelware.charted.server.Bootstrap")
            .spawn()?;

        debug!("started process with id {}", proc.id().unwrap_or(0));
        Ok(ServerProcess { child: proc })
    }

    pub async fn wait(mut self) -> Result<ExitStatus, std::io::Error> {
        self.child.wait().await
    }

    fn get_java_properties() -> Vec<String> {
        let mut options = vec![
            Cow::<String>::Owned("-XX:+HeapDumpOnOutOfMemoryError".to_string()),
            Cow::<String>::Owned("-XX:+ExitOnOutOfMemoryError".to_string()),
            Cow::<String>::Owned("-XX:ErrorFile=logs/hs_err_pid%p.log".to_string()),
            Cow::<String>::Owned("-XX:SurvivorRatio=8".to_string()),
            Cow::<String>::Owned("-Dfile.encoding=UTF-8".to_string()),
            Cow::<String>::Owned("-Djava.awt.headless=true".to_string()),
        ];

        let logback_path = Path::new("./config/logback.properties");
        if logback_path.exists() {
            let prop = format!(
                "-Dorg.noelware.charted.logback.config={}",
                logback_path.display()
            );

            options.push(Cow::Owned(prop));
        }

        if let Ok(env) = var(CHARTED_LOGBACK_PATH_ENV) {
            // If the path actually exists, let's just append it!
            if Path::new(&env).exists() {
                let prop = format!("-Dorg.noelware.charted.logback.config={}", env);
                options.push(Cow::Owned(prop));
            }
        }

        if let Ok(env) = var(WINTERFOX_DEDI_NODE_ENV) {
            let prop = format!("-Pwinterfox.dediNode={}", env);
            options.push(Cow::Owned(prop));
        }

        if let Ok(env) = var(CHARTED_JAVA_OPTS_ENV) {
            for value in env.split(' ') {
                options.push(Cow::Owned(value.to_string()));
            }
        }

        options
            .into_iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
    }

    fn find_java_install() -> Result<String> {
        trace!("finding java installation...");

        #[allow(unused_assignments)]
        let mut java: String = "".into();
        if let Ok(x) = var("JAVA_HOME") {
            trace!("finding java home via 'JAVA_HOME' environment variable...");

            // Check if `jre/sh/java` exists in JAVA_HOME
            #[allow(unused_assignments)]
            let mut java_path = PathBuf::new();
            if Path::new(&format!("{}/jre/sh/java", x)).exists() {
                trace!("found /jre/sh/java binary in JAVA_HOME! using...");
                java_path = Path::new(&format!("{}/jre/sh/java", x)).to_path_buf();
            } else {
                trace!("fallback to /bin/java in JAVA_HOME");
                java_path = Path::new(&format!("{}/bin/java", x)).to_path_buf();
            }

            if !java_path.exists() {
                return Err(anyhow!("The home path for Java was set to an invalid directory: {} \
                Please set the location of the JAVA_HOME environment variable to match the location \
                of the Java installation.", java_path.display()));
            } else {
                info!(
                    "using java distribution [{}] from JAVA_HOME",
                    java_path.display()
                );

                java = java_path.to_str().unwrap().to_string();
            }
        } else {
            warn!(
                "Missing JAVA_HOME environment variable, checking if 'java' exists on machine..."
            );

            match which("java") {
                Ok(path) => {
                    info!("found java installation in [{}]", path.display());
                    java = path.to_str().unwrap().to_string();
                }

                Err(e) => return Err(anyhow!("The JAVA_HOME environment variable was not set and no 'java' command can be \
                found in the current PATH. Please set the location of the JAVA_HOME environment variable to match the location \
                of the Java installation. [original: {}]", e)),
            }
        }

        Ok(java)
    }
}
