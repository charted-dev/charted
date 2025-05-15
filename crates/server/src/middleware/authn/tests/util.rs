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

use crate::{
    Env,
    middleware::authn::{Factory, Options},
};
use axum::{
    Router,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing,
};
use charted_config::{
    Config, database, metrics,
    sessions::{self, Backend},
};
use sentry::protocol::Url;

async fn echo(req: axum::extract::Request) -> impl IntoResponse {
    (StatusCode::OK, Response::new(req.into_body()))
}

// so that sessions are "consistent" enough between tests
const JWT_SECRET_KEY: &str =
    "ahashthatshouldbeavalidhashfromopensslbutidontwanttodothatandnooneshouldusethisvaluetobeginwithuwu";

pub async fn create_environment(ov: impl FnOnce(&mut Env)) -> Env {
    let mut env = Env::new(Config {
        jwt_secret_key: JWT_SECRET_KEY.to_owned(),
        registrations: true,
        single_user: false,
        single_org: false,
        sentry_dsn: None,
        base_url: Some(Url::parse("http://localhost:3651").unwrap()),
        logging: Default::default(),
        storage: Default::default(),
        tracing: None,
        metrics: metrics::Config::Disabled,
        server: Default::default(),

        sessions: sessions::Config {
            enable_basic_auth: false,
            backend: Backend::Static(azalia::btreemap! {
                // echo "noeliscutieuwu" | cargo cli admin authz hash-password --stdin
                "noel" => "$argon2id$v=19$m=19456,t=2,p=1$gIcVA4mVHgr8ZWkmDrtJlw$sb5ypFAvphFCGrJXy9fRI1Gb/2vGIH1FTzDax458+xY"
            })
        },

        database: database::Config::SQLite(database::sqlite::Config {
            common: Default::default(),
            path: String::from(":memory:").into(),
        }),
    }).await.expect("failed to create server environment");

    ov(&mut env);
    env
}

pub async fn create_router(options: Options, ov: impl FnOnce(&mut Env)) -> Router {
    let env = create_environment(ov).await;

    Router::new()
        .route("/echo", routing::post(echo).layer(env.authn(options)))
        .with_state(env)
}

macro_rules! consume_body {
    ($body:ident as $T:ty) => {{
        const fn __assert_deserialize<D: ::serde::de::DeserializeOwned>() {}
        __assert_deserialize::<$T>();

        let bytes = ::axum::body::to_bytes($body, usize::MAX).await.unwrap();
        ::serde_json::from_slice::<$T>(&bytes).unwrap()
    }};
}

pub(in crate::middleware::authn::tests) use consume_body;

macro_rules! testcase {
    (
        [options($options:expr)]
        $(#[$meta:meta])*
        $name:ident($service:ident) $code:block;
    ) => {
        #[::tokio::test]
        $(#[$meta])*
        async fn $name() {
            use ::tower::ServiceExt;
            use ::tracing_subscriber::prelude::*;

            // setup tracing logs here so that we can
            // get logs from integration tests
            let _log_guard = ::tracing_subscriber::registry()
                .with(::azalia::log::WriteLayer::new_with(
                    ::std::io::stderr(),
                    ::azalia::log::writers::default::Writer::default()
                        .with_thread_name(false)
                ))
                .with(::tracing_subscriber::EnvFilter::from_env("INTEGTEST_LOG"))
                .set_default();

            let mut router = $crate::middleware::authn::tests::util::create_router(
                $options,
                |_| {}
            ).await;

            let mut __service = router.as_service::<::axum::body::Body>();
            let $service = __service.ready().await.unwrap();

            let __ret = $code;
            __ret
        }
    };

    (
        [env_override(|$env:ident| $envret:block)]
        $(#[$meta:meta])*
        $name:ident($service:ident) $code:block;
    ) => {
        #[::tokio::test]
        $(#[$meta])*
        async fn $name() {
            use ::tower::ServiceExt;
            use ::tracing_subscriber::prelude::*;

            // setup tracing logs here so that we can
            // get logs from integration tests
            let _log_guard = ::tracing_subscriber::registry()
                .with(::azalia::log::WriteLayer::new_with(
                    ::std::io::stderr(),
                    ::azalia::log::writers::default::Writer::default()
                        .with_thread_name(false)
                ))
                .with(::tracing_subscriber::EnvFilter::from_env("INTEGTEST_LOG"))
                .set_default();

            let mut router = $crate::middleware::authn::tests::util::create_router(
                ::core::default::Default::default(),
                |$env| $envret
            ).await;

            let mut __service = router.as_service::<::axum::body::Body>();
            let $service = __service.ready().await.unwrap();

            let __ret = $code;
            __ret
        }
    };

    ($(#[$meta:meta])* $name:ident($service:ident) $code:block;) => {
        #[::tokio::test]
        $(#[$meta])*
        async fn $name() {
            use ::tower::ServiceExt;
            use ::tracing_subscriber::prelude::*;

            // setup tracing logs here so that we can
            // get logs from integration tests
            let _log_guard = ::tracing_subscriber::registry()
                .with(::azalia::log::WriteLayer::new_with(
                    ::std::io::stderr(),
                    ::azalia::log::writers::default::Writer::default()
                        .with_thread_name(false)
                ))
                .with(::tracing_subscriber::EnvFilter::from_env("INTEGTEST_LOG"))
                .set_default();

            let mut router = $crate::middleware::authn::tests::util::create_router(
                ::core::default::Default::default(),
                |_| {}
            ).await;

            let mut __service = router.as_service::<::axum::body::Body>();
            let $service = __service.ready().await.unwrap();

            let __ret = $code;
            __ret
        }
    };
}

pub(in crate::middleware::authn::tests) use testcase;
