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

use eyre::Result;
use serde_starlark::Assignment;
use starlark::{
    environment::{Globals, Module},
    eval::Evaluator,
    syntax::{AstModule, Dialect},
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

/// Updates the `build/manifests.bzl` file in the root directory of where
/// the charted-server project is located in.
pub fn bazel_cargo_manifest_updater(manifest: String) -> Result<()> {
    let manifests = PathBuf::from("./build/manifests.bzl");
    let manifests = manifests.canonicalize()?;

    debug!(
        location = tracing::field::display(manifests.display()),
        "located `build/manifests.bzl` file, creating Starlark intepreter"
    );

    let contents = fs::read_to_string(manifests.clone())?;
    let ast =
        AstModule::parse("build/manifests.bzl", contents, &Dialect::Standard).map_err(|e| eyre::eyre!(Box::new(e)))?;

    let globals = Globals::standard();
    let module = Module::new();
    debug!(
        location = tracing::field::display(manifests.display()),
        "evaluating bazel file in given"
    );

    {
        let mut eval = Evaluator::new(&module);
        eval.eval_module(ast, &globals).map_err(|e| eyre::eyre!(Box::new(e)))?;
    }

    let manifests_repr = module.get("CARGO_MANIFESTS").expect("`CARGO_MANIFESTS` is missing?!");
    let value = manifests_repr.to_json_value().unwrap();
    let mut array_str = value
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap())
        .collect::<Vec<_>>();

    array_str.push(manifest.as_str());
    array_str.sort();

    let assignment = Assignment::new("CARGO_MANIFESTS", array_str);
    let starlark_repr = serde_starlark::to_string(&assignment).unwrap();
    let mut file = OpenOptions::new()
        .append(false) // we want to overwrite it
        .write(true)
        .read(true)
        .create(false)
        .open(manifests)?;

    write!(
        file,
        r#"# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# buildifier: disable=module-docstring
{starlark_repr}
"#
    )?;

    Ok(())
}
