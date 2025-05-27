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

use crate::{Env, ext::ResultExt, mk_into_responses, openapi::UrlResponse};
use axum::{
    body::Bytes,
    http::{HeaderMap, HeaderValue, StatusCode, header},
};
use charted_core::api;
use charted_datastore::{
    DataStore, Namespace, fs,
    remi::{Blob, File, StorageService},
};
use charted_types::{Organization, Repository, Ulid, User};
use serde_json::json;
use utoipa::{
    IntoParams, PartialSchema,
    openapi::{
        Content, Object, RefOr, Response,
        path::{Parameter, ParameterIn},
        schema::SchemaType,
    },
};

pub type ReturnType = ([(header::HeaderName, header::HeaderValue); 1], Bytes);

charted_core::assert_into_response!(ReturnType);

pub struct GetAvatarR;
mk_into_responses!(for GetAvatarR {
    "200" => [custom(
        Response::builder()
            .description("byte-array of the avatar/icon")
            .content(
                "image/png",
                Content::builder()
                    .schema(Some(RefOr::T(utoipa::openapi::Schema::Object(
                        Object::builder()
                            .schema_type(SchemaType::AnyValue)
                            .build()
                    ))))
                    .build()
            )
            .content(
                "image/jpeg",
                Content::builder()
                    .schema(Some(RefOr::T(utoipa::openapi::Schema::Object(
                        Object::builder()
                            .schema_type(SchemaType::AnyValue)
                            .build()
                    ))))
                    .build()
            )
            .content(
                "image/gif",
                Content::builder()
                    .schema(Some(RefOr::T(utoipa::openapi::Schema::Object(
                        Object::builder()
                            .schema_type(SchemaType::AnyValue)
                            .build()
                    ))))
                    .build()
            )
            .content(
                "image/svg",
                Content::builder()
                    .schema(Some(RefOr::T(utoipa::openapi::Schema::Object(
                        Object::builder()
                            .schema_type(SchemaType::AnyValue)
                            .build()
                    ))))
                    .build()
            )
            .build()
    )];

    "404" => [error(description("avatar or icon by hash was not found"))];
});

pub struct UpdateAvatarR;
mk_into_responses!(for UpdateAvatarR {
    "201" => [ref(UrlResponse)];
});

pub struct Params;
impl IntoParams for Params {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        vec![
            Parameter::builder()
                .name("hash")
                .required(utoipa::openapi::Required::True)
                .parameter_in(parameter_in_provider().unwrap_or_default())
                .description(Some("The hash that the request will check for"))
                .schema(Some(String::schema()))
                .build(),
        ]
    }
}

const DICEBEAR_IDENTICONS_URI: &str = "https://avatars.dicebear.com/api/identicon";
const GRAVATAR_URI: &str = "https://secure.gravatar.com/avatar";

pub trait DataStoreExt<'ds>: Sized {
    fn user_avatars(&'ds self, id: Ulid) -> UserAvatarNamespace<'ds>;
    fn repository_icons(&'ds self, id: Ulid) -> RepositoryIconNamespace<'ds>;
    fn org_icons(&'ds self, id: Ulid) -> OrganizationIconNamespace<'ds>;
}

impl<'ds> DataStoreExt<'ds> for DataStore {
    fn user_avatars(&'ds self, id: Ulid) -> UserAvatarNamespace<'ds> {
        UserAvatarNamespace {
            ns: self.namespace(format!("avatars/users/{id}")),
            id,
        }
    }

    fn repository_icons(&'ds self, id: Ulid) -> RepositoryIconNamespace<'ds> {
        RepositoryIconNamespace {
            ns: self.namespace(format!("repositories/{id}/avatars")),
            id,
        }
    }

    fn org_icons(&'ds self, id: Ulid) -> OrganizationIconNamespace<'ds> {
        OrganizationIconNamespace {
            ns: self.namespace(format!("avatars/orgs/{id}")),
            id,
        }
    }
}

macro_rules! mk_upload_fn {
    (#[instrument($($tt:tt)*)] $T:ty) => {
        #[::tracing::instrument(
            $($tt)*
        )]
        pub async fn upload<'env: 'ds>(
            &self,
            mut multipart: $crate::extract::Multipart,
        ) -> Result<String, ::charted_core::api::Response> {
            let ::core::option::Option::Some(field) = multipart.next_field().await.inspect_err(|e| {
                ::tracing::error!(error = %e, id = %self.id, "failed to get next multipart field");
                ::sentry::capture_error(e);
            }).into_system_failure()? else {
                return ::core::result::Result::Err(
                    ::charted_core::api::err(
                        ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        (
                            ::charted_core::api::ErrorCode::MissingMultipartField,
                            "didn't find a single multipart field"
                        )
                    )
                );
            };

            let data = field
                .bytes()
                .await
                .inspect_err(|e| {
                    ::tracing::error!(error = %e, id = %self.id, "unable to collect data from multipart field");
                    ::sentry::capture_error(e);
                })
                .into_system_failure()?;

            let ct = ::charted_datastore::fs::default_resolver(&data);
            let mime = ct.parse::<::mime::Mime>().inspect_err(|e| {
                ::tracing::error!(error = %e, id = %self.id, "paranoia reached: invalid content type; a bug in `remi-fs`'s default content type resolver");
                ::sentry::capture_error(e);
            }).map_err(|_| {
                ::charted_core::api::err(
                    ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        ::charted_core::api::ErrorCode::InvalidContentType,
                        "paranoia reached: invalid content type",
                    ),
                )
            })?;

            if mime.type_() != ::mime::IMAGE {
                return ::core::result::Result::Err(
                    ::charted_core::api::err(
                        ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        (
                            ::charted_core::api::ErrorCode::InvalidContentType,
                            "paranoia reached: expected a image based content type",
                            ::serde_json::json!({"mediaType":ct})
                        )
                    )
                );
            }

            let ext = match mime.subtype() {
                ::mime::PNG => "png",
                ::mime::JPEG => "jpeg",
                ::mime::GIF => "gif",
                ::mime::SVG => "svg",
                _ => return ::core::result::Result::Err(
                    ::charted_core::api::err(
                        ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        (
                            ::charted_core::api::ErrorCode::InvalidContentType,
                            "paranoia reached: expected a png, jpeg, gif, or svg image",
                            ::serde_json::json!({"mediaType":ct,"subType":mime.subtype().to_string()})
                        )
                    )
                )
            };

            let hash = ::charted_core::rand_string(5);
            let key = concat!("charts.noelware.org/", stringify!($T)).to_ascii_lowercase();
            let request = ::charted_datastore::remi::UploadRequest::default()
                .with_content_type(Some(ct.clone()))
                .with_data(data)
                .with_metadata(::azalia::hashmap! {
                    key => self.id.as_str()
                });

            let final_ = format!("{hash}.{ext}");
            self.ns.upload(&final_, request).await.into_system_failure().map(|()| final_)
        }
    };
}

pub struct UserAvatarNamespace<'ds> {
    ns: Namespace<'ds>,
    id: Ulid,
}

impl<'ds> UserAvatarNamespace<'ds> {
    #[instrument(name = "charted.servers.avatar.getUserAvatar", skip_all, fields(user.id = %self.id))]
    pub async fn get<'env: 'ds>(&self, env: &'env Env, user: &'env User) -> Result<ReturnType, api::Response> {
        if let Some(email) = &user.gravatar_email {
            if let Some(ref hash) = user.avatar_hash &&
                !user.prefers_gravatar
            {
                return self.by_hash(hash).await;
            }

            return gravatar(env, email).await;
        } else if let Some(hash) = &user.avatar_hash {
            return self.by_hash(hash).await;
        }

        dicebear(env, user.id).await
    }

    #[instrument(name = "charted.server.avatars.getUserAvatarByHash", skip_all, fields(hash = hash.as_ref(), user.id = %self.id))]
    pub async fn by_hash(&self, hash: impl AsRef<str>) -> Result<ReturnType, api::Response> {
        let hash = hash.as_ref();
        match self.ns.blob(hash).await.into_system_failure()? {
            Some(Blob::File(File { data, .. })) => {
                let ct = fs::default_resolver(&data);
                let mime = ct.parse::<mime::Mime>().into_system_failure()?;

                if mime.type_() != mime::IMAGE {
                    return Err(api::err(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        (
                            api::ErrorCode::InvalidContentType,
                            "media type for image was not an image",
                            json!({"mediaType": mime.to_string()}),
                        ),
                    ));
                }

                Ok((
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_str(mime.type_().as_str()).into_system_failure()?,
                    )],
                    data,
                ))
            }

            Some(Blob::Directory(_)) => unreachable!(),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "avatar hash was not found",
                    json!({"hash": hash}),
                ),
            )),
        }
    }

    mk_upload_fn! {
        #[instrument(name = "charted.server.avatars.uploadUserAvatar", skip_all)]
        User
    }
}

pub struct RepositoryIconNamespace<'ds> {
    ns: Namespace<'ds>,
    id: Ulid,
}

impl<'ds> RepositoryIconNamespace<'ds> {
    #[instrument(name = "charted.servers.avatar.getRepositoryIcon", skip_all, fields(repository.id = %self.id))]
    pub async fn get<'env: 'ds>(
        &self,
        env: &'env Env,
        repo: &'env Repository,
    ) -> Result<ReturnType, api::Response> {
        if let Some(ref hash) = repo.icon_hash {
            return self.by_hash(hash).await;
        }

        dicebear(env, repo.id).await
    }

    #[instrument(name = "charted.server.avatars.getRepositoryIconByHash", skip_all, fields(hash = hash.as_ref(), repository.id = %self.id))]
    pub async fn by_hash(&self, hash: impl AsRef<str>) -> Result<ReturnType, api::Response> {
        let hash = hash.as_ref();
        match self.ns.blob(hash).await.into_system_failure()? {
            Some(Blob::File(File { data, .. })) => {
                let ct = fs::default_resolver(&data);
                let mime = ct.parse::<mime::Mime>().into_system_failure()?;

                if mime.type_() != mime::IMAGE {
                    return Err(api::err(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        (
                            api::ErrorCode::InvalidContentType,
                            "media type for image was not an image",
                            json!({"mediaType": mime.to_string()}),
                        ),
                    ));
                }

                Ok((
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_str(mime.type_().as_str()).into_system_failure()?,
                    )],
                    data,
                ))
            }

            Some(Blob::Directory(_)) => unreachable!(),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "repository icon with hash was not found",
                    json!({"hash": hash}),
                ),
            )),
        }
    }

    mk_upload_fn! {
        #[instrument(name = "charted.server.avatars.uploadRepositoryIcon", skip_all)]
        Repository
    }
}

pub struct OrganizationIconNamespace<'ds> {
    ns: Namespace<'ds>,
    id: Ulid,
}

impl<'ds> OrganizationIconNamespace<'ds> {
    #[instrument(name = "charted.servers.avatar.getOrganizationIcon", skip_all, fields(organization.id = %self.id))]
    pub async fn get<'env: 'ds>(
        &self,
        env: &'env Env,
        org: &'env Organization,
    ) -> Result<ReturnType, api::Response> {
        if let Some(email) = &org.gravatar_email {
            if let Some(ref hash) = org.icon_hash &&
                !org.prefers_gravatar
            {
                return self.by_hash(hash).await;
            }

            return gravatar(env, email).await;
        } else if let Some(hash) = &org.icon_hash {
            return self.by_hash(hash).await;
        }

        dicebear(env, org.id).await
    }

    #[instrument(
        name = "charted.server.avatars.getOrganizationIconByHash",
        skip_all,
        fields(
            hash = hash.as_ref(),
            organization.id = %self.id
        )
    )]
    pub async fn by_hash(&self, hash: impl AsRef<str>) -> Result<ReturnType, api::Response> {
        let hash = hash.as_ref();
        match self.ns.blob(hash).await.into_system_failure()? {
            Some(Blob::File(File { data, .. })) => {
                let ct = fs::default_resolver(&data);
                let mime = ct.parse::<mime::Mime>().into_system_failure()?;

                if mime.type_() != mime::IMAGE {
                    return Err(api::err(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        (
                            api::ErrorCode::InvalidContentType,
                            "media type for image was not an image",
                            json!({"mediaType": mime.to_string()}),
                        ),
                    ));
                }

                Ok((
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_str(mime.type_().as_str()).into_system_failure()?,
                    )],
                    data,
                ))
            }

            Some(Blob::Directory(_)) => unreachable!(),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "avatar hash was not found",
                    json!({"hash": hash}),
                ),
            )),
        }
    }

    mk_upload_fn! {
        #[instrument(name = "charted.server.avatars.uploadOrgIcon", skip_all)]
        Organization
    }
}

async fn dicebear(env: &Env, id: Ulid) -> Result<ReturnType, api::Response> {
    let url = format!("{DICEBEAR_IDENTICONS_URI}/{id}.png");
    debug!(%url, "requesting avatar from gravatar");

    let res = env.http.get(url).send().await.map_err(api::system_failure)?;
    if res.status() == StatusCode::NOT_FOUND {
        return Err(api::empty(false, StatusCode::NOT_FOUND));
    }

    let ct = res.headers().get(header::CONTENT_TYPE).unwrap();

    Ok((
        [(header::CONTENT_TYPE, ct.clone())],
        res.bytes().await.map_err(api::system_failure)?,
    ))
}

async fn gravatar(env: &Env, email: &str) -> Result<ReturnType, api::Response> {
    let hash = {
        use md5::compute;

        let computed = compute(email.as_bytes());
        hex::encode(*computed)
    };

    let url = format!("{GRAVATAR_URI}/{hash}.png");
    debug!(%url, "requesting avatar from gravatar");

    let res = env.http.get(url).send().await.map_err(api::system_failure)?;
    if res.status() == StatusCode::NOT_FOUND {
        return Err(api::empty(false, StatusCode::NOT_FOUND));
    }

    let ct = res.headers().get(header::CONTENT_TYPE).unwrap();

    Ok((
        [(header::CONTENT_TYPE, ct.clone())],
        res.bytes().await.map_err(api::system_failure)?,
    ))
}
