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

use crate::{tests::fixture, testutil};
use charted_config::storage::Config;
use charted_datastore::{
    DataStore,
    remi::{StorageService, UploadRequest},
};
use charted_types::Ulid;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn filesystem() {
    let _log_guard = testutil::setup_tracing();
    let tmpdir = TempDir::new().unwrap();

    // Since we need to get the path that it is located, we need to disable
    // cleanup. We can do that ourselves though. :>
    let path = tmpdir.keep();
    let ds = DataStore::new(&Config::Filesystem(charted_datastore::fs::StorageConfig::new(
        path.clone(),
    )))
    .await
    .unwrap();

    do_test(ds).await;
    if let Err(e) = std::fs::remove_dir_all(&path) {
        eprintln!("warn: failed to delete all contents in '{}': {e}", path.display());
    }
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn s3() {
    use charted_datastore::s3::{self, aws};

    // Always succeed the test if Docker tests are disabled.
    if super::docker_tests_disabled() {
        return;
    }

    let _log_guard = testutil::setup_tracing();
    let minio = testutil::minio().await.unwrap();
    let port = minio.get_host_port_ipv4(9000).await.unwrap();

    let config = s3::StorageConfig {
        enforce_path_access_style: true,
        default_bucket_acl: Some(aws::s3::types::BucketCannedAcl::Private),
        default_object_acl: Some(aws::s3::types::ObjectCannedAcl::BucketOwnerFullControl),
        secret_access_key: "somedummysecretkey".into(),
        access_key_id: "somedummyaccesskey".into(),
        endpoint: Some(format!("http://localhost:{port}")),
        region: Some(aws::config::Region::from_static("us-east-1")),
        bucket: "test-bucket".into(),

        ..Default::default()
    };

    let ds = DataStore::new(&Config::S3(config)).await.unwrap();
    do_test(ds).await
}

#[cfg(target_os = "linux")]
#[tokio::test]
async fn azure() {
    use charted_datastore::azure;

    // Always succeed the test if Docker tests are disabled.
    if super::docker_tests_disabled() {
        return;
    }

    let _log_guard = testutil::setup_tracing();
    let azurite = testutil::azurite().await.unwrap();
    let port = azurite.get_host_port_ipv4(10000).await.unwrap();

    let config = azure::StorageConfig {
        container: "test-container".into(),
        location: azure::CloudLocation::Emulator {
            address: "0.0.0.0".into(),
            port,
        },

        credentials: azure::Credential::AccessKey {
            account: String::from("devstoreaccount1"),
            access_key: String::from(
                "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==",
            ),
        },
    };

    let ds = DataStore::new(&Config::Azure(config)).await.unwrap();
    do_test(ds).await
}

async fn do_test(ds: DataStore) {
    let fixture = fixture!("tarballs/youtrack.tgz");
    let contents = fs::read(&fixture).await.unwrap();

    let owner = Ulid::new("01J5SG1FXT019M8Q2TB84QVV8V").unwrap();
    let repo = Ulid::new("01J5SG1JAEG4RJCGYC5KJ6QYS2").unwrap();
    let ns = crate::OwnerRepoNamespace::new(&ds, owner, repo);

    for version in ["0.1.0-beta", "0.2.1", "1.0.0-beta.1", "2024.3.24", "1.0.0+d1cebae"] {
        let request = UploadRequest::default()
            .with_content_type(Some("application/tar+gzip"))
            .with_data(contents.clone());

        ns.upload(format!("tarballs/{version}.tgz"), request).await.unwrap();

        // ensure that it exists
        assert!(ns.exists(format!("tarballs/{version}.tgz")).await.unwrap());
    }

    // shows non-prereleases
    let versions = ns.sort_versions(false).await.unwrap();
    assert_eq!(versions, &[
        semver::Version::parse("2024.3.24").unwrap().into(),
        semver::Version::parse("1.0.0+d1cebae").unwrap().into(),
        semver::Version::parse("0.2.1").unwrap().into(),
    ]);

    // shows all pre-releases
    let versions = ns.sort_versions(true).await.unwrap();
    assert_eq!(versions, &[
        semver::Version::parse("2024.3.24").unwrap().into(),
        semver::Version::parse("1.0.0+d1cebae").unwrap().into(),
        semver::Version::parse("1.0.0-beta.1").unwrap().into(),
        semver::Version::parse("0.2.1").unwrap().into(),
        semver::Version::parse("0.1.0-beta").unwrap().into(),
    ]);
}
