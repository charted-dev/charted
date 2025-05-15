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

use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

/// The ordering of the data. Either ascending or descending.
#[derive(Debug, Clone, Copy, Default, Deserialize, ToSchema)]
pub enum Ordering {
    #[serde(rename = "ASC")]
    #[default]
    Ascending,

    #[serde(rename = "DESC")]
    Descending,
}

impl Ordering {
    pub fn into_sea_orm(self) -> sea_orm::Order {
        match self {
            Ordering::Ascending => sea_orm::Order::Asc,
            Ordering::Descending => sea_orm::Order::Desc,
        }
    }
}

/// A pagination request.
#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct PaginationRequest {
    /// Amount of entries per page.
    #[serde(default = "__per_page")]
    #[schema(minimum = 10, maximum = 100)]
    pub per_page: usize,

    /// Ordering of the data.
    #[serde(default)]
    pub order_by: Ordering,

    /// The page to go towards, default is `1`.
    #[serde(default = "__default_page")]
    pub page: usize,
}

#[inline(always)]
const fn __per_page() -> usize {
    10
}

#[inline(always)]
const fn __default_page() -> usize {
    1
}
