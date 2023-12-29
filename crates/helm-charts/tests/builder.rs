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

// disable clippy on Windows as it is all dead code for now
#![cfg_attr(windows, allow(clippy::all))]

use charted_config::var;
use charted_helm_charts::{HelmCharts, UploadReleaseTarball};
use charted_storage::MultiStorageService;
use remi_core::StorageService;
use remi_fs::FilesystemStorageService;
use std::{env::current_dir, path::PathBuf};
use tempfile::TempDir;
use tokio::{fs::create_dir_all, test};

fn tempdir() -> Result<TempDir, std::io::Error> {
    let tmpdir = var!("RUNNER_TEMP", to: PathBuf, is_optional: true);
    match tmpdir {
        Some(path) => TempDir::new_in(path),
        None => TempDir::new(),
    }
}

#[cfg(not(windows))] // test fails due to normalization of paths; fix pls @auguwu :)
#[test]
async fn test_upload() {
    let tmpdir = tempdir().unwrap();
    eprintln!("[TEST] >> tempdir: {}", tmpdir.path().display());

    let service = FilesystemStorageService::new(&tmpdir);
    service.init().await.unwrap();
    if !service.exists("./repositories/1/2").await.unwrap() {
        create_dir_all(service.normalize("./repositories/1/2").unwrap().unwrap())
            .await
            .unwrap();
    }

    let charts = HelmCharts::new(MultiStorageService::Filesystem(service));
    charts.init().await.unwrap();

    let fixtures = match cfg!(bazel) {
        true => {
            let dir = current_dir().unwrap();
            [
                dir.join("tests/__fixtures__/elasticsearch-8.5.1.tgz"),
                dir.join("tests/__fixtures__/hello-world-0.1.0.tgz"),
                dir.join("tests/__fixtures__/zookeeper-9.2.1.tgz"),
            ]
        }
        false => [
            PathBuf::from("./tests/__fixtures__/elasticsearch-8.5.1.tgz"),
            PathBuf::from("./tests/__fixtures__/hello-world-0.1.0.tgz"),
            PathBuf::from("./tests/__fixtures__/zookeeper-9.2.1.tgz"),
        ],
    };

    for fixture in fixtures.iter() {
        dbg!(fixture);
        let canon = fixture.canonicalize().unwrap();
        eprintln!("[TEST] >> FIXTURE: {}", canon.display());

        let _upload = UploadReleaseTarball::default()
            .with_version("0.1.0-beta")
            .with_owner(1)
            .with_repo(2);
    }

    // ensure that the tempdir was closed before all tests have passed.
    tmpdir.close().unwrap();
}
