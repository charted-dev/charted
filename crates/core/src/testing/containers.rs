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

//! The `testing::containers` module contains definitions for running software via
//! Docker for integration testing.
//!
//! * `elasticsearch` is used in `crates/search/elasticsearch` to test Elasticsearch
//!   usage as Noelware uses Elasticsearch for full text search.
//! * `meilisearch` is used in `crates/search/meilisearch` to test Meilisearch
//!   integration as we will also maintain support for it.
//! * `postgresql` and `redis` are used in all the API server integration tests.

pub mod elasticsearch;
pub mod meilisearch;
pub mod postgresql;
pub mod redis;
