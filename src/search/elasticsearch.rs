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

#![allow(unused)] // this module isn't finished yet

use super::JoinedBackend;
use crate::common::models::entities::{Organization, Repository, User};
use elasticsearch::Elasticsearch;

/// Represents a Elasticsearch client for the supported backends.
#[derive(Debug, Clone)]
pub struct Client(Elasticsearch);

impl Client {
    /// Creates a new [`Client`], which can be used to initialize and create a new
    /// [`JoinedBackend`].
    pub fn new() -> eyre::Result<Client> {
        todo!()
    }

    /// Initializes this [`Client`] by creating the necessary indexes if it can.
    pub async fn init(&self) -> eyre::Result<()> {
        todo!()
    }

    /// Creates a new [`JoinedBackend`] for this client.
    pub fn joined(self) -> JoinedBackend {
        JoinedBackend {
            organizations: Box::new(OrganizationElasticsearchBackend(self.0.clone())),
            repositories: Box::new(RepositoryElasticsearchBackend(self.0.clone())),
            users: Box::new(UserElasticsearchBackend(self.0)),
        }
    }
}

macro_rules! impl_backend {
    ($($ty:ty),*) => {
        $(
            paste::paste! {
                #[doc = concat!(" Represents the backend for ", stringify!($ty))]
                #[derive(Debug, Clone)]
                pub struct [<$ty ElasticsearchBackend>](::elasticsearch::Elasticsearch);

                #[async_trait]
                impl $crate::search::Backend<$ty> for [<$ty ElasticsearchBackend>] {
                    async fn search(&self, query: &str, obj: $ty) -> eyre::Result<()> {
                        todo!()
                    }

                    async fn index(&self, obj: $ty) -> eyre::Result<()> {
                        todo!()
                    }

                    async fn bulk_index(&self, obj: Vec<$ty>) -> eyre::Result<()> {
                        todo!()
                    }

                    async fn delete(&self, obj: $ty) -> eyre::Result<()> {
                        todo!()
                    }
                }
            }
        )*
    };
}

impl_backend!(User, Organization, Repository);
