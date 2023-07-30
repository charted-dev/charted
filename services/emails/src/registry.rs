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

use eyre::{eyre, Result};
use mustache::Data;
use once_cell::sync::OnceCell;
use std::{
    fs::create_dir,
    io::ErrorKind,
    path::{Path, PathBuf},
    time::SystemTime,
};
use tokio::{fs::File, io::AsyncReadExt};

static INIT: OnceCell<()> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct TemplateRegistry(PathBuf);

impl TemplateRegistry {
    pub fn new<P: AsRef<Path>>(path: P) -> TemplateRegistry {
        TemplateRegistry(path.as_ref().to_path_buf())
    }

    pub fn init(&self) -> Result<()> {
        if INIT.get().is_some() {
            warn!("template registry was already initialized");
            return Ok(());
        }

        info!("initializing template registry in dir {}", self.0.display());
        if !self.0.exists() {
            warn!("...directory doesn't exist, now creating!");
            create_dir(self.0.clone())?;
        }

        INIT.set(()).unwrap();
        Ok(())
    }

    pub async fn find<P: AsRef<Path>>(&self, path: P) -> Result<bool> {
        match File::open(self.0.join(path.as_ref())).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(eyre!("unable to find file {}: {e}", path.as_ref().display())),
        }
    }

    pub async fn render<P: AsRef<Path>, D: Into<Data>>(&self, path: P, context: D) -> Result<String> {
        let template = self.0.join(path.as_ref());
        if !template.exists() {
            return Err(eyre!("template '{}' was not found", template.display()));
        }

        let start = SystemTime::now();
        debug!("rendering template {}", template.display());

        let mut file = File::open(template.clone()).await?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;

        let rendered = mustache::compile_str(buf.as_str())?;
        let compiled = rendered.render_data_to_string(&context.into())?;
        let elapsed = start.elapsed().unwrap();

        info!("took {elapsed:?} to render template {}", template.display());

        Ok(compiled)
    }
}
