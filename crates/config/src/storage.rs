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

use crate::helpers;
use azalia::{
    config::{env, merge::Merge, TryFromEnv},
    TRUTHY_REGEX,
};
use eyre::{eyre, Context, Report};
use remi_azure::{CloudLocation, Credential};
use remi_s3::aws::s3::{
    config::Region,
    types::{BucketCannedAcl, ObjectCannedAcl},
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, path::PathBuf, str::FromStr};

/// Configures the storage for holding external media and chart indexes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Config {
    /// Uses the local filesystem to store external media and chart indexes.
    Filesystem(azalia::remi::fs::StorageConfig),

    /// Uses Microsoft's [Azure Blob Storage](https://azure.microsoft.com/en-us/products/storage/blobs) to store
    /// external media and chart indexes.
    Azure(azalia::remi::azure::StorageConfig),

    /// Uses Amazon's Simple Storage Service (S3) service or a S3-compatible server to store
    /// external media and chart indexes.
    S3(azalia::remi::s3::StorageConfig),
}

macro_rules! merge_tuple {
    ($first:expr, $second:expr, copyable) => {
        match ($first, $second) {
            (Some(obj1), Some(obj2)) if obj1 != obj2 => {
                $first = Some(obj2);
            }

            (None, Some(obj)) => {
                $first = Some(obj);
            }

            _ => {}
        }
    };

    ($first:expr, $second:expr) => {
        match (&($first), &($second)) {
            (Some(obj1), Some(obj2)) if obj1 != obj2 => {
                $first = Some(obj2.clone());
            }

            (None, Some(obj)) => {
                $first = Some(obj.clone());
            }

            _ => {}
        }
    };
}

impl Merge for Config {
    fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::Filesystem(fs1), Self::Filesystem(fs2)) => {
                fs1.directory.merge(fs2.directory);
            }

            (Self::Azure(me), Self::Azure(other)) => {
                me.container.merge(other.container);

                match (&me.location, &other.location) {
                    (CloudLocation::Public(acc1), CloudLocation::Public(acc2)) if acc1 != acc2 => {
                        me.location = CloudLocation::Public(acc2.clone());
                    }

                    (CloudLocation::China(acc1), CloudLocation::China(acc2)) if acc1 != acc2 => {
                        me.location = CloudLocation::China(acc2.clone());
                    }

                    (
                        CloudLocation::Emulator {
                            address: addr1,
                            port: port1,
                        },
                        CloudLocation::Emulator {
                            address: addr2,
                            port: port2,
                        },
                    ) if addr1 != addr2 || port1 != port2 => {
                        me.location = CloudLocation::Emulator {
                            address: addr2.clone(),
                            port: *port2,
                        };
                    }

                    (_, other) => {
                        me.location = other.clone();
                    }
                }

                match (&me.credentials, &other.credentials) {
                    (
                        Credential::AccessKey {
                            account: acc1,
                            access_key: ak1,
                        },
                        Credential::AccessKey { account, access_key },
                    ) if acc1 != account || access_key != ak1 => {
                        me.credentials = Credential::AccessKey {
                            account: account.clone(),
                            access_key: access_key.clone(),
                        };
                    }

                    (Credential::SASToken(token1), Credential::SASToken(token2)) if token1 != token2 => {
                        me.credentials = Credential::SASToken(token2.to_owned());
                    }

                    (Credential::Bearer(token1), Credential::Bearer(token2)) if token1 != token2 => {
                        me.credentials = Credential::SASToken(token2.to_owned());
                    }

                    (Credential::Anonymous, Credential::Anonymous) => {}

                    // overwrite if they aren't the same at all
                    (_, other) => {
                        me.credentials = other.clone();
                    }
                };
            }

            (Self::S3(me), Self::S3(other)) => {
                azalia::config::merge::strategy::bool::only_if_falsy(
                    &mut me.enable_signer_v4_requests,
                    other.enable_signer_v4_requests,
                );

                azalia::config::merge::strategy::bool::only_if_falsy(
                    &mut me.enforce_path_access_style,
                    other.enforce_path_access_style,
                );

                merge_tuple!(me.default_bucket_acl, other.default_bucket_acl);
                merge_tuple!(me.default_object_acl, other.default_object_acl);

                me.secret_access_key.merge(other.secret_access_key);
                me.access_key_id.merge(other.access_key_id);

                merge_tuple!(me.app_name, other.app_name);
                merge_tuple!(me.endpoint, other.endpoint);
                merge_tuple!(me.region, other.region);

                me.bucket.merge(other.bucket);
            }

            (me, other) => {
                *me = other;
            }
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::Filesystem(remi_fs::StorageConfig {
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
                "filesystem" | "fs" => Ok(Config::Filesystem(remi_fs::StorageConfig {
                    directory: helpers::env_from_str("CHARTED_STORAGE_FILESYSTEM_DIRECTORY", PathBuf::from("./data"))?,
                })),

                "azure" => Ok(Config::Azure(remi_azure::StorageConfig {
                    credentials: to_env_credentials()?,
                    location: to_env_location()?,
                    container: env!("CHARTED_STORAGE_AZURE_CONTAINER", optional).unwrap_or("charted".into()),
                })),

                "s3" => Ok(Config::S3(remi_s3::StorageConfig {
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
                    bucket: env!("CHARTED_STORAGE_S3_BUCKET", optional).unwrap_or("charted".into()),
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

fn to_env_location() -> eyre::Result<CloudLocation> {
    match env!("CHARTED_STORAGE_AZURE_LOCATION") {
        Ok(res) => match res.as_str() {
            "public" => Ok(CloudLocation::Public(
                env!("CHARTED_STORAGE_AZURE_ACCOUNT")
                    .context("missing required env [CHARTED_STORAGE_AZURE_ACCOUNT]")?,
            )),

            "china" => Ok(CloudLocation::China(
                env!("CHARTED_STORAGE_AZURE_ACCOUNT")
                    .context("missing required env [CHARTED_STORAGE_AZURE_ACCOUNT]")?,
            )),

            "emulator" => Ok(CloudLocation::Emulator {
                address: env!("CHARTED_STORAGE_AZURE_EMULATOR_ADDRESS")
                    .context("missing required env [CHARTED_STORAGE_AZURE_EMULATOR_ADDRESS]")?,

                port: helpers::env_from_str("CHARTED_STORAGE_AZURE_EMULATOR_PORT", 10000u16)?,
            }),

            "custom" => Ok(CloudLocation::Custom {
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
