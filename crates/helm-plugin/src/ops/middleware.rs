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

use charted_core::BoxedFuture;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::Next;
use std::time::Instant;
use tracing::{info, Instrument};

pub fn logging<'a>(
    req: Request,
    extensions: &'a mut Extensions,
    next: Next<'a>,
) -> BoxedFuture<'a, reqwest_middleware::Result<Response>> {
    let future = async move {
        info!("-> {} {}", req.method(), req.url());

        let start = Instant::now();
        let res = next.run(req, extensions).await?;
        info!(
            duration = %charted_core::serde::Duration::from(start.elapsed()),
            "<- {}",
            res.status()
        );

        Ok(res)
    };

    Box::pin(future.instrument(tracing::info_span!("charted.helm.http.request")))
}
