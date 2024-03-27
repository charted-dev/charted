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

#[cfg(not(windows))]
#[path = "./cases/get_tarballs.rs"]
mod get_tarballs;

#[cfg(not(windows))]
#[path = "./cases/sort_versions.rs"]
mod sort_versions;

#[path = "./cases/upload.rs"]
mod upload;

#[macro_export]
macro_rules! fixture {
    ($path:literal) => {
        ::std::path::PathBuf::from(::std::env!("CARGO_MANIFEST_DIR"))
            .join("tests/__fixtures__")
            .join($path)
    };
}

#[macro_export]
macro_rules! testcase {
    ($name:ident: |$storage:ident| $code:block) => {
        #[tokio::test]
        async fn $name() {
            let tempdir = ::tempfile::TempDir::new().unwrap();
            let path = tempdir.into_path();
            let storage = ::noelware_remi::StorageService::Filesystem(::remi_fs::StorageService::with_config(
                remi_fs::Config::new(&path),
            ));

            {
                let $storage = storage.clone();
                $code
            }

            // clean up the storage service so we don't dangle the `path` from being destroyed since it
            // is a reference to the tempdir
            ::std::mem::drop(storage);
            ::std::fs::remove_dir_all(path).expect("tempdir to be removed by now");
        }
    };
}
