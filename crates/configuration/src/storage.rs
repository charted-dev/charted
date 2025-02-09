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

use azalia::{
    config::{env, merge::Merge, TryFromEnv},
    remi,
};
use eyre::bail;
use serde::{Deserialize, Serialize};
use std::{env::VarError, path::PathBuf};

const SERVICE: &str = "CHARTED_STORAGE_SERVICE";

/// ## `[storage]` table
/// Configures how the API server stores data like chart indexes,
/// images for user avatars, repository/organization icons, and much
/// more.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Config {
    /// Alows the API server to use the local filesystem to store metadata in.
    Filesystem(remi::fs::StorageConfig),

    /// Alows the API server to use Microsoft's Azure Blob Storage service to store metadata in.
    Azure(remi::azure::StorageConfig),

    /// Alows the API server to use Amazon S3 (or any compatible service) to store metadata in.
    S3(remi::s3::StorageConfig),
}

impl Default for Config {
    fn default() -> Config {
        Config::Filesystem(remi::fs::StorageConfig {
            directory: PathBuf::from("./data"),
        })
    }
}

impl TryFromEnv for Config {
    type Output = Self;
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        match env!(SERVICE) {
            Ok(input) => match &*input.to_ascii_lowercase() {
                "filesystem" | "fs" => Ok(Config::Filesystem(remi::fs::StorageConfig {
                    directory: env!(
                        filesystem::DIRECTORY,
                        |val| PathBuf::from(val);
                        or PathBuf::from("./data")
                    ),
                })),

                "azure" => Ok(Config::Azure(azure::create_config()?)),
                "s3" => Ok(Config::S3(s3::create_config()?)),
                v => bail!(
                    "environment variable `${}` received an invalid input: expected either `filesystem`, `fs`, `s3`, or `azure`; received {} instead",
                    SERVICE,
                    v
                )
            },

            Err(VarError::NotPresent) => Ok(Default::default()),
            Err(VarError::NotUnicode(_)) => bail!("environment variable `${}` received invalid unicode", SERVICE),
        }
    }
}

impl Merge for Config {
    fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::Filesystem(fs1), Self::Filesystem(fs2)) => {
                fs1.directory.merge(fs2.directory);
            }

            (Self::Azure(azure1), Self::Azure(azure2)) => {
                azure::merge_config(azure1, azure2);
            }

            (Self::S3(s3_1), Self::S3(s3_2)) => {
                s3::merge_config(s3_1, s3_2);
            }

            (me, other) => {
                *me = other;
            }
        }
    }
}

pub(crate) mod filesystem {
    pub const DIRECTORY: &str = "CHARTED_STORAGE_FILESYSTEM_DIRECTORY";
}

pub(crate) mod s3 {
    use azalia::{
        config::{env, merge::Merge},
        remi::{
            self,
            s3::aws::{
                config::Region,
                s3::types::{BucketCannedAcl, ObjectCannedAcl},
            },
        },
        TRUTHY_REGEX,
    };
    use eyre::eyre;
    use std::{borrow::Cow, env::VarError, str::FromStr};

    pub const ENABLE_SIGNER_V4_REQUESTS: &str = "CHARTED_STORAGE_S3_ENABLE_SIGNER_V4_REQUESTS";
    pub const ENFORCE_PATH_ACCESS_STYLE: &str = "CHARTED_STORAGE_S3_ENFORCE_PATH_ACCESS_STYLE";
    pub const DEFAULT_OBJECT_ACL: &str = "CHARTED_STORAGE_S3_DEFAULT_OBJECT_ACL";
    pub const DEFAULT_BUCKET_ACL: &str = "CHARTED_STORAGE_S3_DEFAULT_BUCKET_ACL";
    pub const SECRET_ACCESS_KEY: &str = "CHARTED_STORAGE_S3_SECRET_ACCESS_KEY";
    pub const ACCESS_KEY_ID: &str = "CHARTED_STORAGE_S3_ACCESS_KEY_ID";
    pub const APP_NAME: &str = "CHARTED_STORAGE_S3_APP_NAME";
    pub const ENDPOINT: &str = "CHARTED_STORAGE_S3_ENDPOINT";
    pub const PREFIX: &str = "CHARTED_STORAGE_S3_PREFIX";
    pub const REGION: &str = "CHARTED_STORAGE_S3_REGION";
    pub const BUCKET: &str = "CHARTED_STORAGE_S3_BUCKET";

    const DEFAULT_OBJECT_CANNED_ACL: ObjectCannedAcl = ObjectCannedAcl::BucketOwnerFullControl;
    const DEFAULT_BUCKET_CANNED_ACL: BucketCannedAcl = BucketCannedAcl::AuthenticatedRead;

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

    pub fn create_config() -> eyre::Result<remi::s3::StorageConfig> {
        Ok(remi::s3::StorageConfig {
            enable_signer_v4_requests: env!(ENABLE_SIGNER_V4_REQUESTS, |val| TRUTHY_REGEX.is_match(&val); or false),
            enforce_path_access_style: env!(ENFORCE_PATH_ACCESS_STYLE, |val| TRUTHY_REGEX.is_match(&val); or false),
            default_object_acl: env!(DEFAULT_OBJECT_ACL, |val| ObjectCannedAcl::from_str(val.as_str()).ok(); or Some(DEFAULT_OBJECT_CANNED_ACL)),
            default_bucket_acl: env!(DEFAULT_BUCKET_ACL, |val| BucketCannedAcl::from_str(val.as_str()).ok(); or Some(DEFAULT_BUCKET_CANNED_ACL)),
            secret_access_key: env!(SECRET_ACCESS_KEY).map_err(|e| match e {
                VarError::NotPresent => {
                    eyre!("you're required to add the [{SECRET_ACCESS_KEY}] environment variable")
                }

                VarError::NotUnicode(_) => eyre!("wanted valid UTF-8 for env `{SECRET_ACCESS_KEY}`"),
            })?,

            access_key_id: env!(ACCESS_KEY_ID).map_err(|e| match e {
                VarError::NotPresent => {
                    eyre!("you're required to add the [{ACCESS_KEY_ID}] environment variable")
                }

                VarError::NotUnicode(_) => eyre!("wanted valid UTF-8 for env `{ACCESS_KEY_ID}`"),
            })?,

            app_name: env!(APP_NAME, optional),
            endpoint: env!(ENDPOINT, optional),
            prefix: env!(PREFIX, optional),
            region: env!(REGION, |val| Some(Region::new(Cow::Owned(val))); or Some(Region::new(Cow::Borrowed("us-east-1")))),
            bucket: env!(BUCKET, optional).unwrap_or("ume".into()),
        })
    }

    pub fn merge_config(me: &mut remi::s3::StorageConfig, other: remi::s3::StorageConfig) {
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
}

pub(crate) mod azure {
    use azalia::{
        config::{env, merge::Merge},
        remi::{
            self,
            azure::{CloudLocation, Credential},
        },
    };
    use eyre::{eyre, Context};
    use std::env::VarError;

    pub const ACCESS_KEY_ACCOUNT: &str = "CHARTED_STORAGE_AZURE_CREDENTIAL_ACCESSKEY_ACCOUNT";
    pub const ACCESS_KEY: &str = "CHARTED_STORAGE_AZURE_CREDENTIAL_ACCESSKEY";
    pub const SAS_TOKEN: &str = "CHARTED_STORAGE_AZURE_CREDENTIAL_SAS_TOKEN";
    pub const BEARER: &str = "CHARTED_STORAGE_AZURE_CREDENTIAL_BEARER";

    pub const ACCOUNT: &str = "CHARTED_STORAGE_AZURE_ACCOUNT";
    pub const URI: &str = "CHARTED_STORAGE_AZURE_URI";

    pub const CREDENTIAL: &str = "CHARTED_STORAGE_AZURE_CREDENTIAL";
    pub const CONTAINER: &str = "CHARTED_STORAGE_AZURE_CONTAINER";
    pub const LOCATION: &str = "CHARTED_STORAGE_AZURE_LOCATION";

    pub fn create_config() -> eyre::Result<remi::azure::StorageConfig> {
        Ok(remi::azure::StorageConfig {
            credentials: create_credentials_config()?,
            location: create_location()?,
            container: env!(CONTAINER, optional).unwrap_or("ume".into()),
        })
    }

    pub fn merge_config(me: &mut remi::azure::StorageConfig, other: remi::azure::StorageConfig) {
        me.container.merge(other.container);

        match (&me.location, &other.location) {
            (CloudLocation::Public(acc1), CloudLocation::Public(acc2)) if acc1 != acc2 => {
                me.location = CloudLocation::Public(acc2.clone());
            }

            (CloudLocation::China(acc1), CloudLocation::China(acc2)) if acc1 != acc2 => {
                me.location = CloudLocation::China(acc2.clone());
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
        }
    }

    fn create_credentials_config() -> eyre::Result<remi::azure::Credential> {
        match env!(CREDENTIAL) {
            Ok(input) => match &*input.to_ascii_lowercase() {
                "anonymous" | "anon" | "" => Ok(remi::azure::Credential::Anonymous),
                "accesskey" | "access_key" | "access-key" => Ok(remi::azure::Credential::AccessKey {
                    account: env!(ACCESS_KEY_ACCOUNT).with_context(|| format!("missing required environment variable when `${CREDENTIAL}` is set to Access Key: `${ACCESS_KEY_ACCOUNT}`"))?,
                    access_key: env!(ACCESS_KEY).with_context(|| format!("missing required environment variable when `${CREDENTIAL}` is set to Access Key: `${ACCESS_KEY}`"))?
                }),

                "sastoken" | "sas-token" | "sas_token" => Ok(remi::azure::Credential::SASToken(
                    env!(SAS_TOKEN).with_context(|| format!("missing required environment variable when `${CREDENTIAL}` is set to SAS Token: `${SAS_TOKEN}`"))?
                )),

                "bearer" => Ok(remi::azure::Credential::Bearer(
                    env!(SAS_TOKEN).with_context(|| format!("missing required environment variable when `${CREDENTIAL}` is set to SAS Token: `${BEARER}`"))?
                )),

                input => Err(eyre!("unknown input [{input}] for `${CREDENTIAL}` environment variable"))
            },

            Err(VarError::NotPresent) => Ok(remi::azure::Credential::Anonymous),
            Err(VarError::NotUnicode(_)) => Err(eyre!("environment variable `${CREDENTIAL}` was invalid utf-8"))
        }
    }

    fn create_location() -> eyre::Result<CloudLocation> {
        match env!(LOCATION) {
            Ok(res) => match &*res.to_ascii_lowercase() {
                "public" | "" => {
                    Ok(CloudLocation::Public(env!(ACCOUNT).with_context(|| {
                        format!("missing required environment variable: [{ACCOUNT}]")
                    })?))
                }

                "china" => {
                    Ok(CloudLocation::China(env!(ACCOUNT).with_context(|| {
                        format!("missing required environment variable: [{ACCOUNT}]")
                    })?))
                }

                "custom" => Ok(CloudLocation::Custom {
                    account: env!(ACCOUNT)
                        .with_context(|| format!("missing required environment variable: [{ACCOUNT}]"))?,

                    uri: env!(URI).with_context(|| format!("missing required environment variable: [{ACCOUNT}]"))?,
                }),

                input => Err(eyre!(
                    "invalid option given: {input} | expected [public, china, custom]"
                )),
            },

            Err(VarError::NotPresent) => Err(eyre!("missing required environment variable: [{LOCATION}]")),
            Err(VarError::NotUnicode(_)) => Err(eyre!("environment variable [{LOCATION}] was not in valid unicode")),
        }
    }
}
