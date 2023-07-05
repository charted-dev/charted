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

use axum::Router;
use once_cell::sync::Lazy;
use tower::ServiceBuilder;

pub mod v1;

#[allow(non_upper_case_globals)]
static default_router: Lazy<Box<dyn Fn() -> Router + Send + Sync>> = Lazy::new(|| Box::new(v1::create_router));

macro_rules! create_router_internal {
    ($($version:ident: $cr:ident),*) => {
        fn create_router_internal() -> ::axum::Router {
            let mut router = ::axum::Router::new();
            $(
                router = router.clone().nest(concat!("/", stringify!($cr)), $crate::routing::$cr::create_router());
            )*

            router = router.clone().merge(default_router());
            router
        }
    };
}

create_router_internal!(V1: v1);

pub fn create_router() -> Router {
    let stack = ServiceBuilder::new()
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::new());

    create_router_internal().layer(stack).layer(axum::middleware::from_fn(crate::middleware::log))
}
