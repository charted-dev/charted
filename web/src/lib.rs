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

#[cfg_attr(webui, derive(rust_embed::RustEmbed))]
#[cfg_attr(webui, folder = "dist")]
pub struct WebUI;

impl WebUI {
    /// Whether if the web UI should be distributed or not with this
    /// crate.
    ///
    /// This is enabled via the `--cfg=webui=true` rustc flag, which
    /// requires the Vite backend to be built (with `bazel build //web:build`) to
    /// be successfully compiled.
    pub fn enabled() -> bool {
        cfg!(webui)
    }
}
