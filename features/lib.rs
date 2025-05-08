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

use azalia::rust::AsArcAny;
use charted_core::BoxedFuture;
use sea_orm_migration::{MigrationTrait, MigratorTrait};
use serde::Serialize;
use utoipa::{ToSchema, openapi::OpenApi};

#[derive(Debug, Clone, Copy)]
struct NoMigratorAvaliable;
impl MigratorTrait for NoMigratorAvaliable {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}

/// Metadata about this feature.
///
/// This is used for the `/features/[name]` feature to determine the metadata
/// of a feature regardless if it's enabled or not. The `/features` endpoint will
/// return if this feature is enabled or not alongside its metadata as: `[enabled,
/// metadata]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema, Serialize)]
pub struct Metadata {
    /// Name of the feature.
    pub name: &'static str,

    /// The configuration key that this feature is configured in.
    pub config_key: &'static str,

    /// Description about this feature.
    pub description: &'static str,

    /// Authors that created this feature.
    pub authors: &'static [&'static str],

    /// When did this feature appear in?
    pub since: &'static str,

    /// If the feature is deprecated, this is the notice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<Deprecation>,
}

/// Deprecation of this feature and why it was deprecated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema, Serialize)]
pub struct Deprecation {
    /// What version since this feature is deprecated.
    pub since: &'static str,

    /// What version when this feature will be no longer avaliable.
    pub removed_in: &'static str,

    /// Optional message about this deprecation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<&'static str>,
}

/// A server feature. Read more in the [documentation].
///
/// [documentation]: https://charts.noelware.org/docs/server/latest/features
pub trait Feature: AsArcAny + Send + Sync {
    /// Metadata about this feature.
    fn metadata(&self) -> Metadata;

    /// Allows to do initialization of this feature before the API server starts.
    fn init<'feat, 'cx: 'feat>(&self) -> BoxedFuture<'feat, ()> {
        Box::pin(async {})
    }

    /// If this feature extends the API server's routes, then this is where
    /// the OpenAPI document must be avaliable.
    ///
    /// The `doc` parameter will be the initial document that was generated
    /// beforehand.
    fn extend_openapi(&self, doc: &mut OpenApi) {
        let _ = doc;
    }

    /// If this feature implements new REST routes, then this is the method
    /// to implement when extending the server.
    fn router(&self) -> Option<(&'static str, ())> {
        None
    }

    /// Returns a [`MigratorTrait`] of all the migrations that are avaliable
    /// for this server feature, if it extends the database.
    fn migrator(&self) -> Option<impl MigratorTrait>
    where
        Self: Sized,
    {
        None::<NoMigratorAvaliable>
    }
}

azalia::impl_dyn_any!(Feature);
