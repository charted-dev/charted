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

use charted_core::pagination::OrderBy;

/// A object representing how the pagination query should do.
pub struct Request {
    /// Sets how many elements can be in per page.
    pub per_page: usize,

    /// Sets how entities are ordered.
    pub order_by: OrderBy,

    /// Cursor ID to passthrough when flipping through pages.
    pub cursor: Option<u64>,

    /// ID of the owner to filter through. This is only used in `repositories`
    /// and `organizations` pagination.
    pub owner_id: Option<u64>,

    /// whether if private repositories and organizations can be sent through
    pub allow_private: bool,
}
