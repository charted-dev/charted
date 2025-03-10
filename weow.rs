#![feature(prelude_import)]
#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod api {
    //! Types that are used with the API server.
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use std::borrow::Cow;
    /// Specification version for charted's HTTP specification.
    #[display("{}", self.as_str())]
    #[repr(u8)]
    pub enum Version {
        /// ## `v1`
        ///
        /// Released since the initial release of **charted-server**.
        #[default]
        V1 = 1,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Version {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "V1")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Version {
        #[inline]
        fn clone(&self) -> Version {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Version {}
    #[automatically_derived]
    impl ::core::default::Default for Version {
        #[inline]
        fn default() -> Version {
            Self::V1
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Version {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Version {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Version {
        #[inline]
        fn eq(&self, other: &Version) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Version {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Version {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Version,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Version {
        #[inline]
        fn cmp(&self, other: &Version) -> ::core::cmp::Ordering {
            ::core::cmp::Ordering::Equal
        }
    }
    #[allow(deprecated)]
    impl serde::Serialize for Version {
        #[allow(clippy::use_self)]
        fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let value: u8 = match *self {
                Version::V1 => Version::V1 as u8,
            };
            serde::Serialize::serialize(&value, serializer)
        }
    }
    #[allow(deprecated)]
    impl<'de> serde::Deserialize<'de> for Version {
        #[allow(clippy::use_self)]
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            struct discriminant;
            #[allow(non_upper_case_globals)]
            impl discriminant {
                const V1: u8 = Version::V1 as u8;
            }
            match <u8 as serde::Deserialize>::deserialize(deserializer)? {
                discriminant::V1 => ::core::result::Result::Ok(Version::V1),
                other => {
                    ::core::result::Result::Err(
                        serde::de::Error::custom(
                            format_args!(
                                "invalid value: {0}, expected {1}",
                                other,
                                discriminant::V1,
                            ),
                        ),
                    )
                }
            }
        }
    }
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Display for Version {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match self {
                Self::V1 => {
                    derive_more::core::fmt::Display::fmt(
                        &(self.as_str()),
                        __derive_more_f,
                    )
                }
            }
        }
    }
    impl Version {
        pub const fn as_str(&self) -> &str {
            match self {
                Version::V1 => "v1",
            }
        }
        pub const fn as_slice<'a>() -> &'a [Version] {
            &[Version::V1]
        }
    }
    impl From<u8> for Version {
        fn from(value: u8) -> Self {
            match value {
                1 => Version::V1,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "reached an unexpected value for From<u8> -> APIVersion",
                        ),
                    );
                }
            }
        }
    }
    impl From<Version> for u8 {
        fn from(value: Version) -> Self {
            match value {
                Version::V1 => 1,
            }
        }
    }
    impl From<Version> for serde_json::Number {
        fn from(value: Version) -> Self {
            match value {
                Version::V1 => serde_json::Number::from(1),
            }
        }
    }
    #[cfg(feature = "schemars")]
    impl ::schemars::JsonSchema for Version {
        fn is_referenceable() -> bool {
            false
        }
        fn schema_id() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("charted_core::api::Version")
        }
        fn schema_name() -> String {
            String::from("Version")
        }
        fn json_schema(
            _: &mut ::schemars::r#gen::SchemaGenerator,
        ) -> ::schemars::schema::Schema {
            ::schemars::schema::Schema::Object(::schemars::schema::SchemaObject {
                instance_type: Some(
                    ::schemars::schema::SingleOrVec::Single(
                        ::schemars::schema::InstanceType::Number.into(),
                    ),
                ),
                enum_values: Some(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([::serde_json::Value::Number(1.into())]),
                    ),
                ),
                ..Default::default()
            })
        }
    }
    pub type Result<T> = std::result::Result<Response<T>, Response>;
    /// Representation of a response that the API server sends for each request.
    pub struct Response<T = ()> {
        /// The status of the response.
        #[cfg(feature = "axum")]
        #[serde(skip)]
        pub status: ::axum::http::StatusCode,
        /// Was the request that was processed a success?
        pub success: bool,
        /// The data that the REST endpoint sends back, if any.
        ///
        /// When this field is empty, it'll always respond with a `204 No Content`
        /// status code if `errors` is also empty.
        ///
        /// The `success` field will always be set to `true` when
        /// the `data` field is avaliable. All errors are handled
        /// by the `errors` field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<T>,
        /// The error trace for the request that was processed by
        /// the API server.
        ///
        /// The `success` field will always be set to `false` when
        /// the `errors` field is avaliable.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub errors: Vec<Error>,
    }
    impl<T> utoipa::__dev::ComposeSchema for Response<T>
    where
        T: utoipa::ToSchema,
    {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "success",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::Boolean,
                                ),
                            )
                            .description(
                                Some("Was the request that was processed a success?"),
                            ),
                    )
                    .required("success");
                object = object
                    .property(
                        "data",
                        {
                            let _ = <T as utoipa::PartialSchema>::schema;
                            if let Some(composed) = generics.get_mut(0usize) {
                                composed.clone()
                            } else {
                                utoipa::openapi::schema::OneOfBuilder::new()
                                    .item(
                                        utoipa::openapi::schema::ObjectBuilder::new()
                                            .schema_type(utoipa::openapi::schema::Type::Null),
                                    )
                                    .item(
                                        utoipa::openapi::schema::RefBuilder::new()
                                            .description(
                                                Some(
                                                    "The data that the REST endpoint sends back, if any.\n\nWhen this field is empty, it'll always respond with a `204 No Content`\nstatus code if `errors` is also empty.\n\nThe `success` field will always be set to `true` when\nthe `data` field is avaliable. All errors are handled\nby the `errors` field.",
                                                ),
                                            )
                                            .ref_location_from_schema_name(
                                                ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("{0}", <T as utoipa::ToSchema>::name()),
                                                    );
                                                    res
                                                }),
                                            ),
                                    )
                                    .into()
                            }
                        },
                    );
                object = object
                    .property(
                        "errors",
                        utoipa::openapi::schema::ArrayBuilder::new()
                            .items(
                                utoipa::openapi::schema::RefBuilder::new()
                                    .ref_location_from_schema_name(
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0}", <Error as utoipa::ToSchema>::name()),
                                            );
                                            res
                                        }),
                                    ),
                            )
                            .description(
                                Some(
                                    "The error trace for the request that was processed by\nthe API server.\n\nThe `success` field will always be set to `false` when\nthe `errors` field is avaliable.",
                                ),
                            ),
                    );
                object
            }
                .description(
                    Some(
                        "Representation of a response that the API server sends for each request.",
                    ),
                )
                .into()
        }
    }
    impl<T> utoipa::ToSchema for Response<T>
    where
        T: utoipa::ToSchema,
    {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("Response")
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
                                    format_args!("{0}", <Error as utoipa::ToSchema>::name()),
                                );
                                res
                            }),
                        ),
                        <Error as utoipa::PartialSchema>::schema(),
                    ),
                ]);
            <Error as utoipa::ToSchema>::schemas(schemas);
            <T as utoipa::ToSchema>::schemas(schemas);
        }
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Response<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Response",
                "status",
                &self.status,
                "success",
                &self.success,
                "data",
                &self.data,
                "errors",
                &&self.errors,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<T> _serde::Serialize for Response<T>
        where
            T: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Response",
                    false as usize + 1 + if Option::is_none(&self.data) { 0 } else { 1 }
                        + if Vec::is_empty(&self.errors) { 0 } else { 1 },
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "success",
                    &self.success,
                )?;
                if !Option::is_none(&self.data) {
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "data",
                        &self.data,
                    )?;
                } else {
                    _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "data",
                    )?;
                }
                if !Vec::is_empty(&self.errors) {
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "errors",
                        &self.errors,
                    )?;
                } else {
                    _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "errors",
                    )?;
                }
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, T> _serde::Deserialize<'de> for Response<T>
        where
            T: _serde::Deserialize<'de>,
            T: _serde::__private::Default,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field1),
                            1u64 => _serde::__private::Ok(__Field::__field2),
                            2u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "success" => _serde::__private::Ok(__Field::__field1),
                            "data" => _serde::__private::Ok(__Field::__field2),
                            "errors" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"success" => _serde::__private::Ok(__Field::__field1),
                            b"data" => _serde::__private::Ok(__Field::__field2),
                            b"errors" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                    T: _serde::__private::Default,
                {
                    marker: _serde::__private::PhantomData<Response<T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T>
                where
                    T: _serde::Deserialize<'de>,
                    T: _serde::__private::Default,
                {
                    type Value = Response<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Response",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = _serde::__private::Default::default();
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Response with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<T>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Vec<Error>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        _serde::__private::Ok(Response {
                            status: __field0,
                            success: __field1,
                            data: __field2,
                            errors: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<T>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Vec<Error>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "success",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Option<T>>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("errors"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Vec<Error>>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("success")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        _serde::__private::Ok(Response {
                            status: _serde::__private::Default::default(),
                            success: __field1,
                            data: __field2,
                            errors: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["success", "data", "errors"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Response",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Response<T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<T: PartialEq> PartialEq for Response<T> {
        fn eq(&self, other: &Self) -> bool {
            self.success == other.success && self.data.eq(&other.data)
                && self.errors.eq(&other.errors)
        }
    }
    impl<T: Eq> Eq for Response<T> {}
    #[cfg(feature = "axum")]
    impl<T: Serialize> ::axum::response::IntoResponse for Response<T> {
        fn into_response(self) -> axum::response::Response {
            let data = serde_json::to_string(&self).unwrap();
            axum::http::Response::builder()
                .status(self.status)
                .header(
                    axum::http::header::CONTENT_TYPE,
                    "application/json; charset=utf-8",
                )
                .body(axum::body::Body::from(data))
                .unwrap()
        }
    }
    /// Representation of a error from an error trace.
    pub struct Error {
        /// Contextualized error code on why this request failed.
        ///
        /// This field can be looked up from the documentation to give
        /// a better representation of the error.
        pub code: ErrorCode,
        /// A humane description based off the contextualised `"code"` field.
        pub message: Cow<'static, str>,
        /// If provided, this gives more information about the error
        /// and why it could've possibly failed.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<Value>,
    }
    impl utoipa::__dev::ComposeSchema for Error {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "code",
                        utoipa::openapi::schema::RefBuilder::new()
                            .description(
                                Some(
                                    "Contextualized error code on why this request failed.\n\nThis field can be looked up from the documentation to give\na better representation of the error.",
                                ),
                            )
                            .ref_location_from_schema_name(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}", <ErrorCode as utoipa::ToSchema>::name()),
                                    );
                                    res
                                }),
                            ),
                    )
                    .required("code");
                object = object
                    .property(
                        "message",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            )
                            .description(
                                Some(
                                    "A humane description based off the contextualised `\"code\"` field.",
                                ),
                            ),
                    )
                    .required("message");
                object = object
                    .property(
                        "details",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::schema::SchemaType::AnyValue)
                            .description(
                                Some(
                                    "If provided, this gives more information about the error\nand why it could've possibly failed.",
                                ),
                            ),
                    );
                object
            }
                .description(Some("Representation of a error from an error trace."))
                .into()
        }
    }
    impl utoipa::ToSchema for Error {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("Error")
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
                                    format_args!("{0}", <ErrorCode as utoipa::ToSchema>::name()),
                                );
                                res
                            }),
                        ),
                        <ErrorCode as utoipa::PartialSchema>::schema(),
                    ),
                ]);
            <ErrorCode as utoipa::ToSchema>::schemas(schemas);
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Error",
                "code",
                &self.code,
                "message",
                &self.message,
                "details",
                &&self.details,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Error {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Error",
                    false as usize + 1 + 1
                        + if Option::is_none(&self.details) { 0 } else { 1 },
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "code",
                    &self.code,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message",
                    &self.message,
                )?;
                if !Option::is_none(&self.details) {
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "details",
                        &self.details,
                    )?;
                } else {
                    _serde::ser::SerializeStruct::skip_field(
                        &mut __serde_state,
                        "details",
                    )?;
                }
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Error {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "code" => _serde::__private::Ok(__Field::__field0),
                            "message" => _serde::__private::Ok(__Field::__field1),
                            "details" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"code" => _serde::__private::Ok(__Field::__field0),
                            b"message" => _serde::__private::Ok(__Field::__field1),
                            b"details" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Error>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Error;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Error",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            ErrorCode,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Error with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Cow<'static, str>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Error with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<Value>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        _serde::__private::Ok(Error {
                            code: __field0,
                            message: __field1,
                            details: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<ErrorCode> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Cow<'static, str>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<Value>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("code"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<ErrorCode>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "message",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Cow<'static, str>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "details",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Value>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("code")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("message")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        _serde::__private::Ok(Error {
                            code: __field0,
                            message: __field1,
                            details: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["code", "message", "details"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Error",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Error>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Error {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Error {
        #[inline]
        fn eq(&self, other: &Error) -> bool {
            self.code == other.code && self.message == other.message
                && self.details == other.details
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Error {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<ErrorCode>;
            let _: ::core::cmp::AssertParamIsEq<Cow<'static, str>>;
            let _: ::core::cmp::AssertParamIsEq<Option<Value>>;
        }
    }
    /// Contextualized error code on why this request failed.
    ///
    /// This field can be looked up from the documentation to give
    /// a better representation of the error.
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ErrorCode {
        /// A system failure occurred.
        SystemFailure,
        /// Unexpected EOF when encoding or decoding data.
        UnexpectedEOF,
        /// The endpoint that you're trying to reach is not avaliable.
        RestEndpointNotFound,
        /// The endpoint that you're trying to reach is using an invalid HTTP method.
        InvalidHTTPMethod,
        /// The entity was not found.
        EntityNotFound,
        /// The entity already exists.
        EntityAlreadyExists,
        /// Unexpected internal server error.
        InternalServerError,
        /// Validation for the input data received failed.
        ValidationFailed,
        /// The `Content-Type` header value was invalid.
        InvalidContentType,
        /// Received an invalid HTTP header name.
        InvalidHTTPHeaderName,
        /// Received an invalid HTTP header name.
        InvalidHTTPHeaderValue,
        /// This endpoint only allows Bearer tokens.
        RequiresSessionToken,
        /// Unable to decode base64 content given.
        UnableToDecodeBase64,
        /// Unable to decode ULID given.
        UnableToDecodeUlid,
        /// received invalid UTF-8 data
        InvalidUtf8,
        /// received invalid request body
        InvalidBody,
        /// missing a required header
        MissingHeader,
        /// registrations are disabled
        RegistrationsDisabled,
        /// missing a password to use for authentication
        MissingPassword,
        /// given access was not permitted
        AccessNotPermitted,
        /// something went wrong with the given input/output stream.
        Io,
        /// received an invalid type that was expected
        InvalidType,
        /// generic bad request error, the message gives more context on why it is considered
        /// a bad request.
        BadRequest,
        /// missing a `Content-Type` header in your request
        MissingContentType,
        /// reached an unexpected EOF marker.
        ReachedUnexpectedEof,
        /// invalid input was given
        InvalidInput,
        /// unable to parse a path parameter.
        UnableToParsePathParameter,
        /// missing a required path parameter in the request.
        MissingPathParameter,
        /// received the wrong list of path parameters, this is usually a bug within charted
        /// itself.
        WrongParameters,
        /// the server had failed to validate the path parameter's content.
        ParsingFailedInPathParam,
        /// failed to parse query parameters specified in the uri of the request
        ParsingQueryParamsFailed,
        /// received JWT claim was not found or was invalid
        InvalidJwtClaim,
        /// was missing an `Authorization` header
        MissingAuthorizationHeader,
        /// password given was invalid
        InvalidPassword,
        /// received invalid authentication type
        InvalidAuthenticationType,
        /// received an invalid part in an Authorization header value
        InvalidAuthorizationParts,
        /// received an invalid JWT token
        InvalidSessionToken,
        /// Session already expired.
        SessionExpired,
        /// unknown session.
        UnknownSession,
        /// a refresh token is required in this request.
        RefreshTokenRequired,
        /// the `?per_page` query parameter is maxed out to 100
        MaxPerPageExceeded,
        /// while parsing through the JSON tree received, something went wrong
        InvalidJsonPayload,
        /// multipart field expected was not found
        UnknownMultipartField,
        /// incomplete field data given
        IncompleteMultipartFieldData,
        /// unable to completely read multipart header received
        ReadMultipartHeaderFailed,
        /// was unable to decode the `Content-Type` header in this request
        DecodeMultipartContentTypeFailed,
        /// missing a multipart boundry to parse
        MissingMultipartBoundary,
        /// expected `multipart/form-data`; received something else
        NoMultipartReceived,
        /// received incomplete multipart stream
        IncompleteMultipartStream,
        /// was unable to decode a header name in a multipart request
        DecodeMultipartHeaderNameFailed,
        /// exceeded the maximum amount to stream from
        StreamSizeExceeded,
        /// exceeded the maximum amount of fields to use
        MultipartFieldsSizeExceeded,
        /// received unknown error while reading the given stream
        MultipartStreamReadFailed,
        /// missing an expected multipart field in this request.
        MissingMultipartField,
        /// received an invalid multipart boundary
        InvalidMultipartBoundary,
    }
    impl utoipa::__dev::ComposeSchema for ErrorCode {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            utoipa::openapi::schema::Object::builder()
                .schema_type(
                    utoipa::openapi::schema::SchemaType::new(
                        utoipa::openapi::schema::Type::String,
                    ),
                )
                .enum_values::<
                    [&str; 55usize],
                    &str,
                >(
                    Some([
                        "SYSTEM_FAILURE",
                        "UNEXPECTED_E_O_F",
                        "REST_ENDPOINT_NOT_FOUND",
                        "INVALID_H_T_T_P_METHOD",
                        "ENTITY_NOT_FOUND",
                        "ENTITY_ALREADY_EXISTS",
                        "INTERNAL_SERVER_ERROR",
                        "VALIDATION_FAILED",
                        "INVALID_CONTENT_TYPE",
                        "INVALID_H_T_T_P_HEADER_NAME",
                        "INVALID_H_T_T_P_HEADER_VALUE",
                        "REQUIRES_SESSION_TOKEN",
                        "UNABLE_TO_DECODE_BASE64",
                        "UNABLE_TO_DECODE_ULID",
                        "INVALID_UTF8",
                        "INVALID_BODY",
                        "MISSING_HEADER",
                        "REGISTRATIONS_DISABLED",
                        "MISSING_PASSWORD",
                        "ACCESS_NOT_PERMITTED",
                        "IO",
                        "INVALID_TYPE",
                        "BAD_REQUEST",
                        "MISSING_CONTENT_TYPE",
                        "REACHED_UNEXPECTED_EOF",
                        "INVALID_INPUT",
                        "UNABLE_TO_PARSE_PATH_PARAMETER",
                        "MISSING_PATH_PARAMETER",
                        "WRONG_PARAMETERS",
                        "PARSING_FAILED_IN_PATH_PARAM",
                        "PARSING_QUERY_PARAMS_FAILED",
                        "INVALID_JWT_CLAIM",
                        "MISSING_AUTHORIZATION_HEADER",
                        "INVALID_PASSWORD",
                        "INVALID_AUTHENTICATION_TYPE",
                        "INVALID_AUTHORIZATION_PARTS",
                        "INVALID_SESSION_TOKEN",
                        "SESSION_EXPIRED",
                        "UNKNOWN_SESSION",
                        "REFRESH_TOKEN_REQUIRED",
                        "MAX_PER_PAGE_EXCEEDED",
                        "INVALID_JSON_PAYLOAD",
                        "UNKNOWN_MULTIPART_FIELD",
                        "INCOMPLETE_MULTIPART_FIELD_DATA",
                        "READ_MULTIPART_HEADER_FAILED",
                        "DECODE_MULTIPART_CONTENT_TYPE_FAILED",
                        "MISSING_MULTIPART_BOUNDARY",
                        "NO_MULTIPART_RECEIVED",
                        "INCOMPLETE_MULTIPART_STREAM",
                        "DECODE_MULTIPART_HEADER_NAME_FAILED",
                        "STREAM_SIZE_EXCEEDED",
                        "MULTIPART_FIELDS_SIZE_EXCEEDED",
                        "MULTIPART_STREAM_READ_FAILED",
                        "MISSING_MULTIPART_FIELD",
                        "INVALID_MULTIPART_BOUNDARY",
                    ]),
                )
                .description(
                    Some(
                        "Contextualized error code on why this request failed.\n\nThis field can be looked up from the documentation to give\na better representation of the error.",
                    ),
                )
                .into()
        }
    }
    impl utoipa::ToSchema for ErrorCode {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ErrorCode")
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
    impl ::core::fmt::Debug for ErrorCode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ErrorCode::SystemFailure => "SystemFailure",
                    ErrorCode::UnexpectedEOF => "UnexpectedEOF",
                    ErrorCode::RestEndpointNotFound => "RestEndpointNotFound",
                    ErrorCode::InvalidHTTPMethod => "InvalidHTTPMethod",
                    ErrorCode::EntityNotFound => "EntityNotFound",
                    ErrorCode::EntityAlreadyExists => "EntityAlreadyExists",
                    ErrorCode::InternalServerError => "InternalServerError",
                    ErrorCode::ValidationFailed => "ValidationFailed",
                    ErrorCode::InvalidContentType => "InvalidContentType",
                    ErrorCode::InvalidHTTPHeaderName => "InvalidHTTPHeaderName",
                    ErrorCode::InvalidHTTPHeaderValue => "InvalidHTTPHeaderValue",
                    ErrorCode::RequiresSessionToken => "RequiresSessionToken",
                    ErrorCode::UnableToDecodeBase64 => "UnableToDecodeBase64",
                    ErrorCode::UnableToDecodeUlid => "UnableToDecodeUlid",
                    ErrorCode::InvalidUtf8 => "InvalidUtf8",
                    ErrorCode::InvalidBody => "InvalidBody",
                    ErrorCode::MissingHeader => "MissingHeader",
                    ErrorCode::RegistrationsDisabled => "RegistrationsDisabled",
                    ErrorCode::MissingPassword => "MissingPassword",
                    ErrorCode::AccessNotPermitted => "AccessNotPermitted",
                    ErrorCode::Io => "Io",
                    ErrorCode::InvalidType => "InvalidType",
                    ErrorCode::BadRequest => "BadRequest",
                    ErrorCode::MissingContentType => "MissingContentType",
                    ErrorCode::ReachedUnexpectedEof => "ReachedUnexpectedEof",
                    ErrorCode::InvalidInput => "InvalidInput",
                    ErrorCode::UnableToParsePathParameter => "UnableToParsePathParameter",
                    ErrorCode::MissingPathParameter => "MissingPathParameter",
                    ErrorCode::WrongParameters => "WrongParameters",
                    ErrorCode::ParsingFailedInPathParam => "ParsingFailedInPathParam",
                    ErrorCode::ParsingQueryParamsFailed => "ParsingQueryParamsFailed",
                    ErrorCode::InvalidJwtClaim => "InvalidJwtClaim",
                    ErrorCode::MissingAuthorizationHeader => "MissingAuthorizationHeader",
                    ErrorCode::InvalidPassword => "InvalidPassword",
                    ErrorCode::InvalidAuthenticationType => "InvalidAuthenticationType",
                    ErrorCode::InvalidAuthorizationParts => "InvalidAuthorizationParts",
                    ErrorCode::InvalidSessionToken => "InvalidSessionToken",
                    ErrorCode::SessionExpired => "SessionExpired",
                    ErrorCode::UnknownSession => "UnknownSession",
                    ErrorCode::RefreshTokenRequired => "RefreshTokenRequired",
                    ErrorCode::MaxPerPageExceeded => "MaxPerPageExceeded",
                    ErrorCode::InvalidJsonPayload => "InvalidJsonPayload",
                    ErrorCode::UnknownMultipartField => "UnknownMultipartField",
                    ErrorCode::IncompleteMultipartFieldData => {
                        "IncompleteMultipartFieldData"
                    }
                    ErrorCode::ReadMultipartHeaderFailed => "ReadMultipartHeaderFailed",
                    ErrorCode::DecodeMultipartContentTypeFailed => {
                        "DecodeMultipartContentTypeFailed"
                    }
                    ErrorCode::MissingMultipartBoundary => "MissingMultipartBoundary",
                    ErrorCode::NoMultipartReceived => "NoMultipartReceived",
                    ErrorCode::IncompleteMultipartStream => "IncompleteMultipartStream",
                    ErrorCode::DecodeMultipartHeaderNameFailed => {
                        "DecodeMultipartHeaderNameFailed"
                    }
                    ErrorCode::StreamSizeExceeded => "StreamSizeExceeded",
                    ErrorCode::MultipartFieldsSizeExceeded => {
                        "MultipartFieldsSizeExceeded"
                    }
                    ErrorCode::MultipartStreamReadFailed => "MultipartStreamReadFailed",
                    ErrorCode::MissingMultipartField => "MissingMultipartField",
                    ErrorCode::InvalidMultipartBoundary => "InvalidMultipartBoundary",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorCode {
        #[inline]
        fn clone(&self) -> ErrorCode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorCode {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ErrorCode {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    ErrorCode::SystemFailure => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            0u32,
                            "SYSTEM_FAILURE",
                        )
                    }
                    ErrorCode::UnexpectedEOF => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            1u32,
                            "UNEXPECTED_E_O_F",
                        )
                    }
                    ErrorCode::RestEndpointNotFound => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            2u32,
                            "REST_ENDPOINT_NOT_FOUND",
                        )
                    }
                    ErrorCode::InvalidHTTPMethod => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            3u32,
                            "INVALID_H_T_T_P_METHOD",
                        )
                    }
                    ErrorCode::EntityNotFound => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            4u32,
                            "ENTITY_NOT_FOUND",
                        )
                    }
                    ErrorCode::EntityAlreadyExists => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            5u32,
                            "ENTITY_ALREADY_EXISTS",
                        )
                    }
                    ErrorCode::InternalServerError => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            6u32,
                            "INTERNAL_SERVER_ERROR",
                        )
                    }
                    ErrorCode::ValidationFailed => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            7u32,
                            "VALIDATION_FAILED",
                        )
                    }
                    ErrorCode::InvalidContentType => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            8u32,
                            "INVALID_CONTENT_TYPE",
                        )
                    }
                    ErrorCode::InvalidHTTPHeaderName => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            9u32,
                            "INVALID_H_T_T_P_HEADER_NAME",
                        )
                    }
                    ErrorCode::InvalidHTTPHeaderValue => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            10u32,
                            "INVALID_H_T_T_P_HEADER_VALUE",
                        )
                    }
                    ErrorCode::RequiresSessionToken => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            11u32,
                            "REQUIRES_SESSION_TOKEN",
                        )
                    }
                    ErrorCode::UnableToDecodeBase64 => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            12u32,
                            "UNABLE_TO_DECODE_BASE64",
                        )
                    }
                    ErrorCode::UnableToDecodeUlid => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            13u32,
                            "UNABLE_TO_DECODE_ULID",
                        )
                    }
                    ErrorCode::InvalidUtf8 => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            14u32,
                            "INVALID_UTF8",
                        )
                    }
                    ErrorCode::InvalidBody => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            15u32,
                            "INVALID_BODY",
                        )
                    }
                    ErrorCode::MissingHeader => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            16u32,
                            "MISSING_HEADER",
                        )
                    }
                    ErrorCode::RegistrationsDisabled => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            17u32,
                            "REGISTRATIONS_DISABLED",
                        )
                    }
                    ErrorCode::MissingPassword => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            18u32,
                            "MISSING_PASSWORD",
                        )
                    }
                    ErrorCode::AccessNotPermitted => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            19u32,
                            "ACCESS_NOT_PERMITTED",
                        )
                    }
                    ErrorCode::Io => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            20u32,
                            "IO",
                        )
                    }
                    ErrorCode::InvalidType => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            21u32,
                            "INVALID_TYPE",
                        )
                    }
                    ErrorCode::BadRequest => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            22u32,
                            "BAD_REQUEST",
                        )
                    }
                    ErrorCode::MissingContentType => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            23u32,
                            "MISSING_CONTENT_TYPE",
                        )
                    }
                    ErrorCode::ReachedUnexpectedEof => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            24u32,
                            "REACHED_UNEXPECTED_EOF",
                        )
                    }
                    ErrorCode::InvalidInput => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            25u32,
                            "INVALID_INPUT",
                        )
                    }
                    ErrorCode::UnableToParsePathParameter => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            26u32,
                            "UNABLE_TO_PARSE_PATH_PARAMETER",
                        )
                    }
                    ErrorCode::MissingPathParameter => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            27u32,
                            "MISSING_PATH_PARAMETER",
                        )
                    }
                    ErrorCode::WrongParameters => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            28u32,
                            "WRONG_PARAMETERS",
                        )
                    }
                    ErrorCode::ParsingFailedInPathParam => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            29u32,
                            "PARSING_FAILED_IN_PATH_PARAM",
                        )
                    }
                    ErrorCode::ParsingQueryParamsFailed => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            30u32,
                            "PARSING_QUERY_PARAMS_FAILED",
                        )
                    }
                    ErrorCode::InvalidJwtClaim => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            31u32,
                            "INVALID_JWT_CLAIM",
                        )
                    }
                    ErrorCode::MissingAuthorizationHeader => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            32u32,
                            "MISSING_AUTHORIZATION_HEADER",
                        )
                    }
                    ErrorCode::InvalidPassword => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            33u32,
                            "INVALID_PASSWORD",
                        )
                    }
                    ErrorCode::InvalidAuthenticationType => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            34u32,
                            "INVALID_AUTHENTICATION_TYPE",
                        )
                    }
                    ErrorCode::InvalidAuthorizationParts => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            35u32,
                            "INVALID_AUTHORIZATION_PARTS",
                        )
                    }
                    ErrorCode::InvalidSessionToken => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            36u32,
                            "INVALID_SESSION_TOKEN",
                        )
                    }
                    ErrorCode::SessionExpired => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            37u32,
                            "SESSION_EXPIRED",
                        )
                    }
                    ErrorCode::UnknownSession => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            38u32,
                            "UNKNOWN_SESSION",
                        )
                    }
                    ErrorCode::RefreshTokenRequired => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            39u32,
                            "REFRESH_TOKEN_REQUIRED",
                        )
                    }
                    ErrorCode::MaxPerPageExceeded => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            40u32,
                            "MAX_PER_PAGE_EXCEEDED",
                        )
                    }
                    ErrorCode::InvalidJsonPayload => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            41u32,
                            "INVALID_JSON_PAYLOAD",
                        )
                    }
                    ErrorCode::UnknownMultipartField => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            42u32,
                            "UNKNOWN_MULTIPART_FIELD",
                        )
                    }
                    ErrorCode::IncompleteMultipartFieldData => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            43u32,
                            "INCOMPLETE_MULTIPART_FIELD_DATA",
                        )
                    }
                    ErrorCode::ReadMultipartHeaderFailed => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            44u32,
                            "READ_MULTIPART_HEADER_FAILED",
                        )
                    }
                    ErrorCode::DecodeMultipartContentTypeFailed => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            45u32,
                            "DECODE_MULTIPART_CONTENT_TYPE_FAILED",
                        )
                    }
                    ErrorCode::MissingMultipartBoundary => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            46u32,
                            "MISSING_MULTIPART_BOUNDARY",
                        )
                    }
                    ErrorCode::NoMultipartReceived => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            47u32,
                            "NO_MULTIPART_RECEIVED",
                        )
                    }
                    ErrorCode::IncompleteMultipartStream => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            48u32,
                            "INCOMPLETE_MULTIPART_STREAM",
                        )
                    }
                    ErrorCode::DecodeMultipartHeaderNameFailed => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            49u32,
                            "DECODE_MULTIPART_HEADER_NAME_FAILED",
                        )
                    }
                    ErrorCode::StreamSizeExceeded => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            50u32,
                            "STREAM_SIZE_EXCEEDED",
                        )
                    }
                    ErrorCode::MultipartFieldsSizeExceeded => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            51u32,
                            "MULTIPART_FIELDS_SIZE_EXCEEDED",
                        )
                    }
                    ErrorCode::MultipartStreamReadFailed => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            52u32,
                            "MULTIPART_STREAM_READ_FAILED",
                        )
                    }
                    ErrorCode::MissingMultipartField => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            53u32,
                            "MISSING_MULTIPART_FIELD",
                        )
                    }
                    ErrorCode::InvalidMultipartBoundary => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "ErrorCode",
                            54u32,
                            "INVALID_MULTIPART_BOUNDARY",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ErrorCode {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
                    __field17,
                    __field18,
                    __field19,
                    __field20,
                    __field21,
                    __field22,
                    __field23,
                    __field24,
                    __field25,
                    __field26,
                    __field27,
                    __field28,
                    __field29,
                    __field30,
                    __field31,
                    __field32,
                    __field33,
                    __field34,
                    __field35,
                    __field36,
                    __field37,
                    __field38,
                    __field39,
                    __field40,
                    __field41,
                    __field42,
                    __field43,
                    __field44,
                    __field45,
                    __field46,
                    __field47,
                    __field48,
                    __field49,
                    __field50,
                    __field51,
                    __field52,
                    __field53,
                    __field54,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            13u64 => _serde::__private::Ok(__Field::__field13),
                            14u64 => _serde::__private::Ok(__Field::__field14),
                            15u64 => _serde::__private::Ok(__Field::__field15),
                            16u64 => _serde::__private::Ok(__Field::__field16),
                            17u64 => _serde::__private::Ok(__Field::__field17),
                            18u64 => _serde::__private::Ok(__Field::__field18),
                            19u64 => _serde::__private::Ok(__Field::__field19),
                            20u64 => _serde::__private::Ok(__Field::__field20),
                            21u64 => _serde::__private::Ok(__Field::__field21),
                            22u64 => _serde::__private::Ok(__Field::__field22),
                            23u64 => _serde::__private::Ok(__Field::__field23),
                            24u64 => _serde::__private::Ok(__Field::__field24),
                            25u64 => _serde::__private::Ok(__Field::__field25),
                            26u64 => _serde::__private::Ok(__Field::__field26),
                            27u64 => _serde::__private::Ok(__Field::__field27),
                            28u64 => _serde::__private::Ok(__Field::__field28),
                            29u64 => _serde::__private::Ok(__Field::__field29),
                            30u64 => _serde::__private::Ok(__Field::__field30),
                            31u64 => _serde::__private::Ok(__Field::__field31),
                            32u64 => _serde::__private::Ok(__Field::__field32),
                            33u64 => _serde::__private::Ok(__Field::__field33),
                            34u64 => _serde::__private::Ok(__Field::__field34),
                            35u64 => _serde::__private::Ok(__Field::__field35),
                            36u64 => _serde::__private::Ok(__Field::__field36),
                            37u64 => _serde::__private::Ok(__Field::__field37),
                            38u64 => _serde::__private::Ok(__Field::__field38),
                            39u64 => _serde::__private::Ok(__Field::__field39),
                            40u64 => _serde::__private::Ok(__Field::__field40),
                            41u64 => _serde::__private::Ok(__Field::__field41),
                            42u64 => _serde::__private::Ok(__Field::__field42),
                            43u64 => _serde::__private::Ok(__Field::__field43),
                            44u64 => _serde::__private::Ok(__Field::__field44),
                            45u64 => _serde::__private::Ok(__Field::__field45),
                            46u64 => _serde::__private::Ok(__Field::__field46),
                            47u64 => _serde::__private::Ok(__Field::__field47),
                            48u64 => _serde::__private::Ok(__Field::__field48),
                            49u64 => _serde::__private::Ok(__Field::__field49),
                            50u64 => _serde::__private::Ok(__Field::__field50),
                            51u64 => _serde::__private::Ok(__Field::__field51),
                            52u64 => _serde::__private::Ok(__Field::__field52),
                            53u64 => _serde::__private::Ok(__Field::__field53),
                            54u64 => _serde::__private::Ok(__Field::__field54),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 55",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "SYSTEM_FAILURE" => _serde::__private::Ok(__Field::__field0),
                            "UNEXPECTED_E_O_F" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            "REST_ENDPOINT_NOT_FOUND" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            "INVALID_H_T_T_P_METHOD" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            "ENTITY_NOT_FOUND" => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            "ENTITY_ALREADY_EXISTS" => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            "INTERNAL_SERVER_ERROR" => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            "VALIDATION_FAILED" => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            "INVALID_CONTENT_TYPE" => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            "INVALID_H_T_T_P_HEADER_NAME" => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            "INVALID_H_T_T_P_HEADER_VALUE" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            "REQUIRES_SESSION_TOKEN" => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            "UNABLE_TO_DECODE_BASE64" => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            "UNABLE_TO_DECODE_ULID" => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            "INVALID_UTF8" => _serde::__private::Ok(__Field::__field14),
                            "INVALID_BODY" => _serde::__private::Ok(__Field::__field15),
                            "MISSING_HEADER" => _serde::__private::Ok(__Field::__field16),
                            "REGISTRATIONS_DISABLED" => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            "MISSING_PASSWORD" => {
                                _serde::__private::Ok(__Field::__field18)
                            }
                            "ACCESS_NOT_PERMITTED" => {
                                _serde::__private::Ok(__Field::__field19)
                            }
                            "IO" => _serde::__private::Ok(__Field::__field20),
                            "INVALID_TYPE" => _serde::__private::Ok(__Field::__field21),
                            "BAD_REQUEST" => _serde::__private::Ok(__Field::__field22),
                            "MISSING_CONTENT_TYPE" => {
                                _serde::__private::Ok(__Field::__field23)
                            }
                            "REACHED_UNEXPECTED_EOF" => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            "INVALID_INPUT" => _serde::__private::Ok(__Field::__field25),
                            "UNABLE_TO_PARSE_PATH_PARAMETER" => {
                                _serde::__private::Ok(__Field::__field26)
                            }
                            "MISSING_PATH_PARAMETER" => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            "WRONG_PARAMETERS" => {
                                _serde::__private::Ok(__Field::__field28)
                            }
                            "PARSING_FAILED_IN_PATH_PARAM" => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            "PARSING_QUERY_PARAMS_FAILED" => {
                                _serde::__private::Ok(__Field::__field30)
                            }
                            "INVALID_JWT_CLAIM" => {
                                _serde::__private::Ok(__Field::__field31)
                            }
                            "MISSING_AUTHORIZATION_HEADER" => {
                                _serde::__private::Ok(__Field::__field32)
                            }
                            "INVALID_PASSWORD" => {
                                _serde::__private::Ok(__Field::__field33)
                            }
                            "INVALID_AUTHENTICATION_TYPE" => {
                                _serde::__private::Ok(__Field::__field34)
                            }
                            "INVALID_AUTHORIZATION_PARTS" => {
                                _serde::__private::Ok(__Field::__field35)
                            }
                            "INVALID_SESSION_TOKEN" => {
                                _serde::__private::Ok(__Field::__field36)
                            }
                            "SESSION_EXPIRED" => {
                                _serde::__private::Ok(__Field::__field37)
                            }
                            "UNKNOWN_SESSION" => {
                                _serde::__private::Ok(__Field::__field38)
                            }
                            "REFRESH_TOKEN_REQUIRED" => {
                                _serde::__private::Ok(__Field::__field39)
                            }
                            "MAX_PER_PAGE_EXCEEDED" => {
                                _serde::__private::Ok(__Field::__field40)
                            }
                            "INVALID_JSON_PAYLOAD" => {
                                _serde::__private::Ok(__Field::__field41)
                            }
                            "UNKNOWN_MULTIPART_FIELD" => {
                                _serde::__private::Ok(__Field::__field42)
                            }
                            "INCOMPLETE_MULTIPART_FIELD_DATA" => {
                                _serde::__private::Ok(__Field::__field43)
                            }
                            "READ_MULTIPART_HEADER_FAILED" => {
                                _serde::__private::Ok(__Field::__field44)
                            }
                            "DECODE_MULTIPART_CONTENT_TYPE_FAILED" => {
                                _serde::__private::Ok(__Field::__field45)
                            }
                            "MISSING_MULTIPART_BOUNDARY" => {
                                _serde::__private::Ok(__Field::__field46)
                            }
                            "NO_MULTIPART_RECEIVED" => {
                                _serde::__private::Ok(__Field::__field47)
                            }
                            "INCOMPLETE_MULTIPART_STREAM" => {
                                _serde::__private::Ok(__Field::__field48)
                            }
                            "DECODE_MULTIPART_HEADER_NAME_FAILED" => {
                                _serde::__private::Ok(__Field::__field49)
                            }
                            "STREAM_SIZE_EXCEEDED" => {
                                _serde::__private::Ok(__Field::__field50)
                            }
                            "MULTIPART_FIELDS_SIZE_EXCEEDED" => {
                                _serde::__private::Ok(__Field::__field51)
                            }
                            "MULTIPART_STREAM_READ_FAILED" => {
                                _serde::__private::Ok(__Field::__field52)
                            }
                            "MISSING_MULTIPART_FIELD" => {
                                _serde::__private::Ok(__Field::__field53)
                            }
                            "INVALID_MULTIPART_BOUNDARY" => {
                                _serde::__private::Ok(__Field::__field54)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"SYSTEM_FAILURE" => _serde::__private::Ok(__Field::__field0),
                            b"UNEXPECTED_E_O_F" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            b"REST_ENDPOINT_NOT_FOUND" => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            b"INVALID_H_T_T_P_METHOD" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            b"ENTITY_NOT_FOUND" => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            b"ENTITY_ALREADY_EXISTS" => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            b"INTERNAL_SERVER_ERROR" => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            b"VALIDATION_FAILED" => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            b"INVALID_CONTENT_TYPE" => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            b"INVALID_H_T_T_P_HEADER_NAME" => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            b"INVALID_H_T_T_P_HEADER_VALUE" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            b"REQUIRES_SESSION_TOKEN" => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            b"UNABLE_TO_DECODE_BASE64" => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            b"UNABLE_TO_DECODE_ULID" => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            b"INVALID_UTF8" => _serde::__private::Ok(__Field::__field14),
                            b"INVALID_BODY" => _serde::__private::Ok(__Field::__field15),
                            b"MISSING_HEADER" => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            b"REGISTRATIONS_DISABLED" => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            b"MISSING_PASSWORD" => {
                                _serde::__private::Ok(__Field::__field18)
                            }
                            b"ACCESS_NOT_PERMITTED" => {
                                _serde::__private::Ok(__Field::__field19)
                            }
                            b"IO" => _serde::__private::Ok(__Field::__field20),
                            b"INVALID_TYPE" => _serde::__private::Ok(__Field::__field21),
                            b"BAD_REQUEST" => _serde::__private::Ok(__Field::__field22),
                            b"MISSING_CONTENT_TYPE" => {
                                _serde::__private::Ok(__Field::__field23)
                            }
                            b"REACHED_UNEXPECTED_EOF" => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            b"INVALID_INPUT" => _serde::__private::Ok(__Field::__field25),
                            b"UNABLE_TO_PARSE_PATH_PARAMETER" => {
                                _serde::__private::Ok(__Field::__field26)
                            }
                            b"MISSING_PATH_PARAMETER" => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            b"WRONG_PARAMETERS" => {
                                _serde::__private::Ok(__Field::__field28)
                            }
                            b"PARSING_FAILED_IN_PATH_PARAM" => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            b"PARSING_QUERY_PARAMS_FAILED" => {
                                _serde::__private::Ok(__Field::__field30)
                            }
                            b"INVALID_JWT_CLAIM" => {
                                _serde::__private::Ok(__Field::__field31)
                            }
                            b"MISSING_AUTHORIZATION_HEADER" => {
                                _serde::__private::Ok(__Field::__field32)
                            }
                            b"INVALID_PASSWORD" => {
                                _serde::__private::Ok(__Field::__field33)
                            }
                            b"INVALID_AUTHENTICATION_TYPE" => {
                                _serde::__private::Ok(__Field::__field34)
                            }
                            b"INVALID_AUTHORIZATION_PARTS" => {
                                _serde::__private::Ok(__Field::__field35)
                            }
                            b"INVALID_SESSION_TOKEN" => {
                                _serde::__private::Ok(__Field::__field36)
                            }
                            b"SESSION_EXPIRED" => {
                                _serde::__private::Ok(__Field::__field37)
                            }
                            b"UNKNOWN_SESSION" => {
                                _serde::__private::Ok(__Field::__field38)
                            }
                            b"REFRESH_TOKEN_REQUIRED" => {
                                _serde::__private::Ok(__Field::__field39)
                            }
                            b"MAX_PER_PAGE_EXCEEDED" => {
                                _serde::__private::Ok(__Field::__field40)
                            }
                            b"INVALID_JSON_PAYLOAD" => {
                                _serde::__private::Ok(__Field::__field41)
                            }
                            b"UNKNOWN_MULTIPART_FIELD" => {
                                _serde::__private::Ok(__Field::__field42)
                            }
                            b"INCOMPLETE_MULTIPART_FIELD_DATA" => {
                                _serde::__private::Ok(__Field::__field43)
                            }
                            b"READ_MULTIPART_HEADER_FAILED" => {
                                _serde::__private::Ok(__Field::__field44)
                            }
                            b"DECODE_MULTIPART_CONTENT_TYPE_FAILED" => {
                                _serde::__private::Ok(__Field::__field45)
                            }
                            b"MISSING_MULTIPART_BOUNDARY" => {
                                _serde::__private::Ok(__Field::__field46)
                            }
                            b"NO_MULTIPART_RECEIVED" => {
                                _serde::__private::Ok(__Field::__field47)
                            }
                            b"INCOMPLETE_MULTIPART_STREAM" => {
                                _serde::__private::Ok(__Field::__field48)
                            }
                            b"DECODE_MULTIPART_HEADER_NAME_FAILED" => {
                                _serde::__private::Ok(__Field::__field49)
                            }
                            b"STREAM_SIZE_EXCEEDED" => {
                                _serde::__private::Ok(__Field::__field50)
                            }
                            b"MULTIPART_FIELDS_SIZE_EXCEEDED" => {
                                _serde::__private::Ok(__Field::__field51)
                            }
                            b"MULTIPART_STREAM_READ_FAILED" => {
                                _serde::__private::Ok(__Field::__field52)
                            }
                            b"MISSING_MULTIPART_FIELD" => {
                                _serde::__private::Ok(__Field::__field53)
                            }
                            b"INVALID_MULTIPART_BOUNDARY" => {
                                _serde::__private::Ok(__Field::__field54)
                            }
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ErrorCode>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ErrorCode;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum ErrorCode",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::SystemFailure)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::UnexpectedEOF)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::RestEndpointNotFound)
                            }
                            (__Field::__field3, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidHTTPMethod)
                            }
                            (__Field::__field4, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::EntityNotFound)
                            }
                            (__Field::__field5, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::EntityAlreadyExists)
                            }
                            (__Field::__field6, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InternalServerError)
                            }
                            (__Field::__field7, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::ValidationFailed)
                            }
                            (__Field::__field8, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidContentType)
                            }
                            (__Field::__field9, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidHTTPHeaderName)
                            }
                            (__Field::__field10, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidHTTPHeaderValue)
                            }
                            (__Field::__field11, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::RequiresSessionToken)
                            }
                            (__Field::__field12, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::UnableToDecodeBase64)
                            }
                            (__Field::__field13, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::UnableToDecodeUlid)
                            }
                            (__Field::__field14, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidUtf8)
                            }
                            (__Field::__field15, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidBody)
                            }
                            (__Field::__field16, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MissingHeader)
                            }
                            (__Field::__field17, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::RegistrationsDisabled)
                            }
                            (__Field::__field18, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MissingPassword)
                            }
                            (__Field::__field19, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::AccessNotPermitted)
                            }
                            (__Field::__field20, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::Io)
                            }
                            (__Field::__field21, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidType)
                            }
                            (__Field::__field22, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::BadRequest)
                            }
                            (__Field::__field23, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MissingContentType)
                            }
                            (__Field::__field24, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::ReachedUnexpectedEof)
                            }
                            (__Field::__field25, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidInput)
                            }
                            (__Field::__field26, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::UnableToParsePathParameter)
                            }
                            (__Field::__field27, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MissingPathParameter)
                            }
                            (__Field::__field28, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::WrongParameters)
                            }
                            (__Field::__field29, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::ParsingFailedInPathParam)
                            }
                            (__Field::__field30, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::ParsingQueryParamsFailed)
                            }
                            (__Field::__field31, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidJwtClaim)
                            }
                            (__Field::__field32, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MissingAuthorizationHeader)
                            }
                            (__Field::__field33, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidPassword)
                            }
                            (__Field::__field34, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidAuthenticationType)
                            }
                            (__Field::__field35, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidAuthorizationParts)
                            }
                            (__Field::__field36, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidSessionToken)
                            }
                            (__Field::__field37, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::SessionExpired)
                            }
                            (__Field::__field38, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::UnknownSession)
                            }
                            (__Field::__field39, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::RefreshTokenRequired)
                            }
                            (__Field::__field40, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MaxPerPageExceeded)
                            }
                            (__Field::__field41, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidJsonPayload)
                            }
                            (__Field::__field42, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::UnknownMultipartField)
                            }
                            (__Field::__field43, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    ErrorCode::IncompleteMultipartFieldData,
                                )
                            }
                            (__Field::__field44, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::ReadMultipartHeaderFailed)
                            }
                            (__Field::__field45, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    ErrorCode::DecodeMultipartContentTypeFailed,
                                )
                            }
                            (__Field::__field46, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MissingMultipartBoundary)
                            }
                            (__Field::__field47, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::NoMultipartReceived)
                            }
                            (__Field::__field48, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::IncompleteMultipartStream)
                            }
                            (__Field::__field49, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    ErrorCode::DecodeMultipartHeaderNameFailed,
                                )
                            }
                            (__Field::__field50, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::StreamSizeExceeded)
                            }
                            (__Field::__field51, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    ErrorCode::MultipartFieldsSizeExceeded,
                                )
                            }
                            (__Field::__field52, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MultipartStreamReadFailed)
                            }
                            (__Field::__field53, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::MissingMultipartField)
                            }
                            (__Field::__field54, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(ErrorCode::InvalidMultipartBoundary)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "SYSTEM_FAILURE",
                    "UNEXPECTED_E_O_F",
                    "REST_ENDPOINT_NOT_FOUND",
                    "INVALID_H_T_T_P_METHOD",
                    "ENTITY_NOT_FOUND",
                    "ENTITY_ALREADY_EXISTS",
                    "INTERNAL_SERVER_ERROR",
                    "VALIDATION_FAILED",
                    "INVALID_CONTENT_TYPE",
                    "INVALID_H_T_T_P_HEADER_NAME",
                    "INVALID_H_T_T_P_HEADER_VALUE",
                    "REQUIRES_SESSION_TOKEN",
                    "UNABLE_TO_DECODE_BASE64",
                    "UNABLE_TO_DECODE_ULID",
                    "INVALID_UTF8",
                    "INVALID_BODY",
                    "MISSING_HEADER",
                    "REGISTRATIONS_DISABLED",
                    "MISSING_PASSWORD",
                    "ACCESS_NOT_PERMITTED",
                    "IO",
                    "INVALID_TYPE",
                    "BAD_REQUEST",
                    "MISSING_CONTENT_TYPE",
                    "REACHED_UNEXPECTED_EOF",
                    "INVALID_INPUT",
                    "UNABLE_TO_PARSE_PATH_PARAMETER",
                    "MISSING_PATH_PARAMETER",
                    "WRONG_PARAMETERS",
                    "PARSING_FAILED_IN_PATH_PARAM",
                    "PARSING_QUERY_PARAMS_FAILED",
                    "INVALID_JWT_CLAIM",
                    "MISSING_AUTHORIZATION_HEADER",
                    "INVALID_PASSWORD",
                    "INVALID_AUTHENTICATION_TYPE",
                    "INVALID_AUTHORIZATION_PARTS",
                    "INVALID_SESSION_TOKEN",
                    "SESSION_EXPIRED",
                    "UNKNOWN_SESSION",
                    "REFRESH_TOKEN_REQUIRED",
                    "MAX_PER_PAGE_EXCEEDED",
                    "INVALID_JSON_PAYLOAD",
                    "UNKNOWN_MULTIPART_FIELD",
                    "INCOMPLETE_MULTIPART_FIELD_DATA",
                    "READ_MULTIPART_HEADER_FAILED",
                    "DECODE_MULTIPART_CONTENT_TYPE_FAILED",
                    "MISSING_MULTIPART_BOUNDARY",
                    "NO_MULTIPART_RECEIVED",
                    "INCOMPLETE_MULTIPART_STREAM",
                    "DECODE_MULTIPART_HEADER_NAME_FAILED",
                    "STREAM_SIZE_EXCEEDED",
                    "MULTIPART_FIELDS_SIZE_EXCEEDED",
                    "MULTIPART_STREAM_READ_FAILED",
                    "MISSING_MULTIPART_FIELD",
                    "INVALID_MULTIPART_BOUNDARY",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ErrorCode",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ErrorCode>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ErrorCode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ErrorCode {
        #[inline]
        fn eq(&self, other: &ErrorCode) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ErrorCode {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl From<(ErrorCode, &'static str)> for Error {
        fn from((code, message): (ErrorCode, &'static str)) -> Self {
            Error {
                code,
                message: Cow::Borrowed(message),
                details: None,
            }
        }
    }
    impl From<(ErrorCode, String)> for Error {
        fn from((code, message): (ErrorCode, String)) -> Self {
            Error {
                code,
                message: Cow::Owned(message),
                details: None,
            }
        }
    }
    impl From<(ErrorCode, String, Value)> for Error {
        fn from((code, message, details): (ErrorCode, String, Value)) -> Self {
            Error {
                code,
                message: Cow::Owned(message),
                details: Some(details),
            }
        }
    }
    impl From<(ErrorCode, String, Option<Value>)> for Error {
        fn from((code, message, details): (ErrorCode, String, Option<Value>)) -> Self {
            Error {
                code,
                message: Cow::Owned(message),
                details,
            }
        }
    }
    impl From<(ErrorCode, &'static str, Value)> for Error {
        fn from((code, message, details): (ErrorCode, &'static str, Value)) -> Self {
            Error {
                code,
                message: Cow::Borrowed(message),
                details: Some(details),
            }
        }
    }
    impl From<(ErrorCode, &'static str, Option<Value>)> for Error {
        fn from(
            (code, message, details): (ErrorCode, &'static str, Option<Value>),
        ) -> Self {
            Error {
                code,
                details,
                message: Cow::Borrowed(message),
            }
        }
    }
    impl From<(ErrorCode, Cow<'static, str>)> for Error {
        fn from((code, message): (ErrorCode, Cow<'static, str>)) -> Self {
            Error {
                code,
                details: None,
                message,
            }
        }
    }
    impl From<(ErrorCode, Cow<'static, str>, Value)> for Error {
        fn from(
            (code, message, details): (ErrorCode, Cow<'static, str>, Value),
        ) -> Self {
            Error {
                code,
                message,
                details: Some(details),
            }
        }
    }
    impl From<(ErrorCode, Cow<'static, str>, Option<Value>)> for Error {
        fn from(
            (code, message, details): (ErrorCode, Cow<'static, str>, Option<Value>),
        ) -> Self {
            Error { code, details, message }
        }
    }
    /// Return a successful API response.
    #[cfg(feature = "axum")]
    pub fn ok<T>(status: axum::http::StatusCode, data: T) -> Response<T> {
        Response {
            success: true,
            errors: Vec::new(),
            status,
            data: Some(data),
        }
    }
    /// Returns a empty HTTP API response.
    #[cfg(feature = "axum")]
    pub fn no_content() -> Response<()> {
        from_default(axum::http::StatusCode::NO_CONTENT)
    }
    /// Return a success HTTP API response from `T`'s [`Default`] implementation.
    #[cfg(feature = "axum")]
    pub fn from_default<T: Default>(status: axum::http::StatusCode) -> Response<T> {
        ok(status, T::default())
    }
    /// Returns a failed HTTP API response.
    #[cfg(feature = "axum")]
    pub fn err<E: Into<Error>>(status: axum::http::StatusCode, error: E) -> Response {
        let error = error.into();
        Response {
            success: false,
            errors: <[_]>::into_vec(::alloc::boxed::box_new([error])),
            status,
            data: None,
        }
    }
    /// Propagate a HTTP API response as a internal server error.
    #[cfg(feature = "axum")]
    pub fn internal_server_error() -> Response {
        err(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            (ErrorCode::InternalServerError, "Internal Server Error"),
        )
    }
    /// Propagate a system failure response from [`eyre::Report`].
    #[cfg(feature = "axum")]
    pub fn system_failure_from_report(report: eyre::Report) -> Response {
        struct AError<'a>(&'a dyn std::error::Error);
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for AError<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "AError", &&self.0)
            }
        }
        impl std::fmt::Display for AError<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl std::error::Error for AError<'_> {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.0.source()
            }
        }
        system_failure(AError(report.as_ref()))
    }
    /// Propagate a system failure response.
    #[cfg(feature = "axum")]
    pub fn system_failure<E: std::error::Error>(error: E) -> Response {
        if true {
            let mut errors = Vec::new();
            for err in error.source().iter().take(5) {
                errors.push(Value::String(err.to_string()));
            }
            return err(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                (
                    ErrorCode::SystemFailure,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("system failure occurred: {0}", error),
                        );
                        res
                    }),
                    Some(Value::Array(errors)),
                ),
            );
        }
        err(
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            (ErrorCode::SystemFailure, "system failure occurred", None),
        )
    }
}
pub mod bitflags {
    mod apikeyscope {
        use super::Bitflags;
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        pub type ApiKeyScopes = crate::bitflags::Bitfield<ApiKeyScope>;
        #[allow(clippy::enum_clike_unportable_variant)]
        #[repr(u64)]
        pub enum ApiKeyScope {
            UserAccess = 1u64 << 0u64,
            UserUpdate = 1u64 << 1u64,
            UserDelete = 1u64 << 2u64,
            UserConnections = 1u64 << 3u64,
            UserAvatarUpdate = 1u64 << 4u64,
            UserSessionsList = 1u64 << 5u64,
            RepoAccess = 1u64 << 6u64,
            RepoCreate = 1u64 << 7u64,
            RepoDelete = 1u64 << 8u64,
            RepoUpdate = 1u64 << 9u64,
            RepoIconUpdate = 1u64 << 10u64,
            RepoReleaseCreate = 1u64 << 11u64,
            RepoReleaseUpdate = 1u64 << 12u64,
            RepoReleaseDelete = 1u64 << 13u64,
            RepoMembersList = 1u64 << 14u64,
            RepoMemberUpdate = 1u64 << 15u64,
            RepoMemberKick = 1u64 << 16u64,
            RepoMemberInviteAccess = 1u64 << 17u64,
            RepoMemberInviteDelete = 1u64 << 18u64,
            RepoWebhookList = 1u64 << 19u64,
            RepoWebhookCreate = 1u64 << 20u64,
            RepoWebhookUpdate = 1u64 << 21u64,
            RepoWebhookDelete = 1u64 << 22u64,
            RepoWebhookEventAccess = 1u64 << 23u64,
            RepoWebhookEventDelete = 1u64 << 24u64,
            ApiKeyView = 1u64 << 25u64,
            ApiKeyList = 1u64 << 26u64,
            ApiKeyCreate = 1u64 << 27u64,
            ApiKeyDelete = 1u64 << 28u64,
            ApiKeyUpdate = 1u64 << 29u64,
            OrgAccess = 1u64 << 30u64,
            OrgCreate = 1u64 << 31u64,
            OrgUpdate = 1u64 << 32u64,
            OrgDelete = 1u64 << 33u64,
            OrgMemberInvites = 1u64 << 34u64,
            OrgMemberList = 1u64 << 35u64,
            OrgMemberKick = 1u64 << 36u64,
            OrgMemberUpdate = 1u64 << 37u64,
            OrgWebhookList = 1u64 << 38u64,
            OrgWebhookCreate = 1u64 << 39u64,
            OrgWebhookUpdate = 1u64 << 40u64,
            OrgWebhookDelete = 1u64 << 41u64,
            OrgWebhookEventList = 1u64 << 42u64,
            OrgWebhookEventDelete = 1u64 << 43u64,
            AdminStats = 1u64 << 44u64,
            AdminUserCreate = 1u64 << 45u64,
            AdminUserDelete = 1u64 << 46u64,
            AdminUserUpdate = 1u64 << 47u64,
            AdminOrgDelete = 1u64 << 48u64,
            AdminOrgUpdate = 1u64 << 49u64,
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::fmt::Debug for ApiKeyScope {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ApiKeyScope::UserAccess => "UserAccess",
                        ApiKeyScope::UserUpdate => "UserUpdate",
                        ApiKeyScope::UserDelete => "UserDelete",
                        ApiKeyScope::UserConnections => "UserConnections",
                        ApiKeyScope::UserAvatarUpdate => "UserAvatarUpdate",
                        ApiKeyScope::UserSessionsList => "UserSessionsList",
                        ApiKeyScope::RepoAccess => "RepoAccess",
                        ApiKeyScope::RepoCreate => "RepoCreate",
                        ApiKeyScope::RepoDelete => "RepoDelete",
                        ApiKeyScope::RepoUpdate => "RepoUpdate",
                        ApiKeyScope::RepoIconUpdate => "RepoIconUpdate",
                        ApiKeyScope::RepoReleaseCreate => "RepoReleaseCreate",
                        ApiKeyScope::RepoReleaseUpdate => "RepoReleaseUpdate",
                        ApiKeyScope::RepoReleaseDelete => "RepoReleaseDelete",
                        ApiKeyScope::RepoMembersList => "RepoMembersList",
                        ApiKeyScope::RepoMemberUpdate => "RepoMemberUpdate",
                        ApiKeyScope::RepoMemberKick => "RepoMemberKick",
                        ApiKeyScope::RepoMemberInviteAccess => "RepoMemberInviteAccess",
                        ApiKeyScope::RepoMemberInviteDelete => "RepoMemberInviteDelete",
                        ApiKeyScope::RepoWebhookList => "RepoWebhookList",
                        ApiKeyScope::RepoWebhookCreate => "RepoWebhookCreate",
                        ApiKeyScope::RepoWebhookUpdate => "RepoWebhookUpdate",
                        ApiKeyScope::RepoWebhookDelete => "RepoWebhookDelete",
                        ApiKeyScope::RepoWebhookEventAccess => "RepoWebhookEventAccess",
                        ApiKeyScope::RepoWebhookEventDelete => "RepoWebhookEventDelete",
                        ApiKeyScope::ApiKeyView => "ApiKeyView",
                        ApiKeyScope::ApiKeyList => "ApiKeyList",
                        ApiKeyScope::ApiKeyCreate => "ApiKeyCreate",
                        ApiKeyScope::ApiKeyDelete => "ApiKeyDelete",
                        ApiKeyScope::ApiKeyUpdate => "ApiKeyUpdate",
                        ApiKeyScope::OrgAccess => "OrgAccess",
                        ApiKeyScope::OrgCreate => "OrgCreate",
                        ApiKeyScope::OrgUpdate => "OrgUpdate",
                        ApiKeyScope::OrgDelete => "OrgDelete",
                        ApiKeyScope::OrgMemberInvites => "OrgMemberInvites",
                        ApiKeyScope::OrgMemberList => "OrgMemberList",
                        ApiKeyScope::OrgMemberKick => "OrgMemberKick",
                        ApiKeyScope::OrgMemberUpdate => "OrgMemberUpdate",
                        ApiKeyScope::OrgWebhookList => "OrgWebhookList",
                        ApiKeyScope::OrgWebhookCreate => "OrgWebhookCreate",
                        ApiKeyScope::OrgWebhookUpdate => "OrgWebhookUpdate",
                        ApiKeyScope::OrgWebhookDelete => "OrgWebhookDelete",
                        ApiKeyScope::OrgWebhookEventList => "OrgWebhookEventList",
                        ApiKeyScope::OrgWebhookEventDelete => "OrgWebhookEventDelete",
                        ApiKeyScope::AdminStats => "AdminStats",
                        ApiKeyScope::AdminUserCreate => "AdminUserCreate",
                        ApiKeyScope::AdminUserDelete => "AdminUserDelete",
                        ApiKeyScope::AdminUserUpdate => "AdminUserUpdate",
                        ApiKeyScope::AdminOrgDelete => "AdminOrgDelete",
                        ApiKeyScope::AdminOrgUpdate => "AdminOrgUpdate",
                    },
                )
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::clone::Clone for ApiKeyScope {
            #[inline]
            fn clone(&self) -> ApiKeyScope {
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::marker::Copy for ApiKeyScope {}
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::hash::Hash for ApiKeyScope {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::marker::StructuralPartialEq for ApiKeyScope {}
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::cmp::PartialEq for ApiKeyScope {
            #[inline]
            fn eq(&self, other: &ApiKeyScope) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::cmp::Eq for ApiKeyScope {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::cmp::PartialOrd for ApiKeyScope {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ApiKeyScope,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::cmp::Ord for ApiKeyScope {
            #[inline]
            fn cmp(&self, other: &ApiKeyScope) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
            }
        }
        impl ApiKeyScope {
            pub const fn as_bit(self) -> u64 {
                self as u64
            }
        }
        impl ::std::fmt::Display for ApiKeyScope {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}", self.as_bit()))
            }
        }
        impl ::core::convert::From<ApiKeyScope> for u64 {
            fn from(value: ApiKeyScope) -> u64 {
                value as u64
            }
        }
        impl crate::bitflags::Bitflags for ApiKeyScope {
            type Bit = u64;
            const ZERO: Self::Bit = 0;
            #[inline]
            fn flags() -> ::std::collections::BTreeMap<&'static str, u64> {
                let mut map = ::std::collections::BTreeMap::new();
                map.insert("user:access", 1u64 << 0u64);
                map.insert("user:update", 1u64 << 1u64);
                map.insert("user:delete", 1u64 << 2u64);
                map.insert("user:connections", 1u64 << 3u64);
                map.insert("user:avatar:update", 1u64 << 4u64);
                map.insert("user:sessions:list", 1u64 << 5u64);
                map.insert("repo:access", 1u64 << 6u64);
                map.insert("repo:create", 1u64 << 7u64);
                map.insert("repo:delete", 1u64 << 8u64);
                map.insert("repo:update", 1u64 << 9u64);
                map.insert("repo:icon:update", 1u64 << 10u64);
                map.insert("repo:releases:create", 1u64 << 11u64);
                map.insert("repo:releases:update", 1u64 << 12u64);
                map.insert("repo:releases:delete", 1u64 << 13u64);
                map.insert("repo:members:list", 1u64 << 14u64);
                map.insert("repo:members:update", 1u64 << 15u64);
                map.insert("repo:members:kick", 1u64 << 16u64);
                map.insert("repo:members:invites:access", 1u64 << 17u64);
                map.insert("repo:members:invites:delete", 1u64 << 18u64);
                map.insert("repo:webhooks:list", 1u64 << 19u64);
                map.insert("repo:webhooks:create", 1u64 << 20u64);
                map.insert("repo:webhooks:update", 1u64 << 21u64);
                map.insert("repo:webhooks:delete", 1u64 << 22u64);
                map.insert("repo:webhooks:events:access", 1u64 << 23u64);
                map.insert("repo:webhooks:events:delete", 1u64 << 24u64);
                map.insert("apikeys:view", 1u64 << 25u64);
                map.insert("apikeys:list", 1u64 << 26u64);
                map.insert("apikeys:create", 1u64 << 27u64);
                map.insert("apikeys:delete", 1u64 << 28u64);
                map.insert("apikeys:update", 1u64 << 29u64);
                map.insert("org:access", 1u64 << 30u64);
                map.insert("org:create", 1u64 << 31u64);
                map.insert("org:update", 1u64 << 32u64);
                map.insert("org:delete", 1u64 << 33u64);
                map.insert("org:members:invites", 1u64 << 34u64);
                map.insert("org:members:list", 1u64 << 35u64);
                map.insert("org:members:kick", 1u64 << 36u64);
                map.insert("org:members:update", 1u64 << 37u64);
                map.insert("org:webhooks:list", 1u64 << 38u64);
                map.insert("org:webhooks:create", 1u64 << 39u64);
                map.insert("org:webhooks:update", 1u64 << 40u64);
                map.insert("org:webhooks:delete", 1u64 << 41u64);
                map.insert("org:webhooks:events:list", 1u64 << 42u64);
                map.insert("org:webhooks:events:delete", 1u64 << 43u64);
                map.insert("admin:stats", 1u64 << 44u64);
                map.insert("admin:users:create", 1u64 << 45u64);
                map.insert("admin:users:delete", 1u64 << 46u64);
                map.insert("admin:users:update", 1u64 << 47u64);
                map.insert("admin:orgs:delete", 1u64 << 48u64);
                map.insert("admin:orgs:update", 1u64 << 49u64);
                map
            }
            fn values<'v>() -> &'v [u64] {
                &[
                    1u64 << 0u64,
                    1u64 << 1u64,
                    1u64 << 2u64,
                    1u64 << 3u64,
                    1u64 << 4u64,
                    1u64 << 5u64,
                    1u64 << 6u64,
                    1u64 << 7u64,
                    1u64 << 8u64,
                    1u64 << 9u64,
                    1u64 << 10u64,
                    1u64 << 11u64,
                    1u64 << 12u64,
                    1u64 << 13u64,
                    1u64 << 14u64,
                    1u64 << 15u64,
                    1u64 << 16u64,
                    1u64 << 17u64,
                    1u64 << 18u64,
                    1u64 << 19u64,
                    1u64 << 20u64,
                    1u64 << 21u64,
                    1u64 << 22u64,
                    1u64 << 23u64,
                    1u64 << 24u64,
                    1u64 << 25u64,
                    1u64 << 26u64,
                    1u64 << 27u64,
                    1u64 << 28u64,
                    1u64 << 29u64,
                    1u64 << 30u64,
                    1u64 << 31u64,
                    1u64 << 32u64,
                    1u64 << 33u64,
                    1u64 << 34u64,
                    1u64 << 35u64,
                    1u64 << 36u64,
                    1u64 << 37u64,
                    1u64 << 38u64,
                    1u64 << 39u64,
                    1u64 << 40u64,
                    1u64 << 41u64,
                    1u64 << 42u64,
                    1u64 << 43u64,
                    1u64 << 44u64,
                    1u64 << 45u64,
                    1u64 << 46u64,
                    1u64 << 47u64,
                    1u64 << 48u64,
                    1u64 << 49u64,
                ]
            }
        }
        impl ::core::cmp::PartialEq<u64> for ApiKeyScope {
            fn eq(&self, other: &u64) -> bool {
                (*self as u64) == *other
            }
        }
        impl Serialize for ApiKeyScope {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for ApiKeyScope {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                use serde::de::Error;
                let flags = ApiKeyScope::flags();
                serde_untagged::UntaggedEnumVisitor::new()
                    .expecting("string or uint64")
                    .string(|v| {
                        if let Some(value) = flags.get(v).copied() {
                            return Ok(unsafe {
                                std::mem::transmute::<u64, ApiKeyScope>(value)
                            });
                        }
                        Err(
                            serde_untagged::de::Error::custom(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("unknown variant of ApiKeyScope: {0}", v),
                                    );
                                    res
                                }),
                            ),
                        )
                    })
                    .u64(|v| {
                        let max = <ApiKeyScope as Bitflags>::max();
                        if v >= 1 && v <= max {
                            return Ok(unsafe {
                                std::mem::transmute::<u64, ApiKeyScope>(v)
                            });
                        }
                        Err(
                            serde_untagged::de::Error::custom(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("out of range: [1..{0})", max),
                                    );
                                    res
                                }),
                            ),
                        )
                    })
                    .deserialize(deserializer)
            }
        }
        #[cfg(feature = "openapi")]
        const _: () = {
            use utoipa::{
                openapi::{
                    schema::SchemaType, KnownFormat, ObjectBuilder, OneOfBuilder, RefOr,
                    Schema, SchemaFormat, Type,
                },
                PartialSchema, ToSchema,
            };
            impl PartialSchema for ApiKeyScope {
                fn schema() -> RefOr<Schema> {
                    let flags = <ApiKeyScope as crate::bitflags::Bitflags>::flags();
                    let max = <ApiKeyScope as crate::bitflags::Bitflags>::max();
                    RefOr::T(
                        Schema::OneOf({
                            let oneof = OneOfBuilder::new()
                                .description(
                                    Some(
                                        "Representation of a API key scope. A scope determines a permission between an API key",
                                    ),
                                )
                                .item(
                                    Schema::Object({
                                        let object = ObjectBuilder::new()
                                            .schema_type(SchemaType::Type(Type::String))
                                            .description(
                                                Some(
                                                    "A humane name of the scope. This allows to determine the scope without knowing the integer representation of it.",
                                                ),
                                            )
                                            .enum_values(
                                                Some(flags.keys().copied().collect::<Vec<_>>()),
                                            );
                                        object.build()
                                    }),
                                )
                                .item(
                                    Schema::Object({
                                        let object = ObjectBuilder::new()
                                            .schema_type(SchemaType::Type(Type::Number))
                                            .format(
                                                Some(SchemaFormat::KnownFormat(KnownFormat::UInt64)),
                                            )
                                            .description(
                                                Some(
                                                    "The actual representation of the scope. This is the repsentation the server checks and stores as AND is used when comparing permissions",
                                                ),
                                            )
                                            .minimum(Some(1))
                                            .maximum(Some(max));
                                        object.build()
                                    }),
                                );
                            oneof.build()
                        }),
                    )
                }
            }
            impl ToSchema for ApiKeyScope {}
        };
        impl FromIterator<ApiKeyScope> for crate::bitflags::ApiKeyScopes {
            fn from_iter<T: IntoIterator<Item = ApiKeyScope>>(iter: T) -> Self {
                let mut bitfield = ApiKeyScopes::default();
                bitfield.add(iter);
                bitfield
            }
        }
    }
    mod member_permission {
        pub type MemberPermissions = crate::bitflags::Bitfield<MemberPermission>;
        #[allow(clippy::enum_clike_unportable_variant)]
        #[repr(u64)]
        pub enum MemberPermission {
            MemberInvite = 1u64 << 0u64,
            MemberUpdate = 1u64 << 1u64,
            MemberKick = 1u64 << 2u64,
            MetadataUpdate = 1u64 << 3u64,
            RepoCreate = 1u64 << 4u64,
            RepoDelete = 1u64 << 5u64,
            WebhookCreate = 1u64 << 6u64,
            WebhookUpdate = 1u64 << 7u64,
            WebhookDelete = 1u64 << 8u64,
            MetadataDelete = 1u64 << 9u64,
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::fmt::Debug for MemberPermission {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        MemberPermission::MemberInvite => "MemberInvite",
                        MemberPermission::MemberUpdate => "MemberUpdate",
                        MemberPermission::MemberKick => "MemberKick",
                        MemberPermission::MetadataUpdate => "MetadataUpdate",
                        MemberPermission::RepoCreate => "RepoCreate",
                        MemberPermission::RepoDelete => "RepoDelete",
                        MemberPermission::WebhookCreate => "WebhookCreate",
                        MemberPermission::WebhookUpdate => "WebhookUpdate",
                        MemberPermission::WebhookDelete => "WebhookDelete",
                        MemberPermission::MetadataDelete => "MetadataDelete",
                    },
                )
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::clone::Clone for MemberPermission {
            #[inline]
            fn clone(&self) -> MemberPermission {
                *self
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::marker::Copy for MemberPermission {}
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::hash::Hash for MemberPermission {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::marker::StructuralPartialEq for MemberPermission {}
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::cmp::PartialEq for MemberPermission {
            #[inline]
            fn eq(&self, other: &MemberPermission) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::cmp::Eq for MemberPermission {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::cmp::PartialOrd for MemberPermission {
            #[inline]
            fn partial_cmp(
                &self,
                other: &MemberPermission,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[automatically_derived]
        #[allow(clippy::enum_clike_unportable_variant)]
        impl ::core::cmp::Ord for MemberPermission {
            #[inline]
            fn cmp(&self, other: &MemberPermission) -> ::core::cmp::Ordering {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
            }
        }
        impl MemberPermission {
            pub const fn as_bit(self) -> u64 {
                self as u64
            }
        }
        impl ::std::fmt::Display for MemberPermission {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}", self.as_bit()))
            }
        }
        impl ::core::convert::From<MemberPermission> for u64 {
            fn from(value: MemberPermission) -> u64 {
                value as u64
            }
        }
        impl crate::bitflags::Bitflags for MemberPermission {
            type Bit = u64;
            const ZERO: Self::Bit = 0;
            #[inline]
            fn flags() -> ::std::collections::BTreeMap<&'static str, u64> {
                let mut map = ::std::collections::BTreeMap::new();
                map.insert("member:invite", 1u64 << 0u64);
                map.insert("member:update", 1u64 << 1u64);
                map.insert("member:kick", 1u64 << 2u64);
                map.insert("metadata:update", 1u64 << 3u64);
                map.insert("repo:create", 1u64 << 4u64);
                map.insert("repo:delete", 1u64 << 5u64);
                map.insert("webhooks:create", 1u64 << 6u64);
                map.insert("webhooks:update", 1u64 << 7u64);
                map.insert("webhooks:delete", 1u64 << 8u64);
                map.insert("metadata:delete", 1u64 << 9u64);
                map
            }
            fn values<'v>() -> &'v [u64] {
                &[
                    1u64 << 0u64,
                    1u64 << 1u64,
                    1u64 << 2u64,
                    1u64 << 3u64,
                    1u64 << 4u64,
                    1u64 << 5u64,
                    1u64 << 6u64,
                    1u64 << 7u64,
                    1u64 << 8u64,
                    1u64 << 9u64,
                ]
            }
        }
        impl ::core::cmp::PartialEq<u64> for MemberPermission {
            fn eq(&self, other: &u64) -> bool {
                (*self as u64) == *other
            }
        }
    }
    pub use apikeyscope::*;
    pub use member_permission::*;
    use std::{cmp::min, collections::BTreeMap, marker::PhantomData};
    /// Trait that implements the "scopes" concept.
    ///
    /// The "scopes" concept is similar to [Discord's Permissions] where essentially
    /// a **bitflags** represents a list of flags that will be serialized as a string
    /// which uses bit-wise operations to determine if a flag is included.
    ///
    /// [Discord's Permissions]: https://github.com/discord/discord-api-docs/blob/main/docs/topics/Permissions.md
    pub trait Bitflags: Sized + Send + Sync {
        /// Type representation of a single bit.
        type Bit: Copy;
        /// Constant that represents a zero of the type representation
        /// of this trait.
        const ZERO: Self::Bit;
        /// Returns a [`BTreeMap`] of all possible flags avaliable.
        fn flags() -> BTreeMap<&'static str, Self::Bit>;
        /// Returns a slice of all avaliable bits from `0..{flags.len()}`.
        fn values<'v>() -> &'v [Self::Bit];
        fn max() -> Self::Bit
        where
            Self::Bit: Ord,
        {
            Self::values().iter().max().copied().unwrap_or(Self::ZERO)
        }
    }
    /// Data structure that easily do computations with `F::Bit` easily.
    pub struct Bitfield<F: Bitflags>(F::Bit, PhantomData<F>);
    #[automatically_derived]
    impl<F: ::core::fmt::Debug + Bitflags> ::core::fmt::Debug for Bitfield<F>
    where
        F::Bit: ::core::fmt::Debug,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(
                f,
                "Bitfield",
                &self.0,
                &&self.1,
            )
        }
    }
    #[automatically_derived]
    impl<F: ::core::clone::Clone + Bitflags> ::core::clone::Clone for Bitfield<F>
    where
        F::Bit: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> Bitfield<F> {
            Bitfield(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl<F: ::core::marker::Copy + Bitflags> ::core::marker::Copy for Bitfield<F>
    where
        F::Bit: ::core::marker::Copy,
    {}
    impl<F: Bitflags> Bitfield<F> {
        /// Create a new [`Bitfield`] data structure.
        pub const fn new(value: F::Bit) -> Bitfield<F> {
            Bitfield(value, PhantomData)
        }
        /// Returns the current bit value stored in this [`Bitfield`].
        pub const fn value(&self) -> F::Bit {
            self.0
        }
    }
    impl<F: Bitflags<Bit = u64>> Bitfield<F> {
        /// Returns all the possible enabled bits in the bitfield to determine
        pub fn flags(&self) -> Vec<(&'static str, F::Bit)> {
            F::flags().into_iter().filter(|(_, bit)| self.contains(*bit)).collect()
        }
        /// Adds multiple bits to this [`Bitfield`] and updating the current
        /// value to what was acculumated.
        ///
        /// ## Example
        /// ```rust
        /// # use charted_core::{bitflags, bitflags::Bitfield};
        /// #
        /// # bitflags! {
        /// #     #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        /// #     #[allow(clippy::enum_clike_unportable_variant)]
        /// #     #[repr(u64)]
        /// #     pub Scope[u64] {
        /// #         Hello["hello"] => 1u64 << 0u64;
        /// #         World["world"] => 1u64 << 1u64;
        /// #     }
        /// # }
        /// #
        /// let mut bitfield = Bitfield::<Scope>::new(0);
        /// bitfield.add([Scope::Hello]);
        /// assert_eq!(bitfield.value(), 1);
        /// ```
        #[allow(clippy::should_implement_trait)]
        pub fn add<II: Into<F::Bit>, I: IntoIterator<Item = II>>(&mut self, values: I) {
            let iter = values.into_iter().map(Into::into);
            let new = iter
                .fold(
                    self.0,
                    |mut curr, elem: u64| {
                        if elem == u64::MAX {
                            return curr;
                        }
                        if elem > F::max() {
                            return curr;
                        }
                        curr |= elem;
                        curr
                    },
                );
            self.0 |= new;
        }
        /// Removed multiple bits to this [`Bitfield`] and updating the current
        /// value to what was acculumated.
        ///
        /// ## Example
        /// ```
        /// # use charted_core::{bitflags, bitflags::Bitfield};
        /// #
        /// # bitflags! {
        /// #     #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        /// #     #[allow(clippy::enum_clike_unportable_variant)]
        /// #     #[repr(u64)]
        /// #     pub Scope[u64] {
        /// #         Hello["hello"] => 1u64 << 0u64;
        /// #         World["world"] => 1u64 << 1u64;
        /// #     }
        /// # }
        /// #
        /// let mut bitfield = Bitfield::<Scope>::new(0);
        ///
        /// bitfield.add([Scope::Hello]);
        /// assert_eq!(bitfield.value(), 1);
        ///
        /// bitfield.remove([Scope::Hello]);
        /// assert_eq!(bitfield.value(), 0);
        /// ```
        pub fn remove<II: Into<F::Bit>, I: IntoIterator<Item = II>>(
            &mut self,
            values: I,
        ) {
            let iter = values.into_iter().map(Into::into);
            let removed = iter
                .fold(
                    self.0,
                    |mut curr, elem: u64| {
                        if elem == u64::MAX {
                            return curr;
                        }
                        if elem > F::max() {
                            return curr;
                        }
                        curr |= elem;
                        curr
                    },
                );
            self.0 &= min(removed, 0);
        }
        /// Determines if `bit` is contained in the inner bit.
        pub fn contains<B: Into<F::Bit>>(&self, bit: B) -> bool {
            (self.value() & bit.into()) != 0
        }
    }
    impl<F: Bitflags> Default for Bitfield<F> {
        fn default() -> Self {
            Bitfield(F::ZERO, PhantomData)
        }
    }
    impl<F: Bitflags<Bit = u64>> FromIterator<u64> for Bitfield<F> {
        fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
            let mut bitfield = Bitfield::<F>::default();
            bitfield.add(iter);
            bitfield
        }
    }
}
pub mod serde {
    mod duration {
        use serde::{de, Deserialize, Serialize};
        use std::{fmt::Display, ops::Deref, str::FromStr};
        /// Newtype wrapper for <code>[`std::time::Duration`]</code>.
        ///
        /// This newtype wrapper implements all the standard library types, [`serde::Serialize`],
        /// [`serde::Deserialize`], and others provided by feature flags:
        ///* [`utoipa::PartialSchema`], [`utoipa::ToSchema`] (via the `openapi` crate feature)
        /// [`utoipa::PartialSchema`]: https://docs.rs/utoipa/*/utoipa/trait.PartialSchema.html
        /// [`utoipa::ToSchema`]: https://docs.rs/utoipa/*/utoipa/trait.ToSchema.html
        pub struct Duration(std::time::Duration);
        #[automatically_derived]
        impl ::core::fmt::Debug for Duration {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Duration",
                    &&self.0,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Duration {
            #[inline]
            fn clone(&self) -> Duration {
                let _: ::core::clone::AssertParamIsClone<std::time::Duration>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Duration {}
        #[automatically_derived]
        impl ::core::default::Default for Duration {
            #[inline]
            fn default() -> Duration {
                Duration(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Duration {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Duration {
            #[inline]
            fn eq(&self, other: &Duration) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Duration {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<std::time::Duration>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for Duration {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Duration,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for Duration {
            #[inline]
            fn cmp(&self, other: &Duration) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Duration {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl Duration {
            /// Creates a new `Duration` from the specified number of whole seconds.
            ///
            /// # Examples
            ///
            /// ```
            /// use charted_core::serde::Duration;
            ///
            /// let duration = Duration::from_secs(5);
            ///
            /// assert_eq!(5, duration.as_secs());
            /// assert_eq!(0, duration.subsec_nanos());
            /// ```
            pub const fn from_secs(secs: u64) -> Duration {
                Duration(::std::time::Duration::from_secs(secs))
            }
        }
        impl Display for Duration {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let fmter = humantime::format_duration(self.0);
                <humantime::FormattedDuration as Display>::fmt(&fmter, f)
            }
        }
        impl FromStr for Duration {
            type Err = humantime::DurationError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                humantime::parse_duration(s).map(Duration)
            }
        }
        /// [`serde::Serialize`] for [`std::time::Duration`]: serialized as a u128 value
        /// pointed to the whole millisecond duration.
        impl Serialize for Duration {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_u128(self.0.as_millis())
            }
        }
        impl<'de> Deserialize<'de> for Duration {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct Visitor;
                impl serde::de::Visitor<'_> for Visitor {
                    type Value = Duration;
                    fn expecting(
                        &self,
                        fmt: &mut std::fmt::Formatter,
                    ) -> std::fmt::Result {
                        fmt.write_str("a string of a valid duration or a `u64` value")
                    }
                    fn visit_u64<E: de::Error>(
                        self,
                        value: u64,
                    ) -> Result<Self::Value, E> {
                        Ok(Duration(std::time::Duration::from_millis(value)))
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        humantime::parse_duration(v)
                            .map(Duration)
                            .map_err(de::Error::custom)
                    }
                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        self.visit_str(v.as_str())
                    }
                }
                deserializer.deserialize_any(Visitor)
            }
        }
        impl From<std::time::Duration> for Duration {
            fn from(value: std::time::Duration) -> Self {
                Self(value)
            }
        }
        impl From<Duration> for std::time::Duration {
            fn from(value: Duration) -> Self {
                value.0
            }
        }
        impl From<&Duration> for std::time::Duration {
            fn from(value: &Duration) -> Self {
                value.0
            }
        }
        impl Deref for Duration {
            type Target = ::std::time::Duration;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        #[cfg(feature = "openapi")]
        const _: () = {
            use std::borrow::Cow;
            use utoipa::{
                openapi::{
                    schema::SchemaType, KnownFormat, ObjectBuilder, OneOfBuilder, RefOr,
                    Schema, SchemaFormat, Type,
                },
                PartialSchema, ToSchema,
            };
            impl PartialSchema for Duration {
                fn schema() -> RefOr<Schema> {
                    let oneof = OneOfBuilder::new()
                        .description(
                            Some(
                                "`Duration` is represented as a span of time, usually for system timeouts. `charted-server` supports passing in a unsigned 64-bot integer (represented in milliseconds) or with a string literal (i.e, `1s`) to represent time.",
                            ),
                        )
                        .item({
                            ObjectBuilder::new()
                                .schema_type(SchemaType::Type(Type::Number))
                                .format(
                                    Some(SchemaFormat::KnownFormat(KnownFormat::UInt64)),
                                )
                                .description(
                                    Some("Span of time represented in milliseconds"),
                                )
                                .build()
                        })
                        .item({
                            ObjectBuilder::new()
                                .schema_type(SchemaType::Type(Type::String))
                                .description(
                                    Some(
                                        "Span of time represented in a humane format like `1s`, `15 days`, etc.",
                                    ),
                                )
                                .build()
                        });
                    RefOr::T(Schema::OneOf(oneof.build()))
                }
            }
            impl ToSchema for Duration {
                fn name() -> Cow<'static, str> {
                    Cow::Borrowed("Duration")
                }
            }
        };
    }
    pub use duration::*;
}
#[macro_use]
mod macros {}
mod distribution {
    use serde::Serialize;
    use std::{env, fmt::Display, fs, path::PathBuf, sync::OnceLock};
    const KUBERNETES_SERVICE_TOKEN_FILE: &str = "/run/secrets/kubernetes.io/serviceaccount/token";
    const KUBERNETES_NAMESPACE_FILE: &str = "/run/secrets/kubernetes.io/serviceaccount/namespace";
    /// Automatic detection to check if the distribution of charted-server is running on a
    /// Kubernetes cluster as a pod or not. It'll check in the following paths and check if
    /// they exist:
    ///
    /// * `/run/secrets/kubernetes.io/serviceaccount/token`
    /// * `/run/secrets/kubernetes.io/serviceaccount/namespace`
    fn is_in_k8s() -> bool {
        if env::var("KUBERNETES_SERVICE_HOST").is_ok() {
            return true;
        }
        PathBuf::from(KUBERNETES_SERVICE_TOKEN_FILE)
            .try_exists()
            .or_else(|_| PathBuf::from(KUBERNETES_NAMESPACE_FILE).try_exists())
            .unwrap_or_default()
    }
    /// Detects if charted-server is running as a Docker container, it'll check if
    /// `/.dockerenv` exists or if `/proc/self/cgroup` contains `docker` in it.
    fn is_in_docker_container() -> bool {
        let has_dockerenv = PathBuf::from("/.dockerenv")
            .try_exists()
            .unwrap_or_default();
        let has_cgroup = {
            let cgroup = PathBuf::from("/proc/self/cgroup");
            let Ok(contents) = fs::read_to_string(cgroup) else {
                return false;
            };
            contents.contains("docker")
        };
        has_dockerenv || has_cgroup
    }
    #[serde(rename_all = "lowercase")]
    pub enum Distribution {
        /// Running on a Kubernetes cluster.
        Kubernetes,
        /// This build of charted-server was built from source.
        #[serde(rename = "from_source")]
        #[default]
        FromSource,
        /// Running as a Docker container.
        Docker,
        /// Running from a Nix flake.
        Nix,
        /// Uses a locally built binary from the host.
        Git,
    }
    impl utoipa::__dev::ComposeSchema for Distribution {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            utoipa::openapi::schema::Object::builder()
                .schema_type(
                    utoipa::openapi::schema::SchemaType::new(
                        utoipa::openapi::schema::Type::String,
                    ),
                )
                .enum_values::<
                    [&str; 5usize],
                    &str,
                >(Some(["kubernetes", "from_source", "docker", "nix", "git"]))
                .into()
        }
    }
    impl utoipa::ToSchema for Distribution {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("Distribution")
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
    impl ::core::fmt::Debug for Distribution {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Distribution::Kubernetes => "Kubernetes",
                    Distribution::FromSource => "FromSource",
                    Distribution::Docker => "Docker",
                    Distribution::Nix => "Nix",
                    Distribution::Git => "Git",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Distribution {
        #[inline]
        fn clone(&self) -> Distribution {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Distribution {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Distribution {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Distribution::Kubernetes => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Distribution",
                            0u32,
                            "kubernetes",
                        )
                    }
                    Distribution::FromSource => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Distribution",
                            1u32,
                            "from_source",
                        )
                    }
                    Distribution::Docker => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Distribution",
                            2u32,
                            "docker",
                        )
                    }
                    Distribution::Nix => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Distribution",
                            3u32,
                            "nix",
                        )
                    }
                    Distribution::Git => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "Distribution",
                            4u32,
                            "git",
                        )
                    }
                }
            }
        }
    };
    #[automatically_derived]
    impl ::core::default::Default for Distribution {
        #[inline]
        fn default() -> Distribution {
            Self::FromSource
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Distribution {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Distribution {
        #[inline]
        fn eq(&self, other: &Distribution) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Distribution {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl Distribution {
        pub fn detect() -> Distribution {
            static ONCE: OnceLock<Distribution> = OnceLock::new();
            *ONCE
                .get_or_init(|| {
                    if is_in_k8s() {
                        return Distribution::Kubernetes;
                    }
                    if is_in_docker_container() {
                        return Distribution::Docker;
                    }
                    match ::core::option::Option::Some("git") {
                        Some(s) => {
                            match s {
                                "docker" => Distribution::Docker,
                                "git" => Distribution::Git,
                                "nix" => Distribution::Nix,
                                _ => Distribution::FromSource,
                            }
                        }
                        None => Distribution::FromSource,
                    }
                })
        }
    }
    impl Display for Distribution {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Distribution::Kubernetes => f.write_str("kubernetes"),
                Distribution::Docker => f.write_str("docker"),
                Distribution::Nix => f.write_str("nix package manager"),
                Distribution::Git => f.write_str("git"),
                _ => f.write_str("«from source»"),
            }
        }
    }
}
mod ext {
    /// Extension trait to extend <code>[`Result`]\<T, E\></code>.
    pub trait ResultExt<E>: Sized {
        type Ok;
        /// Transforms this [`Result`] into a [`eyre::Result`] with a
        /// [`eyre::Report`] as the error.
        fn into_report(self) -> eyre::Result<Self::Ok>
        where
            E: Into<eyre::Report>;
    }
    impl<T, E> ResultExt<E> for Result<T, E> {
        type Ok = T;
        fn into_report(self) -> eyre::Result<<Result<T, E> as ResultExt<E>>::Ok>
        where
            E: Into<eyre::Report>,
        {
            self.map_err(Into::into)
        }
    }
}
use argon2::Argon2;
pub use distribution::*;
pub use ext::*;
use rand::distr::{Alphanumeric, SampleString};
use std::sync::{LazyLock, OnceLock};
/// Type-alias that represents a boxed future.
pub type BoxedFuture<'a, Output> = ::core::pin::Pin<
    ::std::boxed::Box<dyn ::core::future::Future<Output = Output> + Send + 'a>,
>;
/// Returns the version of the Rust compiler that charted-server
/// was compiled on.
pub const RUSTC_VERSION: &str = "1.87.0-nightly";
/// Returns the Git commit hash from the charted-server repository that
/// this build was built off from.
pub const COMMIT_HASH: &str = "aa15e981";
/// RFC3339-formatted date of when charted-server was last built at.
pub const BUILD_DATE: &str = "2025-03-08T09:36:08.685954602+00:00";
/// Returns the current version of `charted-server`.
pub const VERSION: &str = "0.1.0";
/// A lazily cached [`Argon2`] instance that is used within
/// the internal `charted-*` crates.
pub static ARGON2: LazyLock<Argon2> = LazyLock::new(Argon2::default);
/// Returns a formatted string of the version that combines the [`VERSION`] and
/// [`COMMIT_HASH`] constants as
/// <code>v[{version}][VERSION]+[{commit.hash}][COMMIT_HASH]</code>.
///
/// If the [`COMMIT_HASH`] is empty (i.e, not by using `git` or wasn't found on system),
/// it'll return <code>v[{version}][VERSION]</code> instead. This is also returned on the
/// `nixpkgs` version of **charted** and **charted-helm-plugin**.
pub fn version() -> &'static str {
    static ONCE: OnceLock<String> = OnceLock::new();
    ONCE.get_or_init(|| {
        use std::fmt::Write;
        let mut buf = String::new();
        buf.write_fmt(format_args!("v{0}", VERSION)).unwrap();
        #[allow(clippy::const_is_empty)]
        if !(COMMIT_HASH == "d1cebae" || COMMIT_HASH.is_empty()) {
            buf.write_fmt(format_args!("+{0}", COMMIT_HASH)).unwrap();
        }
        buf
    })
}
/// Generates a random string with `len`.
pub fn rand_string(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
}
