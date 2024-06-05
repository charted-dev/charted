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

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::FromRequest,
    http::{header, HeaderMap, Request},
    RequestExt,
};
use charted_core::multipart::*;
use std::ops::{Deref, DerefMut};

/// Explicit wrapper type for [`multer::Multipart`] that is also an Axum extractor.
pub struct Multipart(multer::Multipart<'static>);

impl Multipart {
    pub fn into_inner(self) -> multer::Multipart<'static> {
        self.0
    }
}

impl Deref for Multipart {
    type Target = multer::Multipart<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Multipart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<S> FromRequest<S> for Multipart
where
    S: Send + Sync,
{
    type Rejection = MultipartRejection;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let boundary = boundary(req.headers())?;
        let stream = req.with_limited_body().into_body();

        Ok(Self(multer::Multipart::new(stream.into_data_stream(), boundary)))
    }
}

fn boundary(headers: &HeaderMap) -> Result<String, MultipartRejection> {
    let Some(val) = headers.get(header::CONTENT_TYPE) else {
        return Err(multer::Error::NoBoundary.into());
    };

    let Ok(val) = val.to_str() else {
        return Err(MultipartRejection::InvalidBoundary);
    };

    multer::parse_boundary(val).map_err(From::from)
}
