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

use serde_json::{json, Value};
use std::collections::BTreeMap;
use tracing::field::Visit;

pub struct JsonVisitor<'a>(pub &'a mut BTreeMap<String, Value>);

macro_rules! impl_visitor_instructions {
    ($($name:ident => $ty:ty),*) => {
        $(
            fn $name(&mut self, field: &::tracing::field::Field, value: $ty) {
                self.0.insert(field.name().to_string(), ::serde_json::json!(value));
            }
        )*
    };
}

impl<'a> Visit for JsonVisitor<'a> {
    impl_visitor_instructions! {
        record_f64 => f64,
        record_i64 => i64,
        record_u64 => u64,
        record_i128 => i128,
        record_bool => bool,
        record_str => &str
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(field.name().to_string(), json!(format!("{:?}", value)));
    }

    fn record_error(&mut self, field: &tracing::field::Field, value: &(dyn std::error::Error + 'static)) {
        self.0.insert(field.name().to_string(), json!(value.to_string()));
    }
}
