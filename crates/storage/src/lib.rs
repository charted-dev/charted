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

// expose `StorageService` so we don't have to keep declaring `remi_core`, expose `Bytes`
// so we don't have to declare more transitive dependencies to `bytes`.
pub use bytes::Bytes;
pub use remi_core::{Blob, DirectoryBlob, FileBlob, ListBlobsRequest, StorageService, UploadRequest};
pub use remi_fs::{ContentTypeResolver, DefaultContentTypeResolver};

use charted_config::StorageConfig;
use remi_fs::FilesystemStorageService;
use remi_s3::S3StorageService;

type Result<T> = std::result::Result<T, std::io::Error>;

// TODO: move to Box<dyn remi_core::StorageService> once it is object-safe.
#[derive(Debug, Clone)]
pub enum MultiStorageService {
    Filesystem(FilesystemStorageService),
    S3(Box<S3StorageService>),
}

impl From<StorageConfig> for MultiStorageService {
    fn from(config: StorageConfig) -> MultiStorageService {
        match config {
            StorageConfig::Filesystem(fs) => MultiStorageService::Filesystem(FilesystemStorageService::with_config(fs)),
            StorageConfig::S3(s3) => MultiStorageService::S3(Box::new(S3StorageService::new(s3))),
        }
    }
}

macro_rules! gen_methods {
    (
        $(fn $name:ident($($arg_name:ident: $ty:ty),*)$(-> $return_:ty)?;)*
    ) => {
        $(
            fn $name<'life0, 'async_trait>(&'life0 self,$($arg_name: $ty,)*)$(-> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = $return_> + Send + 'async_trait>>)?
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    let __code$(: $return_)? = {
                        match self {
                            MultiStorageService::Filesystem(fs) => fs.$name($($arg_name,)*).await,
                            MultiStorageService::S3(s3) => s3.$name($($arg_name,)*).await,
                        }
                    };

                    __code
                })
            }
        )*
    };
}

impl StorageService for MultiStorageService {
    fn name(self) -> &'static str {
        "charted:remi"
    }

    gen_methods! {
        fn init() -> Result<()>;
        fn open(path: impl AsRef<::std::path::Path> + Send + 'async_trait) -> Result<Option<::bytes::Bytes>>;
        fn blob(path: impl AsRef<::std::path::Path> + Send + 'async_trait) -> Result<Option<remi_core::Blob>>;
        fn blobs(path: Option<impl AsRef<::std::path::Path> + Send + 'async_trait>, options: Option<remi_core::ListBlobsRequest>) -> Result<Vec<remi_core::Blob>>;
        fn delete(path: impl AsRef<::std::path::Path> + Send + 'async_trait) -> Result<()>;
        fn exists(path: impl AsRef<::std::path::Path> + Send + 'async_trait) -> Result<bool>;
        fn upload(path: impl AsRef<::std::path::Path> + Send + 'async_trait, options: remi_core::UploadRequest) -> Result<()>;
    }
}
