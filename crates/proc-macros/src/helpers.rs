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

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Attribute, Error, Expr, ExprLit, Lit, Meta};

pub fn extract_doc_comments<'a, I: Iterator<Item = &'a Attribute>>(attrs: I) -> Result<Vec<String>, syn::Error> {
    let literals = attrs
        .into_iter()
        .filter_map(|attr| match &attr.meta {
            Meta::NameValue(nv) if nv.path.is_ident("doc") => Some(nv.value.clone()),
            _ => None,
        })
        .map(|expr| match expr {
            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(s.value()),
            e => Err(e),
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|expr| Error::new(expr.span(), "Doc comment was not a string literal"))?;

    Ok(literals
        .iter()
        .flat_map(|f| f.split('\n').collect::<Vec<_>>())
        .map(|line| line.to_string())
        .collect())
}

/// Helper struct for [`Option`][std::option::Option] to implement [`ToTokens`]
#[derive(Clone)]
pub struct OptionHelper<T: ToTokens + Clone>(pub std::option::Option<T>);

impl<T: ToTokens + Clone> ToTokens for OptionHelper<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0.clone() {
            Some(tok) => tokens.extend(quote!(Some(#tok))),
            None => tokens.extend(quote!(None)),
        }
    }
}

impl<U, T: ToTokens + Clone + From<U>> From<std::option::Option<U>> for OptionHelper<T> {
    fn from(value: std::option::Option<U>) -> Self {
        match value {
            Some(tok) => OptionHelper(Some(T::from(tok))),
            None => OptionHelper(None),
        }
    }
}

/// Helper struct for [`Vec`][std::vec::Vec] that implements [`ToTokens`]. This was
/// implemented due to inconsistences when repeatition were used with `vec![...];`
#[derive(Clone)]
pub struct VecHelper<T: ToTokens + Clone>(pub std::vec::Vec<T>);

impl<T: ToTokens + Clone> ToTokens for VecHelper<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut variants = vec![];
        for me in self.0.iter() {
            variants.push(quote! { vec.push(::core::convert::Into::into(#me)); });
        }

        tokens.extend(quote! {
            let mut vec = ::std::vec::Vec::new();
            #(#variants)*

            vec
        });
    }
}

impl<U: Clone, T: From<U> + ToTokens + Clone> From<std::vec::Vec<U>> for VecHelper<T> {
    fn from(value: std::vec::Vec<U>) -> Self {
        let mut handles: Vec<T> = vec![];
        for handle in value.iter() {
            handles.push(T::from(handle.clone()));
        }

        Self(handles)
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringHelper(std::string::String);
impl ToTokens for StringHelper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = self.0.clone();
        tokens.extend(quote!(::std::string::String::from(#s)));
    }
}

impl From<std::string::String> for StringHelper {
    fn from(value: std::string::String) -> Self {
        Self(value)
    }
}

/// Helper struct for [`RefOr`][utoipa::openapi::RefOr] to implement [`ToTokens`].
#[derive(Clone)]
pub enum RefOr<T: ToTokens> {
    Ref(Ref),
    T(T),
}

impl<T: ToTokens> ToTokens for RefOr<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RefOr::Ref(r) => tokens.extend(quote! { ::utoipa::openapi::RefOr::Ref(#r) }),
            RefOr::T(inner) => tokens.extend(quote! { ::utoipa::openapi::RefOr::T(#inner) }),
        }
    }
}

impl<U, T: ToTokens + From<U>> From<utoipa::openapi::RefOr<U>> for RefOr<T> {
    fn from(value: utoipa::openapi::RefOr<U>) -> Self {
        match value {
            utoipa::openapi::RefOr::Ref(r) => Self::Ref(Ref(r)),
            utoipa::openapi::RefOr::T(t) => Self::T(T::from(t)),
        }
    }
}

/// Helper struct for [`Ref`][utoipa::openapi::Ref], but implements [`ToTokens`].
#[derive(Clone)]
pub struct Ref(utoipa::openapi::Ref);

impl ToTokens for Ref {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let loc = self.0.ref_location.clone();
        let loc_qt = quote!(#loc);

        tokens.extend(quote! {
            ::utoipa::openapi::Ref::new(#loc_qt)
        });
    }
}

impl From<utoipa::openapi::Ref> for Ref {
    fn from(value: utoipa::openapi::Ref) -> Self {
        Ref(value.clone())
    }
}

/// Helper struct for [`PathItemType`][utoipa::openapi::path::PathItemType], but
/// implements [`ToTokens`].
///
/// This enum only supports the HTTP verbs that charted-server uses.
#[derive(Clone)]
pub enum PathItemType {
    Delete,
    Patch,
    Post,
    Put,
    Get,
}

impl ToTokens for PathItemType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PathItemType::Delete => tokens.extend(quote! { ::utoipa::openapi::path::PathItemType::Delete }),
            PathItemType::Patch => tokens.extend(quote! { ::utoipa::openapi::path::PathItemType::Patch }),
            PathItemType::Post => tokens.extend(quote! { ::utoipa::openapi::path::PathItemType::Post }),
            PathItemType::Put => tokens.extend(quote! { ::utoipa::openapi::path::PathItemType::Put }),
            PathItemType::Get => tokens.extend(quote! { ::utoipa::openapi::path::PathItemType::Get }),
        }
    }
}

impl From<utoipa::openapi::path::PathItemType> for PathItemType {
    fn from(value: utoipa::openapi::path::PathItemType) -> Self {
        match value {
            utoipa::openapi::PathItemType::Delete => Self::Delete,
            utoipa::openapi::PathItemType::Patch => Self::Patch,
            utoipa::openapi::PathItemType::Post => Self::Post,
            utoipa::openapi::PathItemType::Put => Self::Put,
            utoipa::openapi::PathItemType::Get => Self::Get,
            _ => unreachable!(),
        }
    }
}

/// Helper struct for [`Response`][utoipa::openapi::Response], that implements
/// [`ToTokens`].
#[derive(Clone)]
pub struct Response(utoipa::openapi::Response);

impl ToTokens for Response {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut variants: Vec<TokenStream> = vec![];

        let desc = self.0.description.clone();
        variants.push(quote!(.description(#desc)));

        for (key, content) in self.0.content.clone() {
            let content = Content::from(content);
            variants.push(quote!(.content(#key, #content)));
        }

        tokens.extend(quote! {
            ::utoipa::openapi::ResponseBuilder::new()#(#variants)*.build()
        })
    }
}

impl From<utoipa::openapi::Response> for Response {
    fn from(value: utoipa::openapi::Response) -> Self {
        Self(value)
    }
}

/// Helper struct for [`Operation`][utoipa::openapi::path::Operation], but implements [`ToTokens`].
#[derive(Clone)]
pub struct Operation(utoipa::openapi::path::Operation);

impl ToTokens for Operation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let operation = self.0.clone();
        let mut iterations: Vec<TokenStream> = vec![];

        if let Some(tags) = operation.tags.clone() {
            for tag in tags.iter() {
                iterations.push(quote!(.tag(#tag)));
            }
        }

        if let Some(description) = operation.description.clone() {
            iterations.push(quote! { .description(Some(#description)) });
        }

        if let Some(op_id) = operation.operation_id.clone() {
            iterations.push(quote! { .operation_id(Some(#op_id)) });
        }

        if let Some(params) = operation.parameters.clone() {
            for param in params.iter().map(|f| Parameter::from(f.clone())) {
                iterations.push(quote!(.parameter(#param)));
            }
        }

        if let Some(req_body) = operation.request_body.clone() {
            let body = RequestBody::from(req_body);
            iterations.push(quote!(.request_body(Some(#body))));
        }

        if let Some(deprecated) = operation.deprecated.clone() {
            let deprecated = Deprecated::from(deprecated);
            iterations.push(quote!(.deprecated(Some(#deprecated))));
        }

        for (status, res) in operation.responses.responses.iter() {
            let RefOr::T(res) = RefOr::<Response>::from(res.clone()) else {
                continue;
            };

            iterations.push(quote!(.response(#status, #res)));
        }

        tokens.extend(quote! {
            ::utoipa::openapi::path::OperationBuilder::new()#(#iterations)*.build()
        })
    }
}

impl From<utoipa::openapi::path::Operation> for Operation {
    fn from(value: utoipa::openapi::path::Operation) -> Self {
        Self(value)
    }
}

/// Helper enum for [`ParameterIn`][utoipa::openapi::path::ParameterIn] to implement
/// [`ToTokens`]. This enum is only meant for what charted-server will usually fill
/// in.
#[derive(Clone)]
pub enum ParameterIn {
    Path,
    Query,
}

impl ToTokens for ParameterIn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ParameterIn::Query => tokens.extend(quote! { ::utoipa::openapi::path::ParameterIn::Query }),
            ParameterIn::Path => tokens.extend(quote! { ::utoipa::openapi::path::ParameterIn::Path }),
        }
    }
}

impl From<utoipa::openapi::path::ParameterIn> for ParameterIn {
    fn from(value: utoipa::openapi::path::ParameterIn) -> Self {
        match value {
            utoipa::openapi::path::ParameterIn::Query => Self::Query,
            utoipa::openapi::path::ParameterIn::Path => Self::Path,
            _ => unreachable!(),
        }
    }
}

/// Helper enum for [`ParameterStyle`][utoipa::openapi::path::ParameterStyle] to implement
/// [`ToTokens`]. This enum is only meant for what charted-server will usually fill
/// in.
#[derive(Clone)]
pub enum ParameterStyle {
    /// Default for [`ParameterType::Query`][utoipa::openapi::path::ParameterIn::Query]
    Form,

    /// Default for [`ParameterType::Path`][utoipa::openapi::path::ParameterIn::Path]
    Simple,
}

impl ToTokens for ParameterStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ParameterStyle::Simple => tokens.extend(quote! { ::utoipa::openapi::path::ParameterStyle::Simple }),
            ParameterStyle::Form => tokens.extend(quote! { ::utoipa::openapi::path::ParameterStyle::Form }),
        }
    }
}

impl From<utoipa::openapi::path::ParameterIn> for ParameterStyle {
    fn from(value: utoipa::openapi::path::ParameterIn) -> Self {
        match value {
            utoipa::openapi::path::ParameterIn::Query => Self::Form,
            utoipa::openapi::path::ParameterIn::Path => Self::Simple,
            _ => unreachable!(),
        }
    }
}

impl From<utoipa::openapi::path::ParameterStyle> for ParameterStyle {
    fn from(value: utoipa::openapi::path::ParameterStyle) -> Self {
        match value {
            utoipa::openapi::path::ParameterStyle::Form => Self::Form,
            utoipa::openapi::path::ParameterStyle::Simple => Self::Simple,
            _ => unreachable!(),
        }
    }
}

/// Helper enum for [`Deprecated`][utoipa::openapi::Deprecated], but implements [`ToTokens`].
#[derive(Clone)]
pub enum Deprecated {
    True,
    False,
}

impl ToTokens for Deprecated {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Deprecated::True => tokens.extend(quote! { ::utoipa::openapi::Deprecated::True }),
            Deprecated::False => tokens.extend(quote! { ::utoipa::openapi::Deprecated::False }),
        }
    }
}

impl From<utoipa::openapi::Deprecated> for Deprecated {
    fn from(value: utoipa::openapi::Deprecated) -> Self {
        match value {
            utoipa::openapi::Deprecated::False => Self::False,
            utoipa::openapi::Deprecated::True => Self::True,
        }
    }
}

/// Helper enum for [`Required`][utoipa::openapi::Deprecated], but implements [`ToTokens`].
#[derive(Clone)]
pub enum Required {
    True,
    False,
}

impl ToTokens for Required {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Required::True => tokens.extend(quote! { ::utoipa::openapi::Required::True }),
            Required::False => tokens.extend(quote! { ::utoipa::openapi::Required::False }),
        }
    }
}

impl From<utoipa::openapi::Required> for Required {
    fn from(value: utoipa::openapi::Required) -> Self {
        match value {
            utoipa::openapi::Required::False => Self::False,
            utoipa::openapi::Required::True => Self::True,
        }
    }
}

/// Helper struct for [`Parameter`][utoipa::openapi::path::Parameter], which implements
/// [`ToTokens`].
#[derive(Clone)]
pub struct Parameter(pub utoipa::openapi::path::Parameter);

impl ToTokens for Parameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut variants: Vec<TokenStream> = vec![];

        let name = self.0.name.clone();
        variants.push(quote! {
            .name(#name)
        });

        let param_in: ParameterIn = self.0.parameter_in.clone().into();
        variants.push(quote! {
            .parameter_in(#param_in)
        });

        let required: Required = self.0.required.clone().into();
        variants.push(quote! {
            .required(#required)
        });

        if let Some(desc) = self.0.description.clone() {
            variants.push(quote! {
                .description(Some(#desc))
            });
        }

        if let Some(deprecated) = self.0.deprecated.clone() {
            let deprecated: Deprecated = deprecated.into();
            variants.push(quote! {
                .deprecated(Some(#deprecated))
            });
        }

        if let Some(schema) = self.0.schema.clone() {
            let schema: RefOr<Schema> = schema.into();
            variants.push(quote! {
                .schema(Some(#schema))
            });
        }

        tokens.extend(quote! {
            ::utoipa::openapi::path::ParameterBuilder::new()#(#variants)*.build()
        });
    }
}

impl From<utoipa::openapi::path::Parameter> for Parameter {
    fn from(value: utoipa::openapi::path::Parameter) -> Self {
        Self(value)
    }
}

/// Helper struct for [`RequestBody`][utoipa::openapi::request_body::RequestBody], that
/// implements [`ToTokens`].
#[derive(Clone)]
pub struct RequestBody(utoipa::openapi::request_body::RequestBody);

impl ToTokens for RequestBody {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut variants: Vec<TokenStream> = vec![];
        if let Some(desc) = self.0.description.clone() {
            variants.push(quote!(.description(Some(#desc))));
        }

        if let Some(required) = self.0.required.clone() {
            let required: Required = required.into();
            variants.push(quote!(.required(Some(#required))));
        }

        for (key, value) in self.0.content.clone().iter() {
            let content = Content::from(value.clone());
            variants.push(quote!(.content(#key, #content)));
        }

        tokens.extend(quote! {
            ::utoipa::openapi::request_body::RequestBodyBuilder::new()#(#variants)*.build()
        });
    }
}

impl From<utoipa::openapi::request_body::RequestBody> for RequestBody {
    fn from(value: utoipa::openapi::request_body::RequestBody) -> Self {
        Self(value)
    }
}

/// Helper struct for [`Content`][utoipa::openapi::Content], that implements [`ToTokens`].
#[derive(Clone)]
pub struct Content(utoipa::openapi::Content);

impl ToTokens for Content {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let schema: RefOr<Schema> = self.0.schema.clone().into();
        tokens.extend(quote! {
            ::utoipa::openapi::ContentBuilder::new().schema(#schema).build()
        })
    }
}

impl From<utoipa::openapi::Content> for Content {
    fn from(value: utoipa::openapi::Content) -> Self {
        Self(value)
    }
}

/// Helper struct for [`Schema`][utoipa::openapi::Schema], that implements [`ToTokens`].
#[derive(Clone)]
pub enum Schema {
    Array(schema::Array),
    Object(schema::Object),
}

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Object(obj) => tokens.extend(quote!(::utoipa::openapi::Schema::Object(#obj))),
            Self::Array(arr) => tokens.extend(quote!(::utoipa::openapi::Schema::Array(#arr))),
        }
    }
}

impl From<utoipa::openapi::Schema> for Schema {
    fn from(value: utoipa::openapi::Schema) -> Self {
        match value {
            utoipa::openapi::Schema::Object(obj) => Self::Object(obj.into()),
            utoipa::openapi::Schema::Array(arr) => Self::Array(arr.into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct SecurityRequirement(pub (String, Vec<String>));

impl ToTokens for SecurityRequirement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (name, scopes) = self.0.clone();
        let name = StringHelper::from(name);
        let scopes: VecHelper<StringHelper> = VecHelper::from(scopes);

        tokens.extend(quote!(::utoipa::openapi::SecurityRequirement::new(#name, #scopes.iter())));
    }
}

/// External helpers for the [`Schema`] helper struct.
pub mod schema {
    use super::StringHelper;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};

    /// Helper struct for [`Array`][utoipa::openapi::Array], that implements [`ToTokens`].
    #[derive(Clone)]
    pub struct Array(utoipa::openapi::Array);

    impl ToTokens for Array {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let mut variants: Vec<TokenStream> = vec![];

            let schema_type: super::schema::SchemaType = self.0.schema_type.clone().into();
            variants.push(quote!(.schema_type(#schema_type)));

            if let Some(desc) = self.0.description.clone() {
                variants.push(quote!(.description(Some(#desc))));
            }

            if let Some(deprecated) = self.0.deprecated.clone() {
                let deprecated: super::Deprecated = deprecated.into();
                variants.push(quote!(.deprecated(#deprecated)));
            }

            if self.0.nullable {
                let nullable = self.0.nullable;
                variants.push(quote!(.nullable(#nullable)));
            }

            tokens.extend(quote! {
                ::utoipa::openapi::ArrayBuilder::new()#(#variants)*.build()
            })
        }
    }

    impl From<utoipa::openapi::Array> for Array {
        fn from(value: utoipa::openapi::Array) -> Self {
            Self(value)
        }
    }

    /// Helper struct for [`Object`][utoipa::openapi::Object], that implements [`ToTokens`].
    #[derive(Clone)]
    pub struct Object(utoipa::openapi::Object);

    impl ToTokens for Object {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let mut variants: Vec<TokenStream> = vec![];

            let schema_type: super::schema::SchemaType = self.0.schema_type.clone().into();
            variants.push(quote!(.schema_type(#schema_type)));

            if let Some(desc) = self.0.description.clone() {
                variants.push(quote!(.description(Some(#desc))));
            }

            if let Some(deprecated) = self.0.deprecated.clone() {
                let deprecated: super::Deprecated = deprecated.into();
                variants.push(quote!(.deprecated(#deprecated)));
            }

            if self.0.nullable {
                let nullable = self.0.nullable;
                variants.push(quote!(.nullable(#nullable)));
            }

            if let Some(format) = self.0.format.clone() {
                let format: super::schema::SchemaFormat = format.into();
                variants.push(quote!(.format(Some(#format))));
            }

            for elem in self.0.required.clone() {
                let elem: super::StringHelper = elem.into();
                variants.push(quote!(.required(#elem)));
            }

            for (prop, r) in self.0.properties.iter() {
                let prop: super::StringHelper = <std::string::String as Into<StringHelper>>::into(prop.clone());
                let r: super::RefOr<super::Schema> = <utoipa::openapi::RefOr<utoipa::openapi::Schema> as Into<
                    super::RefOr<super::Schema>,
                >>::into(r.clone());

                variants.push(quote!(.property(#prop, #r)));
            }

            if let Some(maximum) = self.0.maximum {
                variants.push(quote!(.maximum(Some(#maximum))));
            }

            if let Some(minimum) = self.0.minimum {
                variants.push(quote!(.minimum(Some(#minimum))));
            }

            if let Some(maximum) = self.0.exclusive_maximum {
                variants.push(quote!(.exclusive_maximum(Some(#maximum))));
            }

            if let Some(minimum) = self.0.exclusive_minimum {
                variants.push(quote!(.exclusive_minimum(Some(#minimum))));
            }

            if let Some(max_len) = self.0.max_length {
                variants.push(quote!(.max_length(Some(#max_len))));
            }

            if let Some(min_len) = self.0.min_length {
                variants.push(quote!(.min_length(Some(#min_len))));
            }

            if let Some(pattern) = self.0.pattern.clone() {
                variants.push(quote!(.pattern(#pattern)));
            }

            tokens.extend(quote! {
                ::utoipa::openapi::ObjectBuilder::new()#(#variants)*.build()
            })
        }
    }

    impl From<utoipa::openapi::Object> for Object {
        fn from(value: utoipa::openapi::Object) -> Self {
            Self(value)
        }
    }

    /// Helper enum for [`SchemaType`][utoipa::openapi::SchemaType], that implements [`ToTokens`].
    #[derive(Clone)]
    pub enum SchemaType {
        Object,
        Value,
        String,
        Integer,
        Number,
        Boolean,
        Array,
    }

    impl ToTokens for SchemaType {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match self {
                SchemaType::Object => tokens.extend(quote! { ::utoipa::openapi::SchemaType::Object }),
                SchemaType::Value => tokens.extend(quote! { ::utoipa::openapi::SchemaType::Value }),
                SchemaType::String => tokens.extend(quote! { ::utoipa::openapi::SchemaType::String }),
                SchemaType::Integer => tokens.extend(quote! { ::utoipa::openapi::SchemaType::Integer }),
                SchemaType::Number => tokens.extend(quote! { ::utoipa::openapi::SchemaType::Number }),
                SchemaType::Boolean => tokens.extend(quote! { ::utoipa::openapi::SchemaType::Boolean }),
                SchemaType::Array => tokens.extend(quote! { ::utoipa::openapi::SchemaType::Array }),
            }
        }
    }

    impl From<utoipa::openapi::SchemaType> for SchemaType {
        fn from(value: utoipa::openapi::SchemaType) -> Self {
            match value {
                utoipa::openapi::SchemaType::Array => SchemaType::Array,
                utoipa::openapi::SchemaType::Boolean => SchemaType::Boolean,
                utoipa::openapi::SchemaType::Integer => SchemaType::Integer,
                utoipa::openapi::SchemaType::Number => SchemaType::Number,
                utoipa::openapi::SchemaType::Object => SchemaType::Object,
                utoipa::openapi::SchemaType::String => SchemaType::String,
                utoipa::openapi::SchemaType::Value => SchemaType::Value,
            }
        }
    }

    /// Helper enum for [`SchemaFormat`][utoipa::openapi::SchemaFormat], that implements [`ToTokens`].
    #[derive(Clone)]
    pub enum SchemaFormat {
        KnownFormat(KnownFormat),
        Custom(String),
    }

    impl ToTokens for SchemaFormat {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match self {
                Self::KnownFormat(format) => {
                    tokens.extend(quote! { ::utoipa::openapi::SchemaFormat::KnownFormat(#format) })
                }

                Self::Custom(custom) => tokens.extend(quote! { ::utoipa::openapi::SchemaFormat::Custom(#custom) }),
            }
        }
    }

    impl From<utoipa::openapi::SchemaFormat> for SchemaFormat {
        fn from(value: utoipa::openapi::SchemaFormat) -> Self {
            match value {
                utoipa::openapi::SchemaFormat::Custom(custom) => Self::Custom(custom),
                utoipa::openapi::SchemaFormat::KnownFormat(known) => Self::KnownFormat(known.into()),
            }
        }
    }

    /// Helper enum for [`KnownFormat`][utoipa::openapi::KnownFormat], that implements [`ToTokens`].
    #[derive(Clone)]
    pub enum KnownFormat {
        Float,
        Double,
        Byte,
        Binary,
        Date,
        DateTime,
        Password,
        Int32,
        Int64,
    }

    impl ToTokens for KnownFormat {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match self {
                KnownFormat::Binary => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::Binary }),
                KnownFormat::Double => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::Double }),
                KnownFormat::Float => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::Float }),
                KnownFormat::Byte => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::Byte }),
                KnownFormat::Date => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::Date }),
                KnownFormat::DateTime => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::DateTime }),
                KnownFormat::Password => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::Password }),
                KnownFormat::Int32 => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::Int32 }),
                KnownFormat::Int64 => tokens.extend(quote! { ::utoipa::openapi::KnownFormat::Int64 }),
            }
        }
    }

    impl From<utoipa::openapi::KnownFormat> for KnownFormat {
        #[allow(unreachable_patterns)] // since we don't have 'uuid' enabled and it throws a compile error if we try to use KnownFormat::Uuid.
        fn from(value: utoipa::openapi::KnownFormat) -> Self {
            match value {
                utoipa::openapi::KnownFormat::Binary => KnownFormat::Binary,
                utoipa::openapi::KnownFormat::Byte => KnownFormat::Byte,
                utoipa::openapi::KnownFormat::Date => KnownFormat::Date,
                utoipa::openapi::KnownFormat::DateTime => KnownFormat::DateTime,
                utoipa::openapi::KnownFormat::Double => KnownFormat::Double,
                utoipa::openapi::KnownFormat::Float => KnownFormat::Float,
                utoipa::openapi::KnownFormat::Password => KnownFormat::Password,
                utoipa::openapi::KnownFormat::Int32 => KnownFormat::Int32,
                utoipa::openapi::KnownFormat::Int64 => KnownFormat::Int64,
                _ => todo!(),
            }
        }
    }
}

pub mod collections {
    use quote::{quote, ToTokens};
    use std::hash::Hash;

    /// Helper struct for [`BTreeMap`][std::collections::BTreeMap], but implements
    /// [`ToTokens`].
    #[derive(Clone)]
    pub struct BTreeMap<K: ToTokens + Hash + Eq, V: ToTokens>(pub std::collections::BTreeMap<K, V>);
    impl<K: ToTokens + Hash + Eq, V: ToTokens> ToTokens for BTreeMap<K, V> {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let mut variants: Vec<proc_macro2::TokenStream> = vec![];
            for (key, value) in self.0.iter() {
                variants.push(quote! { map.insert(#key, #value); });
            }

            tokens.extend(quote! {
                let mut map = ::std::collections::BTreeMap::new();
                #(#variants)*

                map
            });
        }
    }

    impl<K: ToTokens + Hash + Eq, V: ToTokens> From<std::collections::BTreeMap<K, V>> for BTreeMap<K, V> {
        fn from(value: std::collections::BTreeMap<K, V>) -> Self {
            Self(value)
        }
    }

    impl<K: ToTokens + Hash + Eq, V: ToTokens> std::ops::Deref for BTreeMap<K, V> {
        type Target = std::collections::BTreeMap<K, V>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    /// Helper struct for [`HashMap`][std::collections::HashMap], but implements
    /// [`ToTokens`].
    #[derive(Clone)]
    pub struct HashMap<K: ToTokens + Hash + Eq, V: ToTokens>(pub std::collections::HashMap<K, V>);
    impl<K: ToTokens + Hash + Eq, V: ToTokens> ToTokens for HashMap<K, V> {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let mut variants: Vec<proc_macro2::TokenStream> = vec![];
            for (key, value) in self.0.iter() {
                variants.push(quote! { map.insert(#key, #value); });
            }

            tokens.extend(quote! {
                let mut map = ::std::collections::HashMap::new();
                #(#variants)*

                map
            });
        }
    }

    impl<K: ToTokens + Hash + Eq, V: ToTokens> From<std::collections::HashMap<K, V>> for HashMap<K, V> {
        fn from(value: std::collections::HashMap<K, V>) -> Self {
            Self(value)
        }
    }

    impl<K: ToTokens + Hash + Eq, V: ToTokens> std::ops::Deref for HashMap<K, V> {
        type Target = std::collections::HashMap<K, V>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
