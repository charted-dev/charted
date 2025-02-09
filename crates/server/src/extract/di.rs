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

use axum::extract::{FromRequest, Request};
use charted_core::api;
use std::any::{type_name, Any};

#[derive(Debug, derive_more::Display)]
#[display("api server is not properly set-up")]
struct ServerNotProperlySetup;
impl std::error::Error for ServerNotProperlySetup {}

#[derive(Debug, derive_more::Display)]
#[display("unknown dependency of type ({}) was not found", type_name)]
struct UnknownDependency {
    type_name: &'static str,
}

impl std::error::Error for UnknownDependency {}

/// `Dep` is an Axum extractor that implements [`FromRequest`] to get
/// a dependency from a [`di::Container`][charted_core::di::Container].
#[derive(Debug, Clone)]
pub struct Dep<T: Any>(pub T);
impl<T: Clone + Any, S: Send + Sync> FromRequest<S> for Dep<T> {
    type Rejection = api::Response;

    async fn from_request(_: Request, _: &S) -> Result<Self, Self::Rejection> {
        let Some(container) = &charted_core::CONTAINER.get() else {
            return Err(api::system_failure(ServerNotProperlySetup));
        };

        match container.get::<T>() {
            Ok(value) => Ok(Dep(value.to_owned())),
            Err(charted_core::di::Error::FailedCast) | Err(charted_core::di::Error::ObjectUnavaliable) => {
                Err(api::system_failure(UnknownDependency {
                    type_name: type_name::<T>(),
                }))
            }

            _ => unreachable!(),
        }
    }
}
