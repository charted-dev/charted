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
    config::{
        env::{self, TryFromEnv},
        merge::Merge,
    },
    remi,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

    /// Alows the API server to use Microsoft's Azure Blob Storage service to store
    /// metadata in.
    Azure(remi::azure::StorageConfig),

    /// Alows the API server to use Amazon S3 (or any compatible service) to store
    /// metadata in.
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
    type Error = eyre::Report;

    fn try_from_env() -> Result<Self, Self::Error> {
        crate::impl_enum_based_env_value!(SERVICE, {
            on match fail: |input| "environment variable `${}` is not invalid: expected `filesystem`, `s3`, or `azure`, received `{}` instead" [SERVICE, input];

            "filesystem" | "fs" | "" => Ok(Config::Filesystem(remi::fs::StorageConfig {
                directory: env::try_parse(filesystem::DIRECTORY)?
            }));

            "azure" => Ok(Config::Azure(azure::create_config()?));
            "s3" => Ok(Config::S3(s3::create_config()?));
        })
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
    use crate::util;
    use azalia::{
        config::{
            env::{self, TryFromEnvValue},
            merge::Merge,
        },
        remi::{
            self,
            s3::aws::{
                config::Region,
                s3::types::{BucketCannedAcl, ObjectCannedAcl},
            },
        },
    };
    use std::{borrow::Cow, convert::Infallible};

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

    struct RegionEnv(Region);
    impl TryFromEnvValue for RegionEnv {
        type Error = Infallible;

        fn try_from_env_value(value: String) -> Result<Self, Self::Error> {
            Ok(RegionEnv(Region::new(value)))
        }
    }

    pub fn create_config() -> eyre::Result<remi::s3::StorageConfig> {
        Ok(remi::s3::StorageConfig {
            enable_signer_v4_requests: util::bool_env(ENABLE_SIGNER_V4_REQUESTS)?,
            enforce_path_access_style: util::bool_env(ENFORCE_PATH_ACCESS_STYLE)?,
            default_bucket_acl: util::env_from_str(DEFAULT_BUCKET_ACL, DEFAULT_BUCKET_CANNED_ACL).map(Some)?,
            default_object_acl: util::env_from_str(DEFAULT_OBJECT_ACL, DEFAULT_OBJECT_CANNED_ACL).map(Some)?,
            secret_access_key: env::try_parse(SECRET_ACCESS_KEY)?,
            access_key_id: env::try_parse(ACCESS_KEY_ID)?,
            app_name: env::try_parse_optional(APP_NAME)?,
            endpoint: env::try_parse_optional(ENDPOINT)?,
            prefix: env::try_parse_optional(PREFIX)?,
            region: env::try_parse_or_else::<_, RegionEnv>(REGION, RegionEnv(Region::new(Cow::Borrowed("us-east-1"))))
                .map(|s| Some(s.0))?,

            bucket: env::try_parse_optional(BUCKET)?.unwrap_or(String::from("charted")),
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
            container: env::try_parse_optional(CONTAINER)?.unwrap_or(String::from("charted")),
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
        crate::impl_enum_based_env_value!(CREDENTIAL, {
            on match fail: |input| "invalid input [{}] for `${}`: expected either `anonymous` (`anon` is accepted as well), \
                `sastoken` (`sas-token`, `sas_token` is accepted as well), \
                `accesskey` (`access-key` and `access_key` is accepted as well) \
                or `bearer`." [input, CREDENTIAL];

            "anonymous" | "anon" | "" => Ok(remi::azure::Credential::Anonymous);
            "accesskey" | "access-key" | "access_key" => Ok(remi::azure::Credential::AccessKey {
                account: env::try_parse(ACCESS_KEY_ACCOUNT)?,
                access_key: env::try_parse(ACCESS_KEY)?
            });

            "sastoken" | "sas-token" | "sas_token" => Ok(remi::azure::Credential::SASToken(env::try_parse(SAS_TOKEN)?));
            "bearer" => Ok(remi::azure::Credential::Bearer(env::try_parse(BEARER)?));
        })
    }

    fn create_location() -> eyre::Result<CloudLocation> {
        crate::impl_enum_based_env_value!(LOCATION, {
            on match fail: |input| "invalid input [{}] for `${}`: expected either: `public`, `china`, or `custom`." [input, LOCATION];

            "public" | "" => Ok(CloudLocation::Public(env::try_parse(ACCOUNT)?));
            "china" => Ok(CloudLocation::China(env::try_parse(ACCOUNT)?));
            "custom" => Ok(CloudLocation::Custom {
                account: env::try_parse(ACCOUNT)?,
                uri: env::try_parse(URI)?
            });
        })
    }
}
