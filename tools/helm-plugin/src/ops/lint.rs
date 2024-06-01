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

//! The `charted_helm_plugin::ops::lint` module contains operations to lint and format
//! a Helm chart with `helm lint` and to lint the `.charted.hcl` configuration file
//! with the [`hclfmt`] CLI.
//!
//! [`hclfmt`]: https://github.com/hashicorp/hcl/blob/main/cmd/hclfmt/main.go

pub mod hclfmt;
pub mod helm;
