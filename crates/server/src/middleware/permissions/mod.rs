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

#![allow(unused)]

use crate::ServerContext;
use axum::{
    body::Body,
    http::{Request, Response},
};
use charted_core::bitflags::{MemberPermission, MemberPermissions};
use charted_types::{Organization, Repository, User};
use tracing::instrument;

/// Which database table we should use?
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Table {
    /// `repository_members`
    Repository,

    /// `organization_members`
    Organization,
}

/// Middleware to check a repository or organization member's permission.
#[derive(Clone)]
pub struct Middleware {
    require_permissions: MemberPermissions,
    table: Table,
}

impl Middleware {
    pub fn new(table: Table) -> Middleware {
        Middleware {
            require_permissions: MemberPermissions::new(0),
            table,
        }
    }

    pub fn permissions<I: IntoIterator<Item = MemberPermission>>(self, permissions: I) -> Self {
        let mut bitfield = self.require_permissions;
        bitfield.add(permissions);

        Self {
            require_permissions: bitfield,
            ..self
        }
    }

    #[instrument(
        name = "charted.server.permissionCheck.organization",
        skip_all,
        fields(%user.id, %user.username, %organization.name, %organization.id)
    )]
    async fn perform_organization_member_permission_check(
        self,
        req: Request<Body>,
        ctx: &ServerContext,
        organization: Organization,
        user: User,
    ) -> Result<Request<Body>, Response<Body>> {
        todo!()
    }

    #[instrument(
        name = "charted.server.permissionCheck.repository",
        skip_all,
        fields(%user.id, %user.username, %repository.name, %repository.id)
    )]
    async fn perform_repository_member_permission_check(
        self,
        req: Request<Body>,
        ctx: &ServerContext,
        repository: Repository,
        user: User,
    ) -> Result<Request<Body>, Response<Body>> {
        todo!()
    }
}
