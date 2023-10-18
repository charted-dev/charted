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

use crate::{var, FromEnv};
use aws_sdk_s3::{
    config::Region,
    types::{BucketCannedAcl, ObjectCannedAcl},
};
use charted_common::TRUTHY_REGEX;
use remi_fs::FilesystemStorageConfig;
use remi_s3::S3StorageConfig;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(clippy::large_enum_variant)]
pub enum StorageConfig {
    Filesystem(FilesystemStorageConfig),
    S3(S3StorageConfig),
}

impl Default for StorageConfig {
    fn default() -> StorageConfig {
        let datadir = var!("CHARTED_STORAGE_FILESYSTEM_DIRECTORY", or_else: "./data".into());
        StorageConfig::Filesystem(FilesystemStorageConfig::new(datadir))
    }
}

impl FromEnv for StorageConfig {
    type Output = StorageConfig;

    fn from_env() -> StorageConfig {
        match std::env::var("CHARTED_STORAGE_SERVICE") {
            Err(_) => StorageConfig::default(),
            Ok(val) => match val.as_str() {
                // Default impl already has what we need it to do for the
                // fs service.
                "filesystem" | "fs" => StorageConfig::default(),
                "s3" => {
                    let enable_signer_v4_requests = var!("CHARTED_STORAGE_S3_ENABLE_SIGNER_V4_REQUESTS", {
                        or_else: false;
                        mapper: |p| TRUTHY_REGEX.is_match(p.as_str());
                    });

                    let enforce_path_access_style = var!("CHARTED_STORAGE_S3_ENFORCE_PATH_ACCESS_STYLE", {
                        or_else: false;
                        mapper: |p| TRUTHY_REGEX.is_match(p.as_str());
                    });

                    let default_object_acl = var!("CHARTED_STORAGE_S3_DEFAULT_OBJECT_ACL", {
                        or_else: ObjectCannedAcl::BucketOwnerFullControl;
                        mapper: |p| ObjectCannedAcl::from_str(p.as_str()).unwrap();
                    });

                    let default_bucket_acl = var!("CHARTED_STORAGE_S3_DEFAULT_BUCKET_ACL", {
                        or_else: BucketCannedAcl::Private;
                        mapper: |p| BucketCannedAcl::from_str(p.as_str()).unwrap();
                    });

                    let secret_access_key = var!("CHARTED_STORAGE_S3_SECRET_ACCESS_KEY", or_else_do: |_| panic!("Missing required environment variable: `CHARTED_STORAGE_S3_SECRET_ACCESS_KEY`."));
                    let access_key_id = var!("CHARTED_STORAGE_S3_ACCESS_KEY_ID", or_else_do: |_| panic!("Missing required environment variable: `CHARTED_STORAGE_S3_ACCESS_KEY_ID`."));
                    let region = var!("CHARTED_STORAGE_S3_REGION", {
                        or_else: Region::new(Cow::Owned("us-east-1".to_owned()));
                        mapper: |val| Region::new(Cow::Owned(val));
                    });

                    let bucket = var!("CHARTED_STORAGE_S3_BUCKET", or_else_do: |_| panic!("Missing required environment variable: `CHARTED_STORAGE_S3_BUCKET`."));
                    let config = S3StorageConfig::default()
                        .with_enable_signer_v4_requests(enable_signer_v4_requests)
                        .with_enforce_path_access_style(enforce_path_access_style)
                        .with_default_bucket_acl(Some(default_bucket_acl))
                        .with_default_object_acl(Some(default_object_acl))
                        .with_secret_access_key(secret_access_key)
                        .with_access_key_id(access_key_id)
                        .with_region(Some(region))
                        .with_bucket(bucket)
                        .seal();

                    StorageConfig::S3(config)
                }

                _ => StorageConfig::default(),
            },
        }
    }
}
