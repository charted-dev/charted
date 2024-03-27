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

use crate::{fixture, testcase};
use charted_helm_charts::HelmCharts;
use remi::{StorageService as _, UploadRequest};
use std::fs;

testcase!(
    get_tarball: |storage| {
        for version in ["0.1.0-beta", "0.2.1", "1.0.0-beta.1", "2023.3.24", "1.0.0+d1cebae"] {
            let fixture = fixture!("hello-world.tgz");
            let contents = fs::read(&fixture).unwrap();

            storage
                .upload(
                    format!("./repositories/1/2/tarballs/{version}.tgz"),
                    UploadRequest::default()
                        .with_content_type(Some("application/tar+gzip"))
                        .with_data(contents),
                )
                .await
                .unwrap();
        }

        let charts = HelmCharts::new(storage);

        // these cases should work
        let _ = charts.get_tarball(1, 2, "latest", false).await.unwrap().unwrap();
        let _ = charts.get_tarball(1, 2, "0.1.0-beta", true).await.unwrap().unwrap();
        let _ = charts.get_tarball(1, 2, "0.1.0-beta", false).await.unwrap_err();
    }
);
