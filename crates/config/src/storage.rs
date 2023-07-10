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

use std::{borrow::Cow, str::FromStr};

use crate::{var, FromEnv, Merge};
use aws_sdk_s3::{
    config::Region,
    types::{BucketCannedAcl, ObjectCannedAcl},
};
use charted_common::TRUTHY_REGEX;
use clap::{arg, error::ErrorKind, ArgGroup, ArgMatches, Args, Error, FromArgMatches};
use remi_fs::FilesystemStorageConfig;
use remi_s3::{S3StorageConfig, S3StorageConfigBuilder};
use serde::{Deserialize, Serialize};

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
        StorageConfig::Filesystem(FilesystemStorageConfig::builder().directory(datadir).build().unwrap())
    }
}

impl FromArgMatches for StorageConfig {
    fn from_arg_matches(matches: &ArgMatches) -> Result<StorageConfig, Error> {
        let binding = String::from("filesystem");
        let storage_type = matches.get_one::<String>("storage-service").unwrap_or(&binding);

        match storage_type.as_str() {
            "filesystem" | "fs" => {
                let directory = matches.get_one::<String>("storage-fs-directory").ok_or_else(|| {
                    clap::Error::raw(
                        ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: `storage-fs-directory`",
                    )
                })?;

                let config = FilesystemStorageConfig::builder()
                    .directory(directory.to_string())
                    .build()
                    .unwrap();

                Ok(StorageConfig::Filesystem(config))
            }

            "s3" => {
                let enable_signer_v4_requests = matches
                    .get_one::<bool>("storage-s3-enable-signer-v4-requests")
                    .unwrap_or(&false);

                let enforce_path_access_style = matches
                    .get_one::<bool>("storage-s3-enforce-path-access-style")
                    .unwrap_or(&false);

                let default_object_acl = match matches.get_one::<String>("storage-s3-default-object-acl") {
                    Some(val) => {
                        let res = ObjectCannedAcl::from_str(val.as_str()).unwrap();
                        if let ObjectCannedAcl::Unknown(val) = res {
                            return Err(Error::raw(
                                ErrorKind::UnknownArgument,
                                format!(
                                    "Unknown canned ACL for objects: [{val:?}]; expected any: [{}]",
                                    ObjectCannedAcl::values().join(", ")
                                )
                                .as_str(),
                            ));
                        }

                        res
                    }

                    None => ObjectCannedAcl::PublicRead,
                };

                let default_bucket_acl = match matches.get_one::<String>("storage-s3-default-bucket-acl") {
                    Some(val) => {
                        let res = BucketCannedAcl::from_str(val.as_str()).unwrap();
                        if let BucketCannedAcl::Unknown(val) = res {
                            return Err(Error::raw(
                                ErrorKind::UnknownArgument,
                                format!(
                                    "Unknown canned ACL for buckets: [{val:?}]; expected any: [{}]",
                                    BucketCannedAcl::values().join(", ")
                                )
                                .as_str(),
                            ));
                        }

                        res
                    }

                    None => BucketCannedAcl::PublicRead,
                };

                let secret_access_key = matches
                    .get_one::<String>("storage-s3-secret-access-key")
                    .ok_or_else(|| {
                        Error::raw(
                            ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: storage-s3-secret-access-key",
                        )
                    })?;

                let access_key_id = matches.get_one::<String>("storage-s3-access-key-id").ok_or_else(|| {
                    Error::raw(
                        ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: storage-s3-access-key-id",
                    )
                })?;

                let region = matches
                    .get_one::<String>("storage-s3-region")
                    .map(|val| Region::new(Cow::Owned(val.clone())))
                    .unwrap_or_else(|| Region::new(Cow::Owned("us-east-1".to_owned())));

                let bucket = matches.get_one::<String>("storage-s3-bucket").ok_or_else(|| {
                    Error::raw(
                        ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: storage-s3-bucket",
                    )
                })?;

                let config = S3StorageConfig::builder()
                    .enable_signer_v4_requests(*enable_signer_v4_requests)
                    .enforce_path_access_style(*enforce_path_access_style)
                    .default_bucket_acl(Some(default_bucket_acl))
                    .default_object_acl(Some(default_object_acl))
                    .secret_access_key((*secret_access_key.clone()).to_string())
                    .access_key_id((*access_key_id.clone()).to_string())
                    .app_name(Some("charted-server".into()))
                    .region(Some(region))
                    .bucket((*bucket.clone()).to_string())
                    .build()
                    .unwrap();

                Ok(StorageConfig::S3(config))
            }

            _ => Err(Error::raw(
                ErrorKind::UnknownArgument,
                format!("Unknown storage type: [{storage_type}]; expected: 'filesystem' or 's3'").as_str(),
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        if matches.contains_id("storage-service") {
            match matches.get_one::<String>("storage-service") {
                Some(ty) => match ty.as_str() {
                    "filesystem" | "fs" => {
                        if matches.contains_id("storage-fs-directory") {
                            let directory = matches.get_one::<String>("storage-fs-directory").ok_or_else(|| {
                                Error::raw(
                                    ErrorKind::MissingRequiredArgument,
                                    "The following required argument was not provided: storage-fs-directory",
                                )
                            })?;

                            let config = FilesystemStorageConfig::builder()
                                .directory(directory.to_string())
                                .build()
                                .unwrap();

                            *self = StorageConfig::Filesystem(config);
                        }

                        Ok(())
                    }

                    "s3" => {
                        let secret_access_key =
                            matches
                                .get_one::<String>("storage-s3-secret-access-key")
                                .ok_or_else(|| {
                                    Error::raw(
                                    ErrorKind::MissingRequiredArgument,
                                    "The following required argument was not provided: storage-s3-secret-access-key",
                                )
                                })?;

                        let access_key_id = matches.get_one::<String>("storage-s3-access-key-id").ok_or_else(|| {
                            Error::raw(
                                ErrorKind::MissingRequiredArgument,
                                "The following required argument was not provided: storage-s3-access-key-id",
                            )
                        })?;

                        let bucket = matches.get_one::<String>("storage-s3-bucket").ok_or_else(|| {
                            Error::raw(
                                ErrorKind::MissingRequiredArgument,
                                "The following required argument was not provided: storage-s3-bucket",
                            )
                        })?;

                        let config = S3StorageConfigBuilder::default()
                            .secret_access_key((*secret_access_key.clone()).to_string())
                            .access_key_id((*access_key_id.clone()).to_string())
                            .app_name(Some("charted-server".into()))
                            .bucket((*bucket.clone()).to_string())
                            .build()
                            .unwrap();

                        *self = StorageConfig::S3(config);
                        Ok(())
                    }
                    _ => Err(Error::raw(
                        ErrorKind::UnknownArgument,
                        format!("unknown type: [{ty}]; expected 'filesystem' or 's3'").as_str(),
                    )),
                },
                None => Ok(()),
            }
        } else {
            Err(Error::raw(
                ErrorKind::MissingRequiredArgument,
                "The following required argument was not provided: storage-s3-bucket",
            ))
        }
    }
}

impl Args for StorageConfig {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("StorageConfig"))
    }

    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.group(
            ArgGroup::new("StorageConfig").multiple(true).args(
                [
                    clap::Id::from("storage-service"),
                    clap::Id::from("storage-fs-directory"),
                    clap::Id::from("storage-s3-enable-signer-v4-requests"),
                    clap::Id::from("storage-s3-enforce-path-access-style"),
                    clap::Id::from("storage-s3-default-object-acl"),
                    clap::Id::from("storage-s3-default-bucket-acl"),
                    clap::Id::from("storage-s3-secret-access-key"),
                    clap::Id::from("storage-s3-access-key-id"),
                    clap::Id::from("storage-s3-region"),
                    clap::Id::from("storage-s3-bucket"),
                ]
                .iter(),
            ),
        )
        .arg(arg!(--"storage-service" <SERVICE> "Storage service to configure.").required(false).default_value("filesystem"))
        .arg(arg!(--"storage-fs-directory" [DIRECTORY] "Sets the directory to use to store Helm charts in. Only applicable with '--storage-service=filesystem'").required(false).default_value("./data"))
        .arg(arg!(--"storage-s3-enable-signer-v4-requests" "Enforces the AWS SDK to enable AWS Signer V4 Requests.").required(false))
        .arg(arg!(--"storage-s3-enforce-path-access-style" "Enables the AWS SDK to use the new Path Access Style (<host>/<bucket>/...) instead of the legacy, domain style (<bucket>.<host>/...). Only applicable with '--storage-service=s3'")
            .long_help("Enables the AWS SDK to use the new Path Access Style (<host>/<bucket>/...) instead of the legacy style (<bucket>.<host>/...) \
                   This is recommended to set when using MinIO instances rather than AWS' official product.")
            .required(false)
        )
        .arg(arg!(--"storage-s3-default-object-acl" [ACL] "ACL to use when creating new S3 objects").required(false).default_value("public-read"))
        .arg(arg!(--"storage-s3-default-bucket-acl" [ACL] "ACL to use when creating new S3 buckets").required(false).default_value("public-read"))
        .arg(arg!(--"storage-s3-access-key-id" <KEY> "Access key ID to use for authenticating to AWS S3. Required if '--storage-service=s3'").required(false))
        .arg(arg!(--"storage-s3-secret-access-key" <KEY> "Secret access key to use for authenticating to AWS S3. Required if '--storage-service=s3'").required(false))
        .arg(arg!(--"storage-s3-region" <REGION> "Region to set where the bucket currently lives in. For MinIO users, the default 'us-east-1' region is sufficient enough if on a single node.").required(false).default_value("us-east-1"))
        .arg(arg!(--"storage-s3-bucket" <BUCKET> "Bucket to use to store Helm charts in. Required if '--storage-service=s3'").required(false))
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        cmd.group(
            ArgGroup::new("StorageConfig").multiple(true).args(
                [
                    clap::Id::from("storage-service"),
                    clap::Id::from("storage-fs-directory"),
                    clap::Id::from("storage-s3-enable-signer-v4-requests"),
                    clap::Id::from("storage-s3-enforce-path-access-style"),
                    clap::Id::from("storage-s3-default-object-acl"),
                    clap::Id::from("storage-s3-default-bucket-acl"),
                    clap::Id::from("storage-s3-secret-access-key"),
                    clap::Id::from("storage-s3-access-key-id"),
                    clap::Id::from("storage-s3-region"),
                    clap::Id::from("storage-s3-bucket"),
                ]
                .iter(),
            ),
        )
        .arg(arg!(--"storage-service" <SERVICE> "Storage service to configure.").required(false).default_value("filesystem"))
        .arg(arg!(--"storage-fs-directory" [DIRECTORY] "Sets the directory to use to store Helm charts in. Only applicable with '--storage-service=filesystem'").required(false))
        .arg(arg!(--"storage-s3-enable-signer-v4-requests" "Enforces the AWS SDK to enable AWS Signer V4 Requests.").required(false))
        .arg(arg!(--"storage-s3-enforce-path-access-style" "Enables the AWS SDK to use the new Path Access Style (<host>/<bucket>/...) instead of the legacy, domain style (<bucket>.<host>/...). Only applicable with '--storage-service=s3'")
            .long_help("Enables the AWS SDK to use the new Path Access Style (<host>/<bucket>/...) instead of the legacy style (<bucket>.<host>/...) \
                   This is recommended to set when using MinIO instances rather than AWS' official product.")
            .required(false)
        )
        .arg(arg!(--"storage-s3-default-object-acl" [ACL] "ACL to use when creating new S3 objects").required(false).default_value("public-read"))
        .arg(arg!(--"storage-s3-default-bucket-acl" [ACL] "ACL to use when creating new S3 buckets").required(false).default_value("public-read"))
        .arg(arg!(--"storage-s3-access-key-id <KEY>" "Access key ID to use for authenticating to AWS S3. Required if '--storage-service=s3'").required(false))
        .arg(arg!(--"storage-s3-secret-access-key <KEY>" "Secret access key to use for authenticating to AWS S3. Required if '--storage-service=s3'").required(false))
        .arg(arg!(--"storage-s3-region <REGION>" "Region to set where the bucket currently lives in. For MinIO users, the default 'us-east-1' region is sufficient enough if on a single node.").required(false).default_value("us-east-1"))
        .arg(arg!(--"storage-s3-bucket <BUCKET>" "Bucket to use to store Helm charts in. Required if '--storage-service=s3'").required(false))
    }
}

impl FromEnv<StorageConfig> for StorageConfig {
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
                        or_else: ObjectCannedAcl::PublicRead;
                        mapper: |p| ObjectCannedAcl::from_str(p.as_str()).unwrap();
                    });

                    let default_bucket_acl = var!("CHARTED_STORAGE_S3_DEFAULT_BUCKET_ACL", {
                        or_else: BucketCannedAcl::PublicRead;
                        mapper: |p| BucketCannedAcl::from_str(p.as_str()).unwrap();
                    });

                    let secret_access_key = var!("CHARTED_STORAGE_S3_SECRET_ACCESS_KEY", or_else_do: |_| panic!("Missing required environment variable: `CHARTED_STORAGE_S3_SECRET_ACCESS_KEY`."));
                    let access_key_id = var!("CHARTED_STORAGE_S3_ACCESS_KEY_ID", or_else_do: |_| panic!("Missing required environment variable: `CHARTED_STORAGE_S3_ACCESS_KEY_ID`."));
                    let region = var!("CHARTED_STORAGE_S3_REGION", {
                        or_else: Region::new(Cow::Owned("us-east-1".to_owned()));
                        mapper: |val| Region::new(Cow::Owned(val.clone()));
                    });

                    let bucket = var!("CHARTED_STORAGE_S3_BUCKET", or_else_do: |_| panic!("Missing required environment variable: `CHARTED_STORAGE_S3_BUCKET`."));
                    let config = S3StorageConfig::builder()
                        .enable_signer_v4_requests(enable_signer_v4_requests)
                        .enforce_path_access_style(enforce_path_access_style)
                        .default_bucket_acl(Some(default_bucket_acl))
                        .default_object_acl(Some(default_object_acl))
                        .secret_access_key(secret_access_key)
                        .access_key_id(access_key_id)
                        .region(Some(region))
                        .bucket(bucket)
                        .build()
                        .unwrap();

                    StorageConfig::S3(config)
                }

                _ => StorageConfig::default(),
            },
        }
    }
}

// macro_rules! gen_merge_rules {
//     ($self:expr, $other:expr, $($name:ident,)+) => {
//         $($self.$name().clone().merge($other.$name());)*
//     };
// }

impl Merge for StorageConfig {
    fn merge(&mut self, _other: Self) {
        // match self {
        //     StorageConfig::Filesystem(fs) => {
        //         if let StorageConfig::Filesystem(other_fs) = other {
        //             gen_merge_rules!(fs, other_fs, directory,);
        //         }
        //     }

        //     StorageConfig::S3(s3) => {
        //         if let StorageConfig::S3(other_s3) = other {
        //             gen_merge_rules!(
        //                 s3,
        //                 other_s3,
        //                 enable_signer_v4_requests,
        //                 enforce_path_access_style,
        //                 default_bucket_acl,
        //                 default_object_acl,
        //                 secret_access_key,
        //                 access_key_id,
        //                 app_name,
        //                 endpoint,
        //                 prefix,
        //                 region,
        //                 bucket,
        //             );
        //         }
        //     }
        // }
    }
}
