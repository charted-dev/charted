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

pub mod charted;
pub mod registry;
pub mod repository;

use ::charted::server::version::APIVersion;
use eyre::Context as _;
use hcl::{
    edit::{
        expr::{Expression, TraversalOperator},
        visit::{visit_body, Visit},
        Ident,
    },
    Value,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};
use url::Url;

/// Represents the HCL configuration file (`.charted.hcl`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuration for the `charted {}` block, which configures
    /// the Helm plugin itself.
    #[serde(default)]
    pub charted: charted::Config,

    /// List of registries that are available for repositories.
    #[serde(
        default = "__default_registries",
        rename = "registry",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub registries: BTreeMap<String, registry::Config>,

    /// List of repositories available.
    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        rename = "repository",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub repositories: BTreeMap<String, repository::Config>,
}

impl Config {
    /// Loads the [`Config`] struct by first visiting all traversals to collect the `registry.*` keys,
    /// so we can load up the evaluation environment and then load it via eval expressions.
    pub fn load<P: AsRef<Path>>(path: P) -> eyre::Result<Config> {
        let path = path.as_ref();
        trace!(path = %path.display(), "loading .charted.hcl file in");

        let contents = fs::read_to_string(path).context(format!("was unable to read file {}", path.display()))?;

        // first, get a list of the referenced registries, so we can enforce that they exist
        #[allow(unused_assignments)]
        let mut registries = vec![];
        {
            let contents = contents.clone();
            let body = contents
                .parse::<hcl::edit::structure::Body>()
                .context("invalid hcl manifest")?;

            let mut visitor = RegistryTraversalVisitor::default();
            visit_body(&mut visitor, &body);

            registries = visitor.0;
        }

        let mut ctx = hcl::eval::Context::new();
        ctx.declare_var(
            "cwd",
            Value::String(path.parent().unwrap_or(path).display().to_string()),
        );

        let mut reg = hcl::Map::<String, Value>::new();
        for registry in registries.iter() {
            reg.insert(registry.clone(), Value::String(registry.clone()));
        }

        ctx.declare_var(
            "registry",
            Value::Object(
                registries
                    .iter()

                    // Since we don't know the URI without evaluating, we will just keep `registry.<name>` ~> `<name>` and
                    // the Helm plugin will determine what registry it belongs to.
                    .fold(hcl::Map::<String, Value>::new(), |mut map, registry| {
                        map.insert(registry.clone(), Value::String(registry.clone()));
                        map
                    }),
            ),
        );

        hcl::eval::from_str(&contents, &ctx).context("unable to evaluate HCL file")
    }
}

fn __default_registries() -> BTreeMap<String, registry::Config> {
    let mut registries = BTreeMap::new();
    registries.insert(
        String::from("default"),
        registry::Config {
            version: APIVersion::V1,
            url: Url::parse("https://charts.noelware.org/api").unwrap(),
        },
    );

    registries
}

#[derive(Default)]
struct RegistryTraversalVisitor(Vec<String>);
impl Visit for RegistryTraversalVisitor {
    fn visit_traversal(&mut self, node: &hcl::edit::expr::Traversal) {
        match &node.expr {
            Expression::Variable(var) if var.value() == &Ident::new("registry") => {
                for node in node.operators.iter() {
                    if let TraversalOperator::GetAttr(ref attr) = *node.value() {
                        let attr = attr.clone();
                        self.0.push(attr.as_str().to_string());
                    }
                }
            }

            _ => {} // skip
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hcl::edit::structure::Body;

    #[test]
    fn test_traversal() {
        let body = r#"body = 1
awau = registry.default
heccccc = true
"#;

        let body = body.parse::<Body>().unwrap();
        let mut visitor = RegistryTraversalVisitor::default();
        visitor.visit_body(&body);

        assert_eq!(visitor.0, vec![String::from("default")]);
    }

    #[test]
    fn validate_testcases_files() {
        for i in 1..4 {
            eprintln!("./testcases/{i}.charted.hcl :: run");
            Config::load(format!("./testcases/{i}.charted.hcl")).unwrap();
            eprintln!("./testcases/{i}.charted.hcl :: ok");
        }
    }
}
