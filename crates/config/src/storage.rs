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

use aws_sdk_s3::{
    config::Region,
    types::{BucketCannedAcl, ObjectCannedAcl},
};
use charted_common::TRUTHY_REGEX;
use eyre::{eyre, Context, Report};
use noelware_config::{env, TryFromEnv};
use remi_azure::Credential;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, path::PathBuf, str::FromStr};

/// Configures the storage for holding external media and chart indexes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Config {
    /// Uses the local filesystem to store external media and chart indexes.
    Filesystem(remi_fs::Config),

    /// Uses Microsoft's [Azure Blob Storage](https://azure.microsoft.com/en-us/products/storage/blobs) to store
    /// external media and chart indexes.
    Azure(remi_azure::StorageConfig),

    /// Uses Amazon's Simple Storage Service (S3) service or a S3-compatible server to store
    /// external media and chart indexes.
    S3(remi_s3::S3StorageConfig),
}

impl Default for Config {
    fn default() -> Config {
        Config::Filesystem(remi_fs::Config {
            directory: PathBuf::from("./data"),
        })
    }
}

impl TryFromEnv for Config {
    type Output = Config;
    type Error = Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        match env!("CHARTED_STORAGE_SERVICE") {
            Ok(res) => match res.to_lowercase().as_str() {
                "filesystem" | "fs" => Ok(Config::Filesystem(remi_fs::Config {
                    directory: charted_common::env(
                        "CHARTED_STORAGE_FILESYSTEM_DIRECTORY",
                        PathBuf::from("./data"),
                        |_| panic!("should never reach here"),
                    )
                    .expect("this should never fail"),
                })),

                "azure" => Ok(Config::Azure(remi_azure::StorageConfig {
                    credentials: to_env_credentials()?,
                    location: to_env_location()?,
                    container: env!("CHARTED_STORAGE_AZURE_CONTAINER", optional).unwrap_or("ume".into()),
                })),

                "s3" => Ok(Config::S3(remi_s3::S3StorageConfig {
                    enable_signer_v4_requests: env!("CHARTED_STORAGE_S3_ENABLE_SIGNER_V4_REQUESTS", |val| TRUTHY_REGEX.is_match(&val); or false),
                    enforce_path_access_style: env!("CHARTED_STORAGE_S3_ENFORCE_PATH_ACCESS_STYLE", |val| TRUTHY_REGEX.is_match(&val); or false),
                    default_object_acl: env!("CHARTED_STORAGE_S3_DEFAULT_OBJECT_ACL", |val| ObjectCannedAcl::from_str(val.as_str()).ok(); or Some(ObjectCannedAcl::BucketOwnerFullControl)),
                    default_bucket_acl: env!("CHARTED_STORAGE_S3_DEFAULT_OBJECT_ACL", |val| BucketCannedAcl::from_str(val.as_str()).ok(); or Some(BucketCannedAcl::AuthenticatedRead)),

                    secret_access_key: env!("CHARTED_STORAGE_S3_SECRET_ACCESS_KEY")
                        .context("required env variable [CHARTED_STORAGE_S3_SECRET_ACCESS_KEY]")?,

                    access_key_id: env!("CHARTED_STORAGE_S3_ACCESS_KEY_ID")
                        .context("required env variable [CHARTED_STORAGE_S3_ACCESS_KEY_ID]")?,

                    app_name: env!("CHARTED_STORAGE_S3_APP_NAME", optional),
                    endpoint: env!("CHARTED_STORAGE_S3_ENDPOINT", optional),
                    prefix: env!("CHARTED_STORAGE_S3_PREFIX", optional),
                    region: env!("CHARTED_STORAGE_S3_REGION", |val| Some(Region::new(Cow::Owned(val))); or Some(Region::new(Cow::Owned("us-east-1".to_owned())))),
                    bucket: env!("CHARTED_STORAGE_S3_BUCKET", optional).unwrap_or("ume".into()),
                })),

                loc => Err(eyre!("expected [filesystem/fs, azure, s3]; received '{loc}'")),
            },
            Err(_) => Ok(Default::default()),
        }
    }
}

fn to_env_credentials() -> eyre::Result<Credential> {
    match env!("CHARTED_STORAGE_AZURE_CREDENTIAL") {
        Ok(res) => match res.as_str() {
            "anonymous" | "anon" => Ok(Credential::Anonymous),
            "accesskey" | "access_key" => Ok(Credential::AccessKey {
                account: env!("CHARTED_STORAGE_AZURE_CREDENTIAL_ACCESSKEY_ACCOUNT")
                    .context("missing required env variable [CHARTED_STORAGE_AZURE_CREDENTIAL_ACCESSKEY_ACCOUNT]")?,
                access_key: env!("CHARTED_STORAGE_AZURE_CREDENTIAL_ACCESSKEY")
                    .context("missing required env variable [CHARTED_STORAGE_AZURE_CREDENTIAL_ACCESSKEY]")?,
            }),

            "sastoken" | "sas_token" => Ok(Credential::SASToken(
                env!("CHARTED_STORAGE_AZURE_CREDENTIAL_SAS_TOKEN")
                    .context("missing required env variable [CHARTED_STORAGE_AZURE_CREDENTIAL_SAS_TOKEN]")?,
            )),

            "bearer" => Ok(Credential::SASToken(
                env!("CHARTED_STORAGE_AZURE_CREDENTIAL_BEARER")
                    .context("missing required env variable [CHARTED_STORAGE_AZURE_CREDENTIAL_BEARER]")?,
            )),

            res => Err(eyre!(
                "expected [anonymous/anon, accesskey/access_key, sastoken/sas_token, bearer]; received '{res}'"
            )),
        },
        Err(_) => Err(eyre!(
            "missing required `CHARTED_STORAGE_AZURE_CREDENTIAL` env or was invalid utf-8"
        )),
    }
}

fn to_env_location() -> eyre::Result<azure_storage::CloudLocation> {
    match env!("CHARTED_STORAGE_AZURE_LOCATION") {
        Ok(res) => match res.as_str() {
            "public" => Ok(azure_storage::CloudLocation::Public {
                account: env!("CHARTED_STORAGE_AZURE_ACCOUNT")
                    .context("missing required env [CHARTED_STORAGE_AZURE_ACCOUNT]")?,
            }),

            "china" => Ok(azure_storage::CloudLocation::China {
                account: env!("CHARTED_STORAGE_AZURE_ACCOUNT")
                    .context("missing required env [CHARTED_STORAGE_AZURE_ACCOUNT]")?,
            }),

            "emulator" => Ok(azure_storage::CloudLocation::Emulator {
                address: env!("CHARTED_STORAGE_AZURE_EMULATOR_ADDRESS")
                    .context("missing required env [CHARTED_STORAGE_AZURE_EMULATOR_ADDRESS]")?,

                port: charted_common::env("CHARTED_STORAGE_AZURE_EMULATOR_PORT", 10000u16, |err| {
                    Cow::Owned(err.to_string())
                })?,
            }),

            "custom" => Ok(azure_storage::CloudLocation::Custom {
                account: env!("CHARTED_STORAGE_AZURE_ACCOUNT")
                    .context("missing required env [CHARTED_STORAGE_AZURE_ACCOUNT]")?,

                uri: env!("CHARTED_STORAGE_AZURE_URI").context("missing required env [CHARTED_STORAGE_AZURE_URI]")?,
            }),

            loc => Err(eyre!("expected [public, china, emulator, custom]; received '{loc}'")),
        },

        Err(_) => Err(eyre!(
            "missing required `CHARTED_STORAGE_AZURE_LOCATION` env or was invalid utf-8"
        )),
    }
}
