pub mod v1 {
    pub mod heartbeat {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        pub struct __path_heartbeat;
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for __path_heartbeat {
            #[inline]
            fn clone(&self) -> __path_heartbeat {
                __path_heartbeat
            }
        }
        impl<'t> utoipa::__dev::Tags<'t> for __path_heartbeat {
            fn tags() -> Vec<&'t str> {
                ["Main"].into()
            }
        }
        impl utoipa::Path for __path_heartbeat {
            fn path() -> String {
                String::from("/v1/heartbeat")
            }
            fn methods() -> Vec<utoipa::openapi::path::HttpMethod> {
                [utoipa::openapi::HttpMethod::Get].into()
            }
            fn operation() -> utoipa::openapi::path::Operation {
                use utoipa::openapi::ToArray;
                use std::iter::FromIterator;
                utoipa::openapi::path::OperationBuilder::new()
                    .responses(
                        utoipa::openapi::ResponsesBuilder::new()
                            .response(
                                "200",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description("Successful response")
                                    .content(
                                        "text/plain",
                                        utoipa::openapi::content::ContentBuilder::new().into(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .operation_id(Some("heartbeat"))
                    .summary(
                        Some("Healthcheck endpoint to determine if services are OK."),
                    )
                    .into()
            }
        }
        impl utoipa::__dev::SchemaReferences for __path_heartbeat {
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {}
        }
        /// Healthcheck endpoint to determine if services are OK.
        pub async fn heartbeat() -> &'static str {
            "Ok."
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_heartbeat_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_heartbeat_into_response_make_value() -> &'static str {
                { "Ok." }
            }
            let value = __axum_macros_check_heartbeat_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_heartbeat_future() {
            /// Healthcheck endpoint to determine if services are OK.
            pub async fn heartbeat() -> &'static str {
                "Ok."
            }
            let future = heartbeat();
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
    }
    pub mod index {
        use crate::{
            extract::Path, openapi::ApiErrorResponse, ops, responses::Yaml, NameOrUlid,
            ServerContext,
        };
        use axum::{extract::State, http::StatusCode};
        use charted_core::api;
        use charted_types::helm;
        use serde_json::json;
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        pub struct __path_get_chart_index;
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for __path_get_chart_index {
            #[inline]
            fn clone(&self) -> __path_get_chart_index {
                __path_get_chart_index
            }
        }
        impl<'t> utoipa::__dev::Tags<'t> for __path_get_chart_index {
            fn tags() -> Vec<&'t str> {
                ["Main"].into()
            }
        }
        impl utoipa::Path for __path_get_chart_index {
            fn path() -> String {
                String::from("/v1/indexes/{idOrName}")
            }
            fn methods() -> Vec<utoipa::openapi::path::HttpMethod> {
                [utoipa::openapi::HttpMethod::Get].into()
            }
            fn operation() -> utoipa::openapi::path::Operation {
                use utoipa::openapi::ToArray;
                use std::iter::FromIterator;
                utoipa::openapi::path::OperationBuilder::new()
                    .responses(
                        utoipa::openapi::ResponsesBuilder::new()
                            .response(
                                "200",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description(
                                        "Chart index for a specific [`User`] or [`Organization`]",
                                    )
                                    .content(
                                        "application/yaml",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <helm::ChartIndex as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .response(
                                "404",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description("Entity was not found")
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .response(
                                "500",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description("Internal Server Error")
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .operation_id(Some("getChartIndex"))
                    .summary(Some("Retrieve a chart index for a User or Organization."))
                    .parameter(
                        utoipa::openapi::path::ParameterBuilder::from(
                                utoipa::openapi::path::Parameter::new("idOrName"),
                            )
                            .parameter_in(utoipa::openapi::path::ParameterIn::Path)
                            .description(
                                Some("Parameter that can take a `Name` or `Ulid`"),
                            )
                            .example(Some(::serde_json::to_value(&"noel").unwrap()))
                            .example(
                                Some(
                                    ::serde_json::to_value(&"01J647WVTPF2W5W99H5MBT0YQE")
                                        .unwrap(),
                                ),
                            )
                            .schema(
                                Some(
                                    utoipa::openapi::schema::RefBuilder::new()
                                        .ref_location_from_schema_name(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!(
                                                        "{0}",
                                                        <NameOrUlid as utoipa::ToSchema>::name(),
                                                    ),
                                                );
                                                res
                                            }),
                                        ),
                                ),
                            )
                            .required(utoipa::openapi::Required::True),
                    )
                    .into()
            }
        }
        impl utoipa::__dev::SchemaReferences for __path_get_chart_index {
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <helm::ChartIndex as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <helm::ChartIndex as utoipa::PartialSchema>::schema(),
                    ));
                <helm::ChartIndex as utoipa::ToSchema>::schemas(schemas);
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <ApiErrorResponse as utoipa::PartialSchema>::schema(),
                    ));
                <ApiErrorResponse as utoipa::ToSchema>::schemas(schemas);
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <ApiErrorResponse as utoipa::PartialSchema>::schema(),
                    ));
                <ApiErrorResponse as utoipa::ToSchema>::schemas(schemas);
            }
        }
        /// Retrieve a chart index for a User or Organization.
        pub async fn get_chart_index(
            State(ctx): State<ServerContext>,
            Path(id_or_name): Path<NameOrUlid>,
        ) -> Result<Yaml<helm::ChartIndex>, api::Response> {
            match ops::db::user::get(&ctx, id_or_name.clone()).await {
                Ok(Some(user)) => {
                    let Some(result) = ops::charts::get_index(&ctx, user.id)
                        .await
                        .map_err(|_| api::internal_server_error())? else {
                        return Err(
                            api::err(
                                StatusCode::NOT_FOUND,
                                (
                                    api::ErrorCode::EntityNotFound,
                                    "index for user doesn't exist, this is definitely a bug",
                                    ::serde_json::Value::Object({
                                        let mut object = ::serde_json::Map::new();
                                        let _ = object
                                            .insert(
                                                ("class").into(),
                                                ::serde_json::to_value(&"User").unwrap(),
                                            );
                                        let _ = object
                                            .insert(
                                                ("id_or_name").into(),
                                                ::serde_json::to_value(&id_or_name).unwrap(),
                                            );
                                        object
                                    }),
                                ),
                            ),
                        );
                    };
                    Ok((StatusCode::OK, result).into())
                }
                Ok(None) => {
                    match ops::db::organization::get(&ctx, id_or_name.clone()).await {
                        Ok(Some(org)) => {
                            let Some(result) = ops::charts::get_index(&ctx, org.id)
                                .await
                                .map_err(|_| api::internal_server_error())? else {
                                return Err(
                                    api::err(
                                        StatusCode::NOT_FOUND,
                                        (
                                            api::ErrorCode::EntityNotFound,
                                            "index for organization doesn't exist, this is definitely a bug",
                                            ::serde_json::Value::Object({
                                                let mut object = ::serde_json::Map::new();
                                                let _ = object
                                                    .insert(
                                                        ("class").into(),
                                                        ::serde_json::to_value(&"Organization").unwrap(),
                                                    );
                                                let _ = object
                                                    .insert(
                                                        ("id_or_name").into(),
                                                        ::serde_json::to_value(&id_or_name).unwrap(),
                                                    );
                                                object
                                            }),
                                        ),
                                    ),
                                );
                            };
                            Ok((StatusCode::OK, result).into())
                        }
                        Ok(None) => {
                            Err(
                                api::err(
                                    StatusCode::NOT_FOUND,
                                    (
                                        api::ErrorCode::EntityNotFound,
                                        "unable to find user or organization",
                                        ::serde_json::Value::Object({
                                            let mut object = ::serde_json::Map::new();
                                            let _ = object
                                                .insert(
                                                    ("id_or_name").into(),
                                                    ::serde_json::to_value(&id_or_name).unwrap(),
                                                );
                                            object
                                        }),
                                    ),
                                ),
                            )
                        }
                        Err(_) => Err(api::internal_server_error()),
                    }
                }
                Err(_) => Err(api::internal_server_error()),
            }
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_get_chart_index_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_get_chart_index_into_response_make_value() -> Result<
                Yaml<helm::ChartIndex>,
                api::Response,
            > {
                let State(ctx): State<ServerContext> = {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                };
                let Path(id_or_name): Path<NameOrUlid> = {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                };
                {
                    match ops::db::user::get(&ctx, id_or_name.clone()).await {
                        Ok(Some(user)) => {
                            let Some(result) = ops::charts::get_index(&ctx, user.id)
                                .await
                                .map_err(|_| api::internal_server_error())? else {
                                return Err(
                                    api::err(
                                        StatusCode::NOT_FOUND,
                                        (
                                            api::ErrorCode::EntityNotFound,
                                            "index for user doesn't exist, this is definitely a bug",
                                            ::serde_json::Value::Object({
                                                let mut object = ::serde_json::Map::new();
                                                let _ = object
                                                    .insert(
                                                        ("class").into(),
                                                        ::serde_json::to_value(&"User").unwrap(),
                                                    );
                                                let _ = object
                                                    .insert(
                                                        ("id_or_name").into(),
                                                        ::serde_json::to_value(&id_or_name).unwrap(),
                                                    );
                                                object
                                            }),
                                        ),
                                    ),
                                );
                            };
                            Ok((StatusCode::OK, result).into())
                        }
                        Ok(None) => {
                            match ops::db::organization::get(&ctx, id_or_name.clone())
                                .await
                            {
                                Ok(Some(org)) => {
                                    let Some(result) = ops::charts::get_index(&ctx, org.id)
                                        .await
                                        .map_err(|_| api::internal_server_error())? else {
                                        return Err(
                                            api::err(
                                                StatusCode::NOT_FOUND,
                                                (
                                                    api::ErrorCode::EntityNotFound,
                                                    "index for organization doesn't exist, this is definitely a bug",
                                                    ::serde_json::Value::Object({
                                                        let mut object = ::serde_json::Map::new();
                                                        let _ = object
                                                            .insert(
                                                                ("class").into(),
                                                                ::serde_json::to_value(&"Organization").unwrap(),
                                                            );
                                                        let _ = object
                                                            .insert(
                                                                ("id_or_name").into(),
                                                                ::serde_json::to_value(&id_or_name).unwrap(),
                                                            );
                                                        object
                                                    }),
                                                ),
                                            ),
                                        );
                                    };
                                    Ok((StatusCode::OK, result).into())
                                }
                                Ok(None) => {
                                    Err(
                                        api::err(
                                            StatusCode::NOT_FOUND,
                                            (
                                                api::ErrorCode::EntityNotFound,
                                                "unable to find user or organization",
                                                ::serde_json::Value::Object({
                                                    let mut object = ::serde_json::Map::new();
                                                    let _ = object
                                                        .insert(
                                                            ("id_or_name").into(),
                                                            ::serde_json::to_value(&id_or_name).unwrap(),
                                                        );
                                                    object
                                                }),
                                            ),
                                        ),
                                    )
                                }
                                Err(_) => Err(api::internal_server_error()),
                            }
                        }
                        Err(_) => Err(api::internal_server_error()),
                    }
                }
            }
            let value = __axum_macros_check_get_chart_index_into_response_make_value()
                .await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[doc(hidden)]
        fn __axum_macros_check_get_chart_index_0_from_request_check()
        where
            State<
                ServerContext,
            >: ::axum::extract::FromRequestParts<ServerContext> + Send,
        {}
        #[allow(warnings)]
        #[doc(hidden)]
        fn __axum_macros_check_get_chart_index_0_from_request_call_check() {
            __axum_macros_check_get_chart_index_0_from_request_check();
        }
        #[allow(warnings)]
        #[doc(hidden)]
        fn __axum_macros_check_get_chart_index_1_from_request_check<M>()
        where
            Path<NameOrUlid>: ::axum::extract::FromRequest<ServerContext, M> + Send,
        {}
        #[allow(warnings)]
        #[doc(hidden)]
        fn __axum_macros_check_get_chart_index_1_from_request_call_check() {
            __axum_macros_check_get_chart_index_1_from_request_check();
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_get_chart_index_future() {
            /// Retrieve a chart index for a User or Organization.
            pub async fn get_chart_index(
                State(ctx): State<ServerContext>,
                Path(id_or_name): Path<NameOrUlid>,
            ) -> Result<Yaml<helm::ChartIndex>, api::Response> {
                match ops::db::user::get(&ctx, id_or_name.clone()).await {
                    Ok(Some(user)) => {
                        let Some(result) = ops::charts::get_index(&ctx, user.id)
                            .await
                            .map_err(|_| api::internal_server_error())? else {
                            return Err(
                                api::err(
                                    StatusCode::NOT_FOUND,
                                    (
                                        api::ErrorCode::EntityNotFound,
                                        "index for user doesn't exist, this is definitely a bug",
                                        ::serde_json::Value::Object({
                                            let mut object = ::serde_json::Map::new();
                                            let _ = object
                                                .insert(
                                                    ("class").into(),
                                                    ::serde_json::to_value(&"User").unwrap(),
                                                );
                                            let _ = object
                                                .insert(
                                                    ("id_or_name").into(),
                                                    ::serde_json::to_value(&id_or_name).unwrap(),
                                                );
                                            object
                                        }),
                                    ),
                                ),
                            );
                        };
                        Ok((StatusCode::OK, result).into())
                    }
                    Ok(None) => {
                        match ops::db::organization::get(&ctx, id_or_name.clone()).await
                        {
                            Ok(Some(org)) => {
                                let Some(result) = ops::charts::get_index(&ctx, org.id)
                                    .await
                                    .map_err(|_| api::internal_server_error())? else {
                                    return Err(
                                        api::err(
                                            StatusCode::NOT_FOUND,
                                            (
                                                api::ErrorCode::EntityNotFound,
                                                "index for organization doesn't exist, this is definitely a bug",
                                                ::serde_json::Value::Object({
                                                    let mut object = ::serde_json::Map::new();
                                                    let _ = object
                                                        .insert(
                                                            ("class").into(),
                                                            ::serde_json::to_value(&"Organization").unwrap(),
                                                        );
                                                    let _ = object
                                                        .insert(
                                                            ("id_or_name").into(),
                                                            ::serde_json::to_value(&id_or_name).unwrap(),
                                                        );
                                                    object
                                                }),
                                            ),
                                        ),
                                    );
                                };
                                Ok((StatusCode::OK, result).into())
                            }
                            Ok(None) => {
                                Err(
                                    api::err(
                                        StatusCode::NOT_FOUND,
                                        (
                                            api::ErrorCode::EntityNotFound,
                                            "unable to find user or organization",
                                            ::serde_json::Value::Object({
                                                let mut object = ::serde_json::Map::new();
                                                let _ = object
                                                    .insert(
                                                        ("id_or_name").into(),
                                                        ::serde_json::to_value(&id_or_name).unwrap(),
                                                    );
                                                object
                                            }),
                                        ),
                                    ),
                                )
                            }
                            Err(_) => Err(api::internal_server_error()),
                        }
                    }
                    Err(_) => Err(api::internal_server_error()),
                }
            }
            let future = get_chart_index(
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                },
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                },
            );
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
    }
    pub mod info {
        use axum::http::StatusCode;
        use charted_core::{api, Distribution, BUILD_DATE, COMMIT_HASH, VERSION};
        use charted_proc_macros::generate_api_response;
        use serde::Serialize;
        use utoipa::ToSchema;
        /// Represents the response for the `GET /info` REST handler.
        pub struct InfoResponse {
            /// The distribution the server is running off from
            pub distribution: Distribution,
            /// The commit hash from the Git repository.
            pub commit_sha: String,
            /// Build date in RFC3339 format
            pub build_date: String,
            /// Product name. Will always be "charted-server"
            pub product: String,
            /// Valid SemVer 2 of the current version of this instance
            pub version: String,
            /// Vendor of charted-server, will always be "Noelware, LLC."
            pub vendor: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for InfoResponse {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "InfoResponse",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "distribution",
                        &self.distribution,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "commit_sha",
                        &self.commit_sha,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "build_date",
                        &self.build_date,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "product",
                        &self.product,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "version",
                        &self.version,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "vendor",
                        &self.vendor,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        impl utoipa::__dev::ComposeSchema for InfoResponse {
            fn compose(
                mut generics: Vec<
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                >,
            ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .property(
                        "distribution",
                        utoipa::openapi::schema::RefBuilder::new()
                            .description(
                                Some("The distribution the server is running off from"),
                            )
                            .ref_location_from_schema_name(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "{0}",
                                            <Distribution as utoipa::ToSchema>::name(),
                                        ),
                                    );
                                    res
                                }),
                            ),
                    )
                    .required("distribution")
                    .property(
                        "commit_sha",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(
                                Some("The commit hash from the Git repository."),
                            ),
                    )
                    .required("commit_sha")
                    .property(
                        "build_date",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(Some("Build date in RFC3339 format")),
                    )
                    .required("build_date")
                    .property(
                        "product",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(
                                Some("Product name. Will always be \"charted-server\""),
                            ),
                    )
                    .required("product")
                    .property(
                        "version",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(
                                Some(
                                    "Valid SemVer 2 of the current version of this instance",
                                ),
                            ),
                    )
                    .required("version")
                    .property(
                        "vendor",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(
                                Some(
                                    "Vendor of charted-server, will always be \"Noelware, LLC.\"",
                                ),
                            ),
                    )
                    .required("vendor")
                    .description(
                        Some("Represents the response for the `GET /info` REST handler."),
                    )
                    .into()
            }
        }
        impl utoipa::ToSchema for InfoResponse {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("InfoResponse")
            }
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {
                schemas
                    .extend([
                        (
                            String::from(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "{0}",
                                            <Distribution as utoipa::ToSchema>::name(),
                                        ),
                                    );
                                    res
                                }),
                            ),
                            <Distribution as utoipa::PartialSchema>::schema(),
                        ),
                    ]);
                <Distribution as utoipa::ToSchema>::schemas(schemas);
            }
        }
        impl Default for InfoResponse {
            fn default() -> InfoResponse {
                InfoResponse {
                    distribution: Distribution::detect(),
                    commit_sha: COMMIT_HASH.to_string(),
                    build_date: BUILD_DATE.to_string(),
                    product: "charted-server".into(),
                    version: VERSION.to_string(),
                    vendor: "Noelware, LLC.".into(),
                }
            }
        }
        #[automatically_derived]
        impl<'r> ::utoipa::ToResponse<'r> for InfoResponse {
            fn response() -> (
                &'r str,
                ::utoipa::openapi::RefOr<::utoipa::openapi::Response>,
            ) {
                let __datatype_schema = <InfoResponse as ::utoipa::ToSchema>::name();
                let __schema = ::utoipa::openapi::Schema::Object({
                    let __object = ::utoipa::openapi::ObjectBuilder::new()
                        .property(
                            "success",
                            ::utoipa::openapi::ObjectBuilder::new()
                                .description(Some("whether if this request was successful"))
                                .schema_type(
                                    ::utoipa::openapi::schema::SchemaType::Type(
                                        ::utoipa::openapi::schema::Type::Boolean,
                                    ),
                                )
                                .build(),
                        )
                        .required("success")
                        .property(
                            "data",
                            ::utoipa::openapi::RefOr::Ref(
                                ::utoipa::openapi::Ref::from_schema_name(__datatype_schema),
                            ),
                        )
                        .required("data")
                        .build();
                    __object
                });
                let __response = ::utoipa::openapi::ResponseBuilder::new()
                    .content(
                        "application/json",
                        ::utoipa::openapi::ContentBuilder::new()
                            .schema(::core::option::Option::Some(__schema))
                            .build(),
                    )
                    .build();
                let __name = "InfoResponse";
                (__name, ::utoipa::openapi::RefOr::T(__response))
            }
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        pub struct __path_info;
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for __path_info {
            #[inline]
            fn clone(&self) -> __path_info {
                __path_info
            }
        }
        impl<'t> utoipa::__dev::Tags<'t> for __path_info {
            fn tags() -> Vec<&'t str> {
                ["Main"].into()
            }
        }
        impl utoipa::Path for __path_info {
            fn path() -> String {
                String::from("/v1/info")
            }
            fn methods() -> Vec<utoipa::openapi::path::HttpMethod> {
                [utoipa::openapi::HttpMethod::Get].into()
            }
            fn operation() -> utoipa::openapi::path::Operation {
                use utoipa::openapi::ToArray;
                use std::iter::FromIterator;
                utoipa::openapi::path::OperationBuilder::new()
                    .responses(
                        utoipa::openapi::ResponsesBuilder::new()
                            .response(
                                "200",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description("Successful response")
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <InfoResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .operation_id(Some("info"))
                    .summary(Some("Shows information about this running instance."))
                    .into()
            }
        }
        impl utoipa::__dev::SchemaReferences for __path_info {
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <InfoResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <InfoResponse as utoipa::PartialSchema>::schema(),
                    ));
                <InfoResponse as utoipa::ToSchema>::schemas(schemas);
            }
        }
        /// Shows information about this running instance.
        pub async fn info() -> api::Response<InfoResponse> {
            api::from_default(StatusCode::OK)
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_info_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_info_into_response_make_value() -> api::Response<
                InfoResponse,
            > {
                { api::from_default(StatusCode::OK) }
            }
            let value = __axum_macros_check_info_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_info_future() {
            /// Shows information about this running instance.
            pub async fn info() -> api::Response<InfoResponse> {
                api::from_default(StatusCode::OK)
            }
            let future = info();
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
    }
    pub mod main {
        use axum::http::StatusCode;
        use charted_core::{api, VERSION};
        use charted_proc_macros::generate_api_response;
        use serde::Serialize;
        use utoipa::ToSchema;
        /// Response object for the `GET /` REST controller.
        pub struct MainResponse {
            /// The message, which will always be "Hello, world!"
            message: String,
            /// You know, for Helm charts?
            tagline: String,
            /// Documentation URL for this generic entrypoint response.
            docs: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for MainResponse {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "MainResponse",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "message",
                        &self.message,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "tagline",
                        &self.tagline,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "docs",
                        &self.docs,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        impl utoipa::__dev::ComposeSchema for MainResponse {
            fn compose(
                mut generics: Vec<
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                >,
            ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::ObjectBuilder::new()
                    .property(
                        "message",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(
                                Some("The message, which will always be \"Hello, world!\""),
                            ),
                    )
                    .required("message")
                    .property(
                        "tagline",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(Some("You know, for Helm charts?")),
                    )
                    .required("tagline")
                    .property(
                        "docs",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(
                                Some(
                                    "Documentation URL for this generic entrypoint response.",
                                ),
                            ),
                    )
                    .required("docs")
                    .description(
                        Some("Response object for the `GET /` REST controller."),
                    )
                    .into()
            }
        }
        impl utoipa::ToSchema for MainResponse {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("MainResponse")
            }
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {
                schemas.extend([]);
            }
        }
        impl Default for MainResponse {
            fn default() -> Self {
                MainResponse {
                    message: "Hello, world! 👋".into(),
                    tagline: "You know, for Helm charts?".into(),
                    docs: ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "https://charts.noelware.org/docs/server/{0}",
                                VERSION,
                            ),
                        );
                        res
                    }),
                }
            }
        }
        #[automatically_derived]
        impl<'r> ::utoipa::ToResponse<'r> for MainResponse {
            fn response() -> (
                &'r str,
                ::utoipa::openapi::RefOr<::utoipa::openapi::Response>,
            ) {
                let __datatype_schema = <MainResponse as ::utoipa::ToSchema>::name();
                let __schema = ::utoipa::openapi::Schema::Object({
                    let __object = ::utoipa::openapi::ObjectBuilder::new()
                        .property(
                            "success",
                            ::utoipa::openapi::ObjectBuilder::new()
                                .description(Some("whether if this request was successful"))
                                .schema_type(
                                    ::utoipa::openapi::schema::SchemaType::Type(
                                        ::utoipa::openapi::schema::Type::Boolean,
                                    ),
                                )
                                .build(),
                        )
                        .required("success")
                        .property(
                            "data",
                            ::utoipa::openapi::RefOr::Ref(
                                ::utoipa::openapi::Ref::from_schema_name(__datatype_schema),
                            ),
                        )
                        .required("data")
                        .build();
                    __object
                });
                let __response = ::utoipa::openapi::ResponseBuilder::new()
                    .content(
                        "application/json",
                        ::utoipa::openapi::ContentBuilder::new()
                            .schema(::core::option::Option::Some(__schema))
                            .build(),
                    )
                    .build();
                let __name = "MainResponse";
                (__name, ::utoipa::openapi::RefOr::T(__response))
            }
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        pub struct __path_main;
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for __path_main {
            #[inline]
            fn clone(&self) -> __path_main {
                __path_main
            }
        }
        impl<'t> utoipa::__dev::Tags<'t> for __path_main {
            fn tags() -> Vec<&'t str> {
                ["Main"].into()
            }
        }
        impl utoipa::Path for __path_main {
            fn path() -> String {
                String::from("/v1")
            }
            fn methods() -> Vec<utoipa::openapi::path::HttpMethod> {
                [utoipa::openapi::HttpMethod::Get].into()
            }
            fn operation() -> utoipa::openapi::path::Operation {
                use utoipa::openapi::ToArray;
                use std::iter::FromIterator;
                utoipa::openapi::path::OperationBuilder::new()
                    .responses(
                        utoipa::openapi::ResponsesBuilder::new()
                            .response(
                                "200",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description("Successful response")
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <MainResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .operation_id(Some("main"))
                    .summary(
                        Some(
                            "Main entrypoint response to the API. Nothing too important.",
                        ),
                    )
                    .into()
            }
        }
        impl utoipa::__dev::SchemaReferences for __path_main {
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <MainResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <MainResponse as utoipa::PartialSchema>::schema(),
                    ));
                <MainResponse as utoipa::ToSchema>::schemas(schemas);
            }
        }
        /// Main entrypoint response to the API. Nothing too important.
        pub async fn main() -> api::Response<MainResponse> {
            api::from_default(StatusCode::OK)
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_main_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_main_into_response_make_value() -> api::Response<
                MainResponse,
            > {
                { api::from_default(StatusCode::OK) }
            }
            let value = __axum_macros_check_main_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_main_future() {
            /// Main entrypoint response to the API. Nothing too important.
            pub async fn main() -> api::Response<MainResponse> {
                api::from_default(StatusCode::OK)
            }
            let future = main();
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
    }
    pub mod openapi {
        #![allow(clippy::incompatible_msrv)]
        use crate::openapi::Document;
        use std::sync::LazyLock;
        use utoipa::OpenApi;
        static CACHED: LazyLock<utoipa::openapi::OpenApi> = LazyLock::new(
            Document::openapi,
        );
        pub async fn openapi() -> String {
            serde_json::to_string(&*CACHED)
                .expect("it should be serialized to a JSON value")
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_openapi_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_openapi_into_response_make_value() -> String {
                {
                    serde_json::to_string(&*CACHED)
                        .expect("it should be serialized to a JSON value")
                }
            }
            let value = __axum_macros_check_openapi_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_openapi_future() {
            pub async fn openapi() -> String {
                serde_json::to_string(&*CACHED)
                    .expect("it should be serialized to a JSON value")
            }
            let future = openapi();
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
    }
    pub mod user {
        pub mod avatars {}
        pub mod repositories {}
        pub mod sessions {}
        use super::EntrypointResponse;
        use crate::{
            extract::Json, hash_password, openapi::ApiErrorResponse, ops, ServerContext,
        };
        use axum::{extract::State, http::StatusCode, routing, Router};
        use charted_core::api;
        use charted_database::{connection, schema::{postgresql, sqlite}};
        use charted_proc_macros::generate_api_response;
        use charted_types::{payloads::user::CreateUserPayload, User};
        use serde_json::json;
        use tracing::{error, instrument};
        use utoipa::ToSchema;
        use validator::ValidateEmail;
        pub struct UserResponse;
        impl utoipa::__dev::ComposeSchema for UserResponse {
            fn compose(
                mut generics: Vec<
                    utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
                >,
            ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                utoipa::openapi::Object::builder()
                    .schema_type(utoipa::openapi::schema::SchemaType::AnyValue)
                    .default(Some(serde_json::Value::Null))
                    .into()
            }
        }
        impl utoipa::ToSchema for UserResponse {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("UserResponse")
            }
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {
                schemas.extend([]);
            }
        }
        #[automatically_derived]
        impl<'r> ::utoipa::ToResponse<'r> for UserResponse {
            fn response() -> (
                &'r str,
                ::utoipa::openapi::RefOr<::utoipa::openapi::Response>,
            ) {
                let __datatype_schema = <User as ::utoipa::ToSchema>::name();
                let __schema = ::utoipa::openapi::Schema::Object({
                    let __object = ::utoipa::openapi::ObjectBuilder::new()
                        .property(
                            "success",
                            ::utoipa::openapi::ObjectBuilder::new()
                                .description(Some("whether if this request was successful"))
                                .schema_type(
                                    ::utoipa::openapi::schema::SchemaType::Type(
                                        ::utoipa::openapi::schema::Type::Boolean,
                                    ),
                                )
                                .build(),
                        )
                        .required("success")
                        .property(
                            "data",
                            ::utoipa::openapi::RefOr::Ref(
                                ::utoipa::openapi::Ref::from_schema_name(__datatype_schema),
                            ),
                        )
                        .required("data")
                        .build();
                    __object
                });
                let __response = ::utoipa::openapi::ResponseBuilder::new()
                    .content(
                        "application/json",
                        ::utoipa::openapi::ContentBuilder::new()
                            .schema(::core::option::Option::Some(__schema))
                            .build(),
                    )
                    .build();
                let __name = "UserResponse";
                (__name, ::utoipa::openapi::RefOr::T(__response))
            }
        }
        pub fn create_router() -> Router<ServerContext> {
            Router::new().route("/", routing::get(main))
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        pub struct __path_main;
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for __path_main {
            #[inline]
            fn clone(&self) -> __path_main {
                __path_main
            }
        }
        impl<'t> utoipa::__dev::Tags<'t> for __path_main {
            fn tags() -> Vec<&'t str> {
                ["Users"].into()
            }
        }
        impl utoipa::Path for __path_main {
            fn path() -> String {
                String::from("/v1/users")
            }
            fn methods() -> Vec<utoipa::openapi::path::HttpMethod> {
                [utoipa::openapi::HttpMethod::Get].into()
            }
            fn operation() -> utoipa::openapi::path::Operation {
                use utoipa::openapi::ToArray;
                use std::iter::FromIterator;
                utoipa::openapi::path::OperationBuilder::new()
                    .responses(
                        utoipa::openapi::ResponsesBuilder::new()
                            .response(
                                "200",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description("Entrypoint response")
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <EntrypointResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .operation_id(Some("users"))
                    .summary(Some("Entrypoint to the Users API."))
                    .into()
            }
        }
        impl utoipa::__dev::SchemaReferences for __path_main {
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <EntrypointResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <EntrypointResponse as utoipa::PartialSchema>::schema(),
                    ));
                <EntrypointResponse as utoipa::ToSchema>::schemas(schemas);
            }
        }
        /// Entrypoint to the Users API.
        pub async fn main() -> api::Response<EntrypointResponse> {
            api::ok(StatusCode::OK, EntrypointResponse::new("Users"))
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_main_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_main_into_response_make_value() -> api::Response<
                EntrypointResponse,
            > {
                { api::ok(StatusCode::OK, EntrypointResponse::new("Users")) }
            }
            let value = __axum_macros_check_main_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_main_future() {
            /// Entrypoint to the Users API.
            pub async fn main() -> api::Response<EntrypointResponse> {
                api::ok(StatusCode::OK, EntrypointResponse::new("Users"))
            }
            let future = main();
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        pub struct __path_create_user;
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for __path_create_user {
            #[inline]
            fn clone(&self) -> __path_create_user {
                __path_create_user
            }
        }
        impl<'t> utoipa::__dev::Tags<'t> for __path_create_user {
            fn tags() -> Vec<&'t str> {
                ["Users"].into()
            }
        }
        impl utoipa::Path for __path_create_user {
            fn path() -> String {
                String::from("/v1/users")
            }
            fn methods() -> Vec<utoipa::openapi::path::HttpMethod> {
                [utoipa::openapi::HttpMethod::Post].into()
            }
            fn operation() -> utoipa::openapi::path::Operation {
                use utoipa::openapi::ToArray;
                use std::iter::FromIterator;
                utoipa::openapi::path::OperationBuilder::new()
                    .request_body(
                        Some(
                            utoipa::openapi::request_body::RequestBodyBuilder::new()
                                .content(
                                    "application/json",
                                    utoipa::openapi::content::ContentBuilder::new()
                                        .schema(
                                            Some(utoipa::openapi::schema::Ref::new("CreateUserPayload")),
                                        )
                                        .into(),
                                )
                                .description(
                                    Some(
                                        "Payload for creating a new user. The `password` field can be omitted if the session backend is not `Local`.",
                                    ),
                                )
                                .build(),
                        ),
                    )
                    .responses(
                        utoipa::openapi::ResponsesBuilder::new()
                            .response(
                                "201",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description("User has been created")
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <UserResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .response(
                                "403",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description(
                                        "Returned if the server doesn't allow user registrations or if this is a single-user registry",
                                    )
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .response(
                                "406",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description(
                                        "Returned if the authentication backend requires a `password` field or the `email` field is not a valid email",
                                    )
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .response(
                                "409",
                                utoipa::openapi::ResponseBuilder::new()
                                    .description(
                                        "Returned if the `username` or `email` provided is already registered",
                                    )
                                    .content(
                                        "application/json",
                                        utoipa::openapi::content::ContentBuilder::new()
                                            .schema(
                                                Some(
                                                    utoipa::openapi::schema::RefBuilder::new()
                                                        .ref_location_from_schema_name(
                                                            ::alloc::__export::must_use({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "{0}",
                                                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                                                    ),
                                                                );
                                                                res
                                                            }),
                                                        ),
                                                ),
                                            )
                                            .into(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .operation_id(Some("createUser"))
                    .into()
            }
        }
        impl utoipa::__dev::SchemaReferences for __path_create_user {
            fn schemas(
                schemas: &mut Vec<
                    (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
                >,
            ) {
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <UserResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <UserResponse as utoipa::PartialSchema>::schema(),
                    ));
                <UserResponse as utoipa::ToSchema>::schemas(schemas);
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <ApiErrorResponse as utoipa::PartialSchema>::schema(),
                    ));
                <ApiErrorResponse as utoipa::ToSchema>::schemas(schemas);
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <ApiErrorResponse as utoipa::PartialSchema>::schema(),
                    ));
                <ApiErrorResponse as utoipa::ToSchema>::schemas(schemas);
                schemas
                    .push((
                        String::from(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}",
                                        <ApiErrorResponse as utoipa::ToSchema>::name(),
                                    ),
                                );
                                res
                            }),
                        ),
                        <ApiErrorResponse as utoipa::PartialSchema>::schema(),
                    ));
                <ApiErrorResponse as utoipa::ToSchema>::schemas(schemas);
            }
        }
        pub async fn create_user(
            State(cx): State<ServerContext>,
            Json(
                CreateUserPayload { email, password, username },
            ): Json<CreateUserPayload>,
        ) -> api::Result<User> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "charted.server.ops.v1.createUser",
                            "charted_server::routing::v1::user",
                            tracing::Level::INFO,
                            ::core::option::Option::Some(
                                "crates/server/src/routing/v1/user/mod.rs",
                            ),
                            ::core::option::Option::Some(100u32),
                            ::core::option::Option::Some(
                                "charted_server::routing::v1::user",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["user.name"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(
                        meta,
                        &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields()
                                .value_set(
                                    &[
                                        (
                                            &::core::iter::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::core::option::Option::Some(
                                                &display(&username) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        },
                    )
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    if match tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(
                                    &{
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &display(&username) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    },
                                );
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_instrument_future = async move {
                #[allow(
                    unknown_lints,
                    unreachable_code,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::unreachable,
                    clippy::let_with_type_underscore,
                    clippy::empty_loop
                )]
                if false {
                    let __tracing_attr_fake_return: api::Result<User> = loop {};
                    return __tracing_attr_fake_return;
                }
                {
                    if !cx.config.registrations || cx.config.single_user {
                        return Err(
                            api::err(
                                StatusCode::FORBIDDEN,
                                (
                                    api::ErrorCode::RegistrationsDisabled,
                                    "this instance has user registrations disabled",
                                ),
                            ),
                        );
                    }
                    if cx
                        .authz
                        .as_ref()
                        .downcast::<charted_authz_local::Backend>()
                        .is_some() && password.is_none()
                    {
                        return Err(
                            api::err(
                                StatusCode::NOT_ACCEPTABLE,
                                (
                                    api::ErrorCode::MissingPassword,
                                    "authentication backend requires you to include a password for this new account",
                                ),
                            ),
                        );
                    }
                    if !email.validate_email() {
                        return Err(
                            api::err(
                                StatusCode::NOT_ACCEPTABLE,
                                (
                                    api::ErrorCode::ValidationFailed,
                                    "`email` is not a valid email",
                                    ::serde_json::Value::Object({
                                        let mut object = ::serde_json::Map::new();
                                        let _ = object
                                            .insert(
                                                ("email").into(),
                                                ::serde_json::to_value(&&email).unwrap(),
                                            );
                                        object
                                    }),
                                ),
                            ),
                        );
                    }
                    {
                        let mut conn = cx
                            .pool
                            .get()
                            .inspect_err(|e| {
                                sentry::capture_error(e);
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/server/src/routing/v1/user/mod.rs:152",
                                                "charted_server::routing::v1::user",
                                                ::tracing::Level::ERROR,
                                                ::core::option::Option::Some(
                                                    "crates/server/src/routing/v1/user/mod.rs",
                                                ),
                                                ::core::option::Option::Some(152u32),
                                                ::core::option::Option::Some(
                                                    "charted_server::routing::v1::user",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message", "error"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("failed to get db connection") as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("failed to get db connection") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                            })
                            .map_err(|_| api::internal_server_error())?;
                        let uname = &username;
                        let exists = {
                            #[allow(unused)]
                            use ::diesel::prelude::*;
                            match *conn {
                                ::charted_database::DbConnection::PostgreSQL(
                                    ref mut conn,
                                ) => {
                                    conn.build_transaction()
                                        .read_only()
                                        .run::<
                                            _,
                                            eyre::Report,
                                            _,
                                        >(|txn| {
                                            use postgresql::users::{dsl::*, table};
                                            use diesel::pg::Pg;
                                            match table
                                                .select(<User as SelectableHelper<Pg>>::as_select())
                                                .filter(username.eq(uname))
                                                .first(txn)
                                            {
                                                Ok(_) => Ok(true),
                                                Err(diesel::result::Error::NotFound) => Ok(false),
                                                Err(e) => Err(eyre::Report::from(e)),
                                            }
                                        })
                                }
                                ::charted_database::DbConnection::SQLite(ref mut conn) => {
                                    conn.immediate_transaction(|txn| {
                                        use sqlite::users::{dsl::*, table};
                                        use diesel::sqlite::Sqlite;
                                        match table
                                            .select(<User as SelectableHelper<Sqlite>>::as_select())
                                            .filter(username.eq(uname))
                                            .first(txn)
                                        {
                                            Ok(_) => Ok(true),
                                            Err(diesel::result::Error::NotFound) => Ok(false),
                                            Err(e) => Err(eyre::Report::from(e)),
                                        }
                                    })
                                }
                            }
                        }
                            .inspect_err(|e| {
                                sentry_eyre::capture_report(e);
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/server/src/routing/v1/user/mod.rs:181",
                                                "charted_server::routing::v1::user",
                                                ::tracing::Level::ERROR,
                                                ::core::option::Option::Some(
                                                    "crates/server/src/routing/v1/user/mod.rs",
                                                ),
                                                ::core::option::Option::Some(181u32),
                                                ::core::option::Option::Some(
                                                    "charted_server::routing::v1::user",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message", "user.name", "error"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("failed to query user by username")
                                                                    as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &display(&username) as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("failed to query user by username")
                                                                                            as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &display(&username) as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                            })
                            .map_err(|_| api::internal_server_error())?;
                        if exists {
                            return Err(
                                api::err(
                                    StatusCode::CONFLICT,
                                    (
                                        api::ErrorCode::EntityAlreadyExists,
                                        "a user with `username` already exists",
                                        ::serde_json::Value::Object({
                                            let mut object = ::serde_json::Map::new();
                                            let _ = object
                                                .insert(
                                                    ("username").into(),
                                                    ::serde_json::to_value(&uname.as_str()).unwrap(),
                                                );
                                            object
                                        }),
                                    ),
                                ),
                            );
                        }
                    }
                    {
                        let mut conn = cx
                            .pool
                            .get()
                            .inspect_err(|e| {
                                sentry::capture_error(e);
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/server/src/routing/v1/user/mod.rs:203",
                                                "charted_server::routing::v1::user",
                                                ::tracing::Level::ERROR,
                                                ::core::option::Option::Some(
                                                    "crates/server/src/routing/v1/user/mod.rs",
                                                ),
                                                ::core::option::Option::Some(203u32),
                                                ::core::option::Option::Some(
                                                    "charted_server::routing::v1::user",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message", "error"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("failed to get db connection") as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("failed to get db connection") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                            })
                            .map_err(|_| api::internal_server_error())?;
                        let em = &email;
                        let exists = {
                            #[allow(unused)]
                            use ::diesel::prelude::*;
                            match *conn {
                                ::charted_database::DbConnection::PostgreSQL(
                                    ref mut conn,
                                ) => {
                                    conn.build_transaction()
                                        .read_only()
                                        .run::<
                                            _,
                                            eyre::Report,
                                            _,
                                        >(|txn| {
                                            use postgresql::users::{dsl::*, table};
                                            use diesel::pg::Pg;
                                            match table
                                                .select(<User as SelectableHelper<Pg>>::as_select())
                                                .filter(email.eq(em))
                                                .first(txn)
                                            {
                                                Ok(_) => Ok(true),
                                                Err(diesel::result::Error::NotFound) => Ok(false),
                                                Err(e) => Err(eyre::Report::from(e)),
                                            }
                                        })
                                }
                                ::charted_database::DbConnection::SQLite(ref mut conn) => {
                                    conn.immediate_transaction(|txn| {
                                        use sqlite::users::{dsl::*, table};
                                        use diesel::sqlite::Sqlite;
                                        match table
                                            .select(<User as SelectableHelper<Sqlite>>::as_select())
                                            .filter(email.eq(em))
                                            .first(txn)
                                        {
                                            Ok(_) => Ok(true),
                                            Err(diesel::result::Error::NotFound) => Ok(false),
                                            Err(e) => Err(eyre::Report::from(e)),
                                        }
                                    })
                                }
                            }
                        }
                            .inspect_err(|e| {
                                sentry_eyre::capture_report(e);
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/server/src/routing/v1/user/mod.rs:233",
                                                "charted_server::routing::v1::user",
                                                ::tracing::Level::ERROR,
                                                ::core::option::Option::Some(
                                                    "crates/server/src/routing/v1/user/mod.rs",
                                                ),
                                                ::core::option::Option::Some(233u32),
                                                ::core::option::Option::Some(
                                                    "charted_server::routing::v1::user",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message", "user.email", "error"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!("failed to query user by email") as &dyn Value,
                                                            ),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&em as &dyn Value),
                                                        ),
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("failed to query user by email") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&em as &dyn Value),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                            })
                            .map_err(|_| api::internal_server_error())?;
                        if exists {
                            return Err(
                                api::err(
                                    StatusCode::CONFLICT,
                                    (
                                        api::ErrorCode::EntityAlreadyExists,
                                        "a user with the `email` given already exists",
                                        ::serde_json::Value::Object({
                                            let mut object = ::serde_json::Map::new();
                                            let _ = object
                                                .insert(
                                                    ("email").into(),
                                                    ::serde_json::to_value(&em).unwrap(),
                                                );
                                            object
                                        }),
                                    ),
                                ),
                            );
                        }
                    }
                    let password = if let Some(ref password) = password {
                        if password.len() < 8 {
                            return Err(
                                api::err(
                                    StatusCode::NOT_ACCEPTABLE,
                                    (
                                        api::ErrorCode::InvalidPassword,
                                        "`password` length was expected to be 8 characters or longer",
                                    ),
                                ),
                            );
                        }
                        Some(
                            hash_password(password)
                                .map_err(|_| api::internal_server_error())?,
                        )
                    } else {
                        None
                    };
                    let id = cx
                        .ulid_generator
                        .generate()
                        .inspect_err(|e| {
                            sentry::capture_error(e);
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/server/src/routing/v1/user/mod.rs:270",
                                            "charted_server::routing::v1::user",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some(
                                                "crates/server/src/routing/v1/user/mod.rs",
                                            ),
                                            ::core::option::Option::Some(270u32),
                                            ::core::option::Option::Some(
                                                "charted_server::routing::v1::user",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                        if match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &value_set,
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "received monotonic overflow -- please inspect this as fast you can!!!!!",
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                    if match ::tracing::Level::ERROR {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &{
                                                                #[allow(unused_imports)]
                                                                use ::tracing::field::{debug, display, Value};
                                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                                __CALLSITE
                                                                    .metadata()
                                                                    .fields()
                                                                    .value_set(
                                                                        &[
                                                                            (
                                                                                &::core::iter::Iterator::next(&mut iter)
                                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                                ::core::option::Option::Some(
                                                                                    &format_args!(
                                                                                        "received monotonic overflow -- please inspect this as fast you can!!!!!",
                                                                                    ) as &dyn Value,
                                                                                ),
                                                                            ),
                                                                        ],
                                                                    )
                                                            },
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                }
                            };
                        })
                        .map_err(|_| api::internal_server_error())?;
                    let user = User {
                        verified_publisher: false,
                        gravatar_email: None,
                        description: None,
                        avatar_hash: None,
                        created_at: chrono::Utc::now().into(),
                        updated_at: chrono::Utc::now().into(),
                        password,
                        username,
                        email,
                        admin: false,
                        name: None,
                        id: id.into(),
                    };
                    ops::charts::create_index(&cx, &user)
                        .await
                        .map_err(|_| api::internal_server_error())?;
                    Ok(api::ok(StatusCode::CREATED, user))
                }
            };
            if !__tracing_attr_span.is_disabled() {
                tracing::Instrument::instrument(
                        __tracing_instrument_future,
                        __tracing_attr_span,
                    )
                    .await
            } else {
                __tracing_instrument_future.await
            }
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        async fn __axum_macros_check_create_user_into_response() {
            #[allow(warnings)]
            #[allow(unreachable_code)]
            #[doc(hidden)]
            async fn __axum_macros_check_create_user_into_response_make_value() -> api::Result<
                User,
            > {
                let State(cx): State<ServerContext> = {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                };
                let Json(
                    CreateUserPayload { email, password, username },
                ): Json<CreateUserPayload> = {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                };
                {
                    {}
                    let __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "charted.server.ops.v1.createUser",
                                    "charted_server::routing::v1::user",
                                    tracing::Level::INFO,
                                    ::core::option::Option::Some(
                                        "crates/server/src/routing/v1/user/mod.rs",
                                    ),
                                    ::core::option::Option::Some(100u32),
                                    ::core::option::Option::Some(
                                        "charted_server::routing::v1::user",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["user.name"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = __CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Span::new(
                                meta,
                                &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &display(&username) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                },
                            )
                        } else {
                            let span = ::tracing::__macro_support::__disabled_span(
                                __CALLSITE.metadata(),
                            );
                            if match tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(
                                            &{
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &display(&username) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            },
                                        );
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    let __tracing_instrument_future = async move {
                        #[allow(
                            unknown_lints,
                            unreachable_code,
                            clippy::diverging_sub_expression,
                            clippy::let_unit_value,
                            clippy::unreachable,
                            clippy::let_with_type_underscore,
                            clippy::empty_loop
                        )]
                        if false {
                            let __tracing_attr_fake_return: api::Result<User> = loop {};
                            return __tracing_attr_fake_return;
                        }
                        {
                            if !cx.config.registrations || cx.config.single_user {
                                return Err(
                                    api::err(
                                        StatusCode::FORBIDDEN,
                                        (
                                            api::ErrorCode::RegistrationsDisabled,
                                            "this instance has user registrations disabled",
                                        ),
                                    ),
                                );
                            }
                            if cx
                                .authz
                                .as_ref()
                                .downcast::<charted_authz_local::Backend>()
                                .is_some() && password.is_none()
                            {
                                return Err(
                                    api::err(
                                        StatusCode::NOT_ACCEPTABLE,
                                        (
                                            api::ErrorCode::MissingPassword,
                                            "authentication backend requires you to include a password for this new account",
                                        ),
                                    ),
                                );
                            }
                            if !email.validate_email() {
                                return Err(
                                    api::err(
                                        StatusCode::NOT_ACCEPTABLE,
                                        (
                                            api::ErrorCode::ValidationFailed,
                                            "`email` is not a valid email",
                                            ::serde_json::Value::Object({
                                                let mut object = ::serde_json::Map::new();
                                                let _ = object
                                                    .insert(
                                                        ("email").into(),
                                                        ::serde_json::to_value(&&email).unwrap(),
                                                    );
                                                object
                                            }),
                                        ),
                                    ),
                                );
                            }
                            {
                                let mut conn = cx
                                    .pool
                                    .get()
                                    .inspect_err(|e| {
                                        sentry::capture_error(e);
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event crates/server/src/routing/v1/user/mod.rs:152",
                                                        "charted_server::routing::v1::user",
                                                        ::tracing::Level::ERROR,
                                                        ::core::option::Option::Some(
                                                            "crates/server/src/routing/v1/user/mod.rs",
                                                        ),
                                                        ::core::option::Option::Some(152u32),
                                                        ::core::option::Option::Some(
                                                            "charted_server::routing::v1::user",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message", "error"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::ERROR
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                    if match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                                    {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                            {
                                                                use ::tracing::log;
                                                                let level = match ::tracing::Level::ERROR {
                                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                    _ => ::tracing::log::Level::Trace,
                                                                };
                                                                if level <= log::max_level() {
                                                                    let meta = __CALLSITE.metadata();
                                                                    let log_meta = log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(meta.target())
                                                                        .build();
                                                                    let logger = log::logger();
                                                                    if logger.enabled(&log_meta) {
                                                                        ::tracing::__macro_support::__tracing_log(
                                                                            meta,
                                                                            logger,
                                                                            log_meta,
                                                                            &value_set,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {}
                                                        }
                                                    } else {
                                                        {}
                                                    };
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(
                                                                        &format_args!("failed to get db connection") as &dyn Value,
                                                                    ),
                                                                ),
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &{
                                                                            #[allow(unused_imports)]
                                                                            use ::tracing::field::{debug, display, Value};
                                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                                            __CALLSITE
                                                                                .metadata()
                                                                                .fields()
                                                                                .value_set(
                                                                                    &[
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(
                                                                                                &format_args!("failed to get db connection") as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                        ),
                                                                                    ],
                                                                                )
                                                                        },
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            }
                                        };
                                    })
                                    .map_err(|_| api::internal_server_error())?;
                                let uname = &username;
                                let exists = {
                                    #[allow(unused)]
                                    use ::diesel::prelude::*;
                                    match *conn {
                                        ::charted_database::DbConnection::PostgreSQL(
                                            ref mut conn,
                                        ) => {
                                            conn.build_transaction()
                                                .read_only()
                                                .run::<
                                                    _,
                                                    eyre::Report,
                                                    _,
                                                >(|txn| {
                                                    use postgresql::users::{dsl::*, table};
                                                    use diesel::pg::Pg;
                                                    match table
                                                        .select(<User as SelectableHelper<Pg>>::as_select())
                                                        .filter(username.eq(uname))
                                                        .first(txn)
                                                    {
                                                        Ok(_) => Ok(true),
                                                        Err(diesel::result::Error::NotFound) => Ok(false),
                                                        Err(e) => Err(eyre::Report::from(e)),
                                                    }
                                                })
                                        }
                                        ::charted_database::DbConnection::SQLite(ref mut conn) => {
                                            conn.immediate_transaction(|txn| {
                                                use sqlite::users::{dsl::*, table};
                                                use diesel::sqlite::Sqlite;
                                                match table
                                                    .select(<User as SelectableHelper<Sqlite>>::as_select())
                                                    .filter(username.eq(uname))
                                                    .first(txn)
                                                {
                                                    Ok(_) => Ok(true),
                                                    Err(diesel::result::Error::NotFound) => Ok(false),
                                                    Err(e) => Err(eyre::Report::from(e)),
                                                }
                                            })
                                        }
                                    }
                                }
                                    .inspect_err(|e| {
                                        sentry_eyre::capture_report(e);
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event crates/server/src/routing/v1/user/mod.rs:181",
                                                        "charted_server::routing::v1::user",
                                                        ::tracing::Level::ERROR,
                                                        ::core::option::Option::Some(
                                                            "crates/server/src/routing/v1/user/mod.rs",
                                                        ),
                                                        ::core::option::Option::Some(181u32),
                                                        ::core::option::Option::Some(
                                                            "charted_server::routing::v1::user",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message", "user.name", "error"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::ERROR
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                    if match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                                    {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                            {
                                                                use ::tracing::log;
                                                                let level = match ::tracing::Level::ERROR {
                                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                    _ => ::tracing::log::Level::Trace,
                                                                };
                                                                if level <= log::max_level() {
                                                                    let meta = __CALLSITE.metadata();
                                                                    let log_meta = log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(meta.target())
                                                                        .build();
                                                                    let logger = log::logger();
                                                                    if logger.enabled(&log_meta) {
                                                                        ::tracing::__macro_support::__tracing_log(
                                                                            meta,
                                                                            logger,
                                                                            log_meta,
                                                                            &value_set,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {}
                                                        }
                                                    } else {
                                                        {}
                                                    };
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(
                                                                        &format_args!("failed to query user by username")
                                                                            as &dyn Value,
                                                                    ),
                                                                ),
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(
                                                                        &display(&username) as &dyn Value,
                                                                    ),
                                                                ),
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &{
                                                                            #[allow(unused_imports)]
                                                                            use ::tracing::field::{debug, display, Value};
                                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                                            __CALLSITE
                                                                                .metadata()
                                                                                .fields()
                                                                                .value_set(
                                                                                    &[
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(
                                                                                                &format_args!("failed to query user by username")
                                                                                                    as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(
                                                                                                &display(&username) as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                        ),
                                                                                    ],
                                                                                )
                                                                        },
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            }
                                        };
                                    })
                                    .map_err(|_| api::internal_server_error())?;
                                if exists {
                                    return Err(
                                        api::err(
                                            StatusCode::CONFLICT,
                                            (
                                                api::ErrorCode::EntityAlreadyExists,
                                                "a user with `username` already exists",
                                                ::serde_json::Value::Object({
                                                    let mut object = ::serde_json::Map::new();
                                                    let _ = object
                                                        .insert(
                                                            ("username").into(),
                                                            ::serde_json::to_value(&uname.as_str()).unwrap(),
                                                        );
                                                    object
                                                }),
                                            ),
                                        ),
                                    );
                                }
                            }
                            {
                                let mut conn = cx
                                    .pool
                                    .get()
                                    .inspect_err(|e| {
                                        sentry::capture_error(e);
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event crates/server/src/routing/v1/user/mod.rs:203",
                                                        "charted_server::routing::v1::user",
                                                        ::tracing::Level::ERROR,
                                                        ::core::option::Option::Some(
                                                            "crates/server/src/routing/v1/user/mod.rs",
                                                        ),
                                                        ::core::option::Option::Some(203u32),
                                                        ::core::option::Option::Some(
                                                            "charted_server::routing::v1::user",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message", "error"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::ERROR
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                    if match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                                    {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                            {
                                                                use ::tracing::log;
                                                                let level = match ::tracing::Level::ERROR {
                                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                    _ => ::tracing::log::Level::Trace,
                                                                };
                                                                if level <= log::max_level() {
                                                                    let meta = __CALLSITE.metadata();
                                                                    let log_meta = log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(meta.target())
                                                                        .build();
                                                                    let logger = log::logger();
                                                                    if logger.enabled(&log_meta) {
                                                                        ::tracing::__macro_support::__tracing_log(
                                                                            meta,
                                                                            logger,
                                                                            log_meta,
                                                                            &value_set,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {}
                                                        }
                                                    } else {
                                                        {}
                                                    };
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(
                                                                        &format_args!("failed to get db connection") as &dyn Value,
                                                                    ),
                                                                ),
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &{
                                                                            #[allow(unused_imports)]
                                                                            use ::tracing::field::{debug, display, Value};
                                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                                            __CALLSITE
                                                                                .metadata()
                                                                                .fields()
                                                                                .value_set(
                                                                                    &[
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(
                                                                                                &format_args!("failed to get db connection") as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                        ),
                                                                                    ],
                                                                                )
                                                                        },
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            }
                                        };
                                    })
                                    .map_err(|_| api::internal_server_error())?;
                                let em = &email;
                                let exists = {
                                    #[allow(unused)]
                                    use ::diesel::prelude::*;
                                    match *conn {
                                        ::charted_database::DbConnection::PostgreSQL(
                                            ref mut conn,
                                        ) => {
                                            conn.build_transaction()
                                                .read_only()
                                                .run::<
                                                    _,
                                                    eyre::Report,
                                                    _,
                                                >(|txn| {
                                                    use postgresql::users::{dsl::*, table};
                                                    use diesel::pg::Pg;
                                                    match table
                                                        .select(<User as SelectableHelper<Pg>>::as_select())
                                                        .filter(email.eq(em))
                                                        .first(txn)
                                                    {
                                                        Ok(_) => Ok(true),
                                                        Err(diesel::result::Error::NotFound) => Ok(false),
                                                        Err(e) => Err(eyre::Report::from(e)),
                                                    }
                                                })
                                        }
                                        ::charted_database::DbConnection::SQLite(ref mut conn) => {
                                            conn.immediate_transaction(|txn| {
                                                use sqlite::users::{dsl::*, table};
                                                use diesel::sqlite::Sqlite;
                                                match table
                                                    .select(<User as SelectableHelper<Sqlite>>::as_select())
                                                    .filter(email.eq(em))
                                                    .first(txn)
                                                {
                                                    Ok(_) => Ok(true),
                                                    Err(diesel::result::Error::NotFound) => Ok(false),
                                                    Err(e) => Err(eyre::Report::from(e)),
                                                }
                                            })
                                        }
                                    }
                                }
                                    .inspect_err(|e| {
                                        sentry_eyre::capture_report(e);
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event crates/server/src/routing/v1/user/mod.rs:233",
                                                        "charted_server::routing::v1::user",
                                                        ::tracing::Level::ERROR,
                                                        ::core::option::Option::Some(
                                                            "crates/server/src/routing/v1/user/mod.rs",
                                                        ),
                                                        ::core::option::Option::Some(233u32),
                                                        ::core::option::Option::Some(
                                                            "charted_server::routing::v1::user",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message", "user.email", "error"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::ERROR
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                    if match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                                    {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                            {
                                                                use ::tracing::log;
                                                                let level = match ::tracing::Level::ERROR {
                                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                    _ => ::tracing::log::Level::Trace,
                                                                };
                                                                if level <= log::max_level() {
                                                                    let meta = __CALLSITE.metadata();
                                                                    let log_meta = log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(meta.target())
                                                                        .build();
                                                                    let logger = log::logger();
                                                                    if logger.enabled(&log_meta) {
                                                                        ::tracing::__macro_support::__tracing_log(
                                                                            meta,
                                                                            logger,
                                                                            log_meta,
                                                                            &value_set,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {}
                                                        }
                                                    } else {
                                                        {}
                                                    };
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(
                                                                        &format_args!("failed to query user by email") as &dyn Value,
                                                                    ),
                                                                ),
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(&em as &dyn Value),
                                                                ),
                                                                (
                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &{
                                                                            #[allow(unused_imports)]
                                                                            use ::tracing::field::{debug, display, Value};
                                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                                            __CALLSITE
                                                                                .metadata()
                                                                                .fields()
                                                                                .value_set(
                                                                                    &[
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(
                                                                                                &format_args!("failed to query user by email") as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(&em as &dyn Value),
                                                                                        ),
                                                                                        (
                                                                                            &::core::iter::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                        ),
                                                                                    ],
                                                                                )
                                                                        },
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            }
                                        };
                                    })
                                    .map_err(|_| api::internal_server_error())?;
                                if exists {
                                    return Err(
                                        api::err(
                                            StatusCode::CONFLICT,
                                            (
                                                api::ErrorCode::EntityAlreadyExists,
                                                "a user with the `email` given already exists",
                                                ::serde_json::Value::Object({
                                                    let mut object = ::serde_json::Map::new();
                                                    let _ = object
                                                        .insert(
                                                            ("email").into(),
                                                            ::serde_json::to_value(&em).unwrap(),
                                                        );
                                                    object
                                                }),
                                            ),
                                        ),
                                    );
                                }
                            }
                            let password = if let Some(ref password) = password {
                                if password.len() < 8 {
                                    return Err(
                                        api::err(
                                            StatusCode::NOT_ACCEPTABLE,
                                            (
                                                api::ErrorCode::InvalidPassword,
                                                "`password` length was expected to be 8 characters or longer",
                                            ),
                                        ),
                                    );
                                }
                                Some(
                                    hash_password(password)
                                        .map_err(|_| api::internal_server_error())?,
                                )
                            } else {
                                None
                            };
                            let id = cx
                                .ulid_generator
                                .generate()
                                .inspect_err(|e| {
                                    sentry::capture_error(e);
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/server/src/routing/v1/user/mod.rs:270",
                                                    "charted_server::routing::v1::user",
                                                    ::tracing::Level::ERROR,
                                                    ::core::option::Option::Some(
                                                        "crates/server/src/routing/v1/user/mod.rs",
                                                    ),
                                                    ::core::option::Option::Some(270u32),
                                                    ::core::option::Option::Some(
                                                        "charted_server::routing::v1::user",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &format_args!(
                                                                        "received monotonic overflow -- please inspect this as fast you can!!!!!",
                                                                    ) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(
                                                                                            &format_args!(
                                                                                                "received monotonic overflow -- please inspect this as fast you can!!!!!",
                                                                                            ) as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                })
                                .map_err(|_| api::internal_server_error())?;
                            let user = User {
                                verified_publisher: false,
                                gravatar_email: None,
                                description: None,
                                avatar_hash: None,
                                created_at: chrono::Utc::now().into(),
                                updated_at: chrono::Utc::now().into(),
                                password,
                                username,
                                email,
                                admin: false,
                                name: None,
                                id: id.into(),
                            };
                            ops::charts::create_index(&cx, &user)
                                .await
                                .map_err(|_| api::internal_server_error())?;
                            Ok(api::ok(StatusCode::CREATED, user))
                        }
                    };
                    if !__tracing_attr_span.is_disabled() {
                        tracing::Instrument::instrument(
                                __tracing_instrument_future,
                                __tracing_attr_span,
                            )
                            .await
                    } else {
                        __tracing_instrument_future.await
                    }
                }
            }
            let value = __axum_macros_check_create_user_into_response_make_value().await;
            fn check<T>(_: T)
            where
                T: ::axum::response::IntoResponse,
            {}
            check(value);
        }
        #[allow(warnings)]
        #[doc(hidden)]
        fn __axum_macros_check_create_user_0_from_request_check()
        where
            State<
                ServerContext,
            >: ::axum::extract::FromRequestParts<ServerContext> + Send,
        {}
        #[allow(warnings)]
        #[doc(hidden)]
        fn __axum_macros_check_create_user_0_from_request_call_check() {
            __axum_macros_check_create_user_0_from_request_check();
        }
        #[allow(warnings)]
        #[doc(hidden)]
        fn __axum_macros_check_create_user_1_from_request_check()
        where
            Json<CreateUserPayload>: ::axum::extract::FromRequest<ServerContext> + Send,
        {}
        #[allow(warnings)]
        #[doc(hidden)]
        fn __axum_macros_check_create_user_1_from_request_call_check() {
            __axum_macros_check_create_user_1_from_request_check();
        }
        #[allow(warnings)]
        #[allow(unreachable_code)]
        #[doc(hidden)]
        fn __axum_macros_check_create_user_future() {
            pub async fn create_user(
                State(cx): State<ServerContext>,
                Json(
                    CreateUserPayload { email, password, username },
                ): Json<CreateUserPayload>,
            ) -> api::Result<User> {
                {}
                let __tracing_attr_span = {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "charted.server.ops.v1.createUser",
                                "charted_server::routing::v1::user",
                                tracing::Level::INFO,
                                ::core::option::Option::Some(
                                    "crates/server/src/routing/v1/user/mod.rs",
                                ),
                                ::core::option::Option::Some(100u32),
                                ::core::option::Option::Some(
                                    "charted_server::routing::v1::user",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["user.name"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let mut interest = ::tracing::subscriber::Interest::never();
                    if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            interest = __CALLSITE.interest();
                            !interest.is_never()
                        }
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                    {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Span::new(
                            meta,
                            &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = meta.fields().iter();
                                meta.fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(
                                                    &display(&username) as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            },
                        )
                    } else {
                        let span = ::tracing::__macro_support::__disabled_span(
                            __CALLSITE.metadata(),
                        );
                        if match tracing::Level::INFO {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    span.record_all(
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &display(&username) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    );
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                        span
                    }
                };
                let __tracing_instrument_future = async move {
                    #[allow(
                        unknown_lints,
                        unreachable_code,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::unreachable,
                        clippy::let_with_type_underscore,
                        clippy::empty_loop
                    )]
                    if false {
                        let __tracing_attr_fake_return: api::Result<User> = loop {};
                        return __tracing_attr_fake_return;
                    }
                    {
                        if !cx.config.registrations || cx.config.single_user {
                            return Err(
                                api::err(
                                    StatusCode::FORBIDDEN,
                                    (
                                        api::ErrorCode::RegistrationsDisabled,
                                        "this instance has user registrations disabled",
                                    ),
                                ),
                            );
                        }
                        if cx
                            .authz
                            .as_ref()
                            .downcast::<charted_authz_local::Backend>()
                            .is_some() && password.is_none()
                        {
                            return Err(
                                api::err(
                                    StatusCode::NOT_ACCEPTABLE,
                                    (
                                        api::ErrorCode::MissingPassword,
                                        "authentication backend requires you to include a password for this new account",
                                    ),
                                ),
                            );
                        }
                        if !email.validate_email() {
                            return Err(
                                api::err(
                                    StatusCode::NOT_ACCEPTABLE,
                                    (
                                        api::ErrorCode::ValidationFailed,
                                        "`email` is not a valid email",
                                        ::serde_json::Value::Object({
                                            let mut object = ::serde_json::Map::new();
                                            let _ = object
                                                .insert(
                                                    ("email").into(),
                                                    ::serde_json::to_value(&&email).unwrap(),
                                                );
                                            object
                                        }),
                                    ),
                                ),
                            );
                        }
                        {
                            let mut conn = cx
                                .pool
                                .get()
                                .inspect_err(|e| {
                                    sentry::capture_error(e);
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/server/src/routing/v1/user/mod.rs:152",
                                                    "charted_server::routing::v1::user",
                                                    ::tracing::Level::ERROR,
                                                    ::core::option::Option::Some(
                                                        "crates/server/src/routing/v1/user/mod.rs",
                                                    ),
                                                    ::core::option::Option::Some(152u32),
                                                    ::core::option::Option::Some(
                                                        "charted_server::routing::v1::user",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message", "error"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &format_args!("failed to get db connection") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(
                                                                                            &format_args!("failed to get db connection") as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                })
                                .map_err(|_| api::internal_server_error())?;
                            let uname = &username;
                            let exists = {
                                #[allow(unused)]
                                use ::diesel::prelude::*;
                                match *conn {
                                    ::charted_database::DbConnection::PostgreSQL(
                                        ref mut conn,
                                    ) => {
                                        conn.build_transaction()
                                            .read_only()
                                            .run::<
                                                _,
                                                eyre::Report,
                                                _,
                                            >(|txn| {
                                                use postgresql::users::{dsl::*, table};
                                                use diesel::pg::Pg;
                                                match table
                                                    .select(<User as SelectableHelper<Pg>>::as_select())
                                                    .filter(username.eq(uname))
                                                    .first(txn)
                                                {
                                                    Ok(_) => Ok(true),
                                                    Err(diesel::result::Error::NotFound) => Ok(false),
                                                    Err(e) => Err(eyre::Report::from(e)),
                                                }
                                            })
                                    }
                                    ::charted_database::DbConnection::SQLite(ref mut conn) => {
                                        conn.immediate_transaction(|txn| {
                                            use sqlite::users::{dsl::*, table};
                                            use diesel::sqlite::Sqlite;
                                            match table
                                                .select(<User as SelectableHelper<Sqlite>>::as_select())
                                                .filter(username.eq(uname))
                                                .first(txn)
                                            {
                                                Ok(_) => Ok(true),
                                                Err(diesel::result::Error::NotFound) => Ok(false),
                                                Err(e) => Err(eyre::Report::from(e)),
                                            }
                                        })
                                    }
                                }
                            }
                                .inspect_err(|e| {
                                    sentry_eyre::capture_report(e);
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/server/src/routing/v1/user/mod.rs:181",
                                                    "charted_server::routing::v1::user",
                                                    ::tracing::Level::ERROR,
                                                    ::core::option::Option::Some(
                                                        "crates/server/src/routing/v1/user/mod.rs",
                                                    ),
                                                    ::core::option::Option::Some(181u32),
                                                    ::core::option::Option::Some(
                                                        "charted_server::routing::v1::user",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message", "user.name", "error"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &format_args!("failed to query user by username")
                                                                        as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &display(&username) as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(
                                                                                            &format_args!("failed to query user by username")
                                                                                                as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(
                                                                                            &display(&username) as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                })
                                .map_err(|_| api::internal_server_error())?;
                            if exists {
                                return Err(
                                    api::err(
                                        StatusCode::CONFLICT,
                                        (
                                            api::ErrorCode::EntityAlreadyExists,
                                            "a user with `username` already exists",
                                            ::serde_json::Value::Object({
                                                let mut object = ::serde_json::Map::new();
                                                let _ = object
                                                    .insert(
                                                        ("username").into(),
                                                        ::serde_json::to_value(&uname.as_str()).unwrap(),
                                                    );
                                                object
                                            }),
                                        ),
                                    ),
                                );
                            }
                        }
                        {
                            let mut conn = cx
                                .pool
                                .get()
                                .inspect_err(|e| {
                                    sentry::capture_error(e);
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/server/src/routing/v1/user/mod.rs:203",
                                                    "charted_server::routing::v1::user",
                                                    ::tracing::Level::ERROR,
                                                    ::core::option::Option::Some(
                                                        "crates/server/src/routing/v1/user/mod.rs",
                                                    ),
                                                    ::core::option::Option::Some(203u32),
                                                    ::core::option::Option::Some(
                                                        "charted_server::routing::v1::user",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message", "error"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &format_args!("failed to get db connection") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(
                                                                                            &format_args!("failed to get db connection") as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                })
                                .map_err(|_| api::internal_server_error())?;
                            let em = &email;
                            let exists = {
                                #[allow(unused)]
                                use ::diesel::prelude::*;
                                match *conn {
                                    ::charted_database::DbConnection::PostgreSQL(
                                        ref mut conn,
                                    ) => {
                                        conn.build_transaction()
                                            .read_only()
                                            .run::<
                                                _,
                                                eyre::Report,
                                                _,
                                            >(|txn| {
                                                use postgresql::users::{dsl::*, table};
                                                use diesel::pg::Pg;
                                                match table
                                                    .select(<User as SelectableHelper<Pg>>::as_select())
                                                    .filter(email.eq(em))
                                                    .first(txn)
                                                {
                                                    Ok(_) => Ok(true),
                                                    Err(diesel::result::Error::NotFound) => Ok(false),
                                                    Err(e) => Err(eyre::Report::from(e)),
                                                }
                                            })
                                    }
                                    ::charted_database::DbConnection::SQLite(ref mut conn) => {
                                        conn.immediate_transaction(|txn| {
                                            use sqlite::users::{dsl::*, table};
                                            use diesel::sqlite::Sqlite;
                                            match table
                                                .select(<User as SelectableHelper<Sqlite>>::as_select())
                                                .filter(email.eq(em))
                                                .first(txn)
                                            {
                                                Ok(_) => Ok(true),
                                                Err(diesel::result::Error::NotFound) => Ok(false),
                                                Err(e) => Err(eyre::Report::from(e)),
                                            }
                                        })
                                    }
                                }
                            }
                                .inspect_err(|e| {
                                    sentry_eyre::capture_report(e);
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/server/src/routing/v1/user/mod.rs:233",
                                                    "charted_server::routing::v1::user",
                                                    ::tracing::Level::ERROR,
                                                    ::core::option::Option::Some(
                                                        "crates/server/src/routing/v1/user/mod.rs",
                                                    ),
                                                    ::core::option::Option::Some(233u32),
                                                    ::core::option::Option::Some(
                                                        "charted_server::routing::v1::user",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message", "user.email", "error"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::ERROR
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::ERROR {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::ERROR {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &format_args!("failed to query user by email") as &dyn Value,
                                                                ),
                                                            ),
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(&em as &dyn Value),
                                                            ),
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(
                                                                                            &format_args!("failed to query user by email") as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(&em as &dyn Value),
                                                                                    ),
                                                                                    (
                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::core::option::Option::Some(&display(&e) as &dyn Value),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                })
                                .map_err(|_| api::internal_server_error())?;
                            if exists {
                                return Err(
                                    api::err(
                                        StatusCode::CONFLICT,
                                        (
                                            api::ErrorCode::EntityAlreadyExists,
                                            "a user with the `email` given already exists",
                                            ::serde_json::Value::Object({
                                                let mut object = ::serde_json::Map::new();
                                                let _ = object
                                                    .insert(
                                                        ("email").into(),
                                                        ::serde_json::to_value(&em).unwrap(),
                                                    );
                                                object
                                            }),
                                        ),
                                    ),
                                );
                            }
                        }
                        let password = if let Some(ref password) = password {
                            if password.len() < 8 {
                                return Err(
                                    api::err(
                                        StatusCode::NOT_ACCEPTABLE,
                                        (
                                            api::ErrorCode::InvalidPassword,
                                            "`password` length was expected to be 8 characters or longer",
                                        ),
                                    ),
                                );
                            }
                            Some(
                                hash_password(password)
                                    .map_err(|_| api::internal_server_error())?,
                            )
                        } else {
                            None
                        };
                        let id = cx
                            .ulid_generator
                            .generate()
                            .inspect_err(|e| {
                                sentry::capture_error(e);
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/server/src/routing/v1/user/mod.rs:270",
                                                "charted_server::routing::v1::user",
                                                ::tracing::Level::ERROR,
                                                ::core::option::Option::Some(
                                                    "crates/server/src/routing/v1/user/mod.rs",
                                                ),
                                                ::core::option::Option::Some(270u32),
                                                ::core::option::Option::Some(
                                                    "charted_server::routing::v1::user",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::ERROR {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::ERROR {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!(
                                                                    "received monotonic overflow -- please inspect this as fast you can!!!!!",
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::ERROR {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!(
                                                                                            "received monotonic overflow -- please inspect this as fast you can!!!!!",
                                                                                        ) as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                            })
                            .map_err(|_| api::internal_server_error())?;
                        let user = User {
                            verified_publisher: false,
                            gravatar_email: None,
                            description: None,
                            avatar_hash: None,
                            created_at: chrono::Utc::now().into(),
                            updated_at: chrono::Utc::now().into(),
                            password,
                            username,
                            email,
                            admin: false,
                            name: None,
                            id: id.into(),
                        };
                        ops::charts::create_index(&cx, &user)
                            .await
                            .map_err(|_| api::internal_server_error())?;
                        Ok(api::ok(StatusCode::CREATED, user))
                    }
                };
                if !__tracing_attr_span.is_disabled() {
                    tracing::Instrument::instrument(
                            __tracing_instrument_future,
                            __tracing_attr_span,
                        )
                        .await
                } else {
                    __tracing_instrument_future.await
                }
            }
            let future = create_user(
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                },
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                },
            );
            fn check<T>(_: T)
            where
                T: ::std::future::Future + Send,
            {}
            check(future);
        }
    }
    use crate::ServerContext;
    use axum::{
        extract::Request, http::StatusCode, response::IntoResponse, routing, Router,
    };
    use charted_core::{api, VERSION};
    use charted_proc_macros::generate_api_response;
    use serde::Serialize;
    use serde_json::json;
    use std::borrow::Cow;
    use utoipa::ToSchema;
    /// Generic entrypoint message for any API route like `/users`.
    pub struct EntrypointResponse {
        /// Humane message to greet you.
        pub message: Cow<'static, str>,
        /// URI to the documentation for this entrypoint.
        pub docs: Cow<'static, str>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EntrypointResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "EntrypointResponse",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message",
                    &self.message,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "docs",
                    &self.docs,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl utoipa::__dev::ComposeSchema for EntrypointResponse {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    "message",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(
                            utoipa::openapi::schema::SchemaType::new(
                                utoipa::openapi::schema::Type::String,
                            ),
                        )
                        .description(Some("Humane message to greet you.")),
                )
                .required("message")
                .property(
                    "docs",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(
                            utoipa::openapi::schema::SchemaType::new(
                                utoipa::openapi::schema::Type::String,
                            ),
                        )
                        .description(
                            Some("URI to the documentation for this entrypoint."),
                        ),
                )
                .required("docs")
                .description(
                    Some("Generic entrypoint message for any API route like `/users`."),
                )
                .into()
        }
    }
    impl utoipa::ToSchema for EntrypointResponse {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("EntrypointResponse")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas.extend([]);
        }
    }
    impl EntrypointResponse {
        pub fn new(entity: impl AsRef<str>) -> EntrypointResponse {
            let entity = entity.as_ref();
            EntrypointResponse {
                message: Cow::Owned(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("welcome to the {0} API", entity),
                        );
                        res
                    }),
                ),
                docs: Cow::Owned(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "https://charts.noelware.org/docs/server/{1}/api/reference/{0}",
                                entity.to_lowercase().replace(' ', ""),
                                VERSION,
                            ),
                        );
                        res
                    }),
                ),
            }
        }
    }
    #[automatically_derived]
    impl<'r> ::utoipa::ToResponse<'r> for EntrypointResponse {
        fn response() -> (
            &'r str,
            ::utoipa::openapi::RefOr<::utoipa::openapi::Response>,
        ) {
            let __datatype_schema = <EntrypointResponse as ::utoipa::ToSchema>::name();
            let __schema = ::utoipa::openapi::Schema::Object({
                let __object = ::utoipa::openapi::ObjectBuilder::new()
                    .property(
                        "success",
                        ::utoipa::openapi::ObjectBuilder::new()
                            .description(Some("whether if this request was successful"))
                            .schema_type(
                                ::utoipa::openapi::schema::SchemaType::Type(
                                    ::utoipa::openapi::schema::Type::Boolean,
                                ),
                            )
                            .build(),
                    )
                    .required("success")
                    .property(
                        "data",
                        ::utoipa::openapi::RefOr::Ref(
                            ::utoipa::openapi::Ref::from_schema_name(__datatype_schema),
                        ),
                    )
                    .required("data")
                    .build();
                __object
            });
            let __response = ::utoipa::openapi::ResponseBuilder::new()
                .content(
                    "application/json",
                    ::utoipa::openapi::ContentBuilder::new()
                        .schema(::core::option::Option::Some(__schema))
                        .build(),
                )
                .build();
            let __name = "EntrypointResponse";
            (__name, ::utoipa::openapi::RefOr::T(__response))
        }
    }
    pub fn create_router(_: &ServerContext) -> Router<ServerContext> {
        Router::new()
            .nest("/users", user::create_router())
            .route("/indexes/:idOrName", routing::get(index::get_chart_index))
            .route("/heartbeat", routing::get(heartbeat::heartbeat))
            .route("/openapi.json", routing::get(openapi::openapi))
            .route("/info", routing::get(info::info))
            .route("/", routing::get(main::main))
            .fallback(fallback)
    }
    async fn fallback(req: Request) -> impl IntoResponse {
        api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::HandlerNotFound,
                "endpoint was not found",
                ::serde_json::Value::Object({
                    let mut object = ::serde_json::Map::new();
                    let _ = object
                        .insert(
                            ("method").into(),
                            ::serde_json::to_value(&req.method().as_str()).unwrap(),
                        );
                    let _ = object
                        .insert(
                            ("uri").into(),
                            ::serde_json::to_value(&req.uri().path()).unwrap(),
                        );
                    object
                }),
            ),
        )
    }
}
