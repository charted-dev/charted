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

use crate::helpers::{Parameter, RequestBody, Response};
use charted_common::{models::Name, ID};
use proc_macro2::{Ident, Span};
use std::collections::HashMap;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Expr, ExprAssign, ExprLit, ExprMacro, ExprPath, ExprTuple, Lit, Macro, Result, Token,
};
use utoipa::{
    openapi::{
        path::{ParameterBuilder, ParameterIn},
        request_body::RequestBodyBuilder,
        ContentBuilder, KnownFormat, ObjectBuilder, PathItemType, Ref, RefOr, ResponseBuilder, Schema, SchemaFormat,
        SchemaType,
    },
    ToSchema,
};

#[derive(Clone, Default)]
pub enum Availability<T> {
    Available(T),

    #[default]
    Unavailable,
}

impl<T> Availability<T> {
    pub fn unwrap_or(self, else_: T) -> T {
        match self {
            Self::Available(item) => item,
            Self::Unavailable => else_,
        }
    }
}

#[derive(Clone, Default)]
pub struct Args {
    pub id: String,
    pub tags: Vec<String>,
    pub item_type: Availability<PathItemType>,
    pub responses: HashMap<u16, Response>,
    pub parameters: HashMap<String, Parameter>,
    pub description: Option<String>,
    pub request_body: Option<RequestBody>,
    pub is_deprecated: Option<Option<String>>,
}

mod kw {
    syn::custom_keyword!(securityScheme);
    syn::custom_keyword!(queryParameter);
    syn::custom_keyword!(pathParameter);
    syn::custom_keyword!(description);
    syn::custom_keyword!(requestBody);
    syn::custom_keyword!(deprecated);
    syn::custom_keyword!(response);
    syn::custom_keyword!(content);
    syn::custom_keyword!(method);
    syn::custom_keyword!(tags);
    syn::custom_keyword!(id);
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = Args::default();

        if input.peek(kw::method) {
            input.parse::<kw::method>()?;
            input.parse::<Token![=]>()?;

            let ident = input.parse::<Ident>()?;
            args.item_type = match ident.to_string().as_str() {
                "get" => Availability::Available(PathItemType::Get),
                "put" => Availability::Available(PathItemType::Put),
                "post" => Availability::Available(PathItemType::Post),
                "patch" => Availability::Available(PathItemType::Patch),
                "delete" => Availability::Available(PathItemType::Delete),
                _ => Availability::Unavailable,
            };

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if input.peek(kw::id) && args.id.is_empty() {
            args.parse_operation_id(input)?;
        }

        if input.peek(kw::tags) && args.tags.is_empty() {
            args.parse_tags(input)?;
        }

        if input.peek(kw::description) && args.description.is_none() {
            args.parse_description(input)?;
        }

        if input.peek(kw::deprecated) && args.is_deprecated.is_none() {
            args.parse_deprecation(input)?;
        }

        if input.peek(kw::requestBody) {
            let request_body = Args::parse_request_body(input)?;
            if args.request_body.is_none() {
                args.request_body = Some(request_body);
            }
        }

        // since multiple pathParameter, queryParameter, and response
        // can be collected, we will do a while loop for each
        while input.peek(kw::response) || input.peek(kw::pathParameter) || input.peek(kw::queryParameter) {
            if input.peek(kw::response) {
                let (status, res) = Args::parse_response(input)?;
                if args.responses.contains_key(&status) {
                    return Err(Error::new(
                        Span::call_site(),
                        format!("status code [{status}] is already available"),
                    ));
                }

                args.responses.insert(status, res);
            }

            if input.peek(kw::pathParameter) {
                let param = Args::parse_path_parameter(input)?;
                let inner = param.0.clone(); // we only want the name that we populated here :Nod:

                if args.parameters.contains_key(&inner.name) {
                    return Err(Error::new(
                        Span::call_site(),
                        format!("parameter {} is already registered", inner.name.clone()),
                    ));
                }

                args.parameters.insert(inner.name, param);
            }

            if input.peek(kw::queryParameter) {
                let param = Args::parse_query_parameter(input)?;
                let inner = param.0.clone(); // we only want the name that we populated here :Nod:

                if args.parameters.contains_key(&inner.name) {
                    return Err(Error::new(
                        Span::call_site(),
                        format!("parameter {} is already registered", inner.name.clone()),
                    ));
                }

                args.parameters.insert(inner.name, param);
            }
        }

        Ok(args)
    }
}

impl Args {
    pub(crate) fn parse_deprecation(&mut self, input: ParseStream) -> Result<()> {
        input.parse::<kw::deprecated>()?;

        // parses "= <why>"
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;

            let expr = input.parse::<Expr>()?;
            let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = expr else {
                return Err(Error::new(expr.span(), "expected literal string"));
            };

            let why = s.value();
            self.is_deprecated = Some(Some(why));
        } else {
            self.is_deprecated = Some(None);
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(())
    }

    pub(crate) fn parse_path_parameter(stream: ParseStream) -> Result<Parameter> {
        stream.parse::<kw::pathParameter>()?;

        let mut builder = ParameterBuilder::new().parameter_in(ParameterIn::Path);
        let c;
        parenthesized!(c in stream);

        let params = c.parse_terminated(Expr::parse, Token![,])?;
        let mut args = params.iter();

        // first arg is the name
        let Some(name) = args.next() else {
            return Err(Error::new(params.span(), "expected an argument"));
        };

        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = name else {
            return Err(Error::new(name.span(), "expected literal string for first argument"));
        };

        let name = s.value();
        builder = builder.name(name);

        // next is schema, which will use a "mimic" macro; `schema!(...)`
        // or a "path" which might be a string, int, etc
        let Some(schema) = args.next() else {
            return Err(Error::new(params.span(), "expected 2nd argument"));
        };

        match schema {
            Expr::Path(ExprPath { path, .. }) => {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(path.span(), "expected single identifier as the Path"));
                };

                let Some(schema) = ident_to_type(ident) else {
                    return Err(Error::new(ident.span(), "expected 'string', 'datetime', 'int32', 'int64', 'binary', 'snowflake', or 'name' as the identifier"));
                };

                builder = builder.schema(Some(RefOr::T(schema)));
            }

            Expr::Macro(ExprMacro {
                mac: Macro { tokens, path, .. },
                ..
            }) => {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(path.span(), "expected single identifier"));
                };

                let ident = ident.to_string();
                if ident.as_str() != "schema" {
                    return Err(Error::new(path.span(), "expected macro name to be 'schema'"));
                }

                let tt = tokens.clone();
                let s = match syn::parse::<Lit>(tt.into()) {
                    Ok(Lit::Str(s)) => s,
                    Ok(lit) => {
                        return Err(Error::new(
                            lit.span(),
                            "failed to parse literal string from macro invocation",
                        ))
                    }

                    Err(e) => {
                        return Err(Error::new(
                            e.span(),
                            "failed to parse literal string from macro invocation",
                        ))
                    }
                };

                let value = s.value();
                builder = builder.schema(Some(RefOr::Ref(Ref::from_schema_name(value))));
            }

            expr => return Err(Error::new(expr.span(), "expected macro invocation or path argument")),
        }

        // parse other arguments, which will be expression assignment,
        // i.e, "description = <...>"
        for expr in args {
            if let Expr::Path(ExprPath { path, .. }) = expr {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(path.span(), "expected path with a single identifier"));
                };

                let ident = ident.to_string();
                match ident.as_str() {
                    "deprecated" => {
                        builder = builder.deprecated(Some(utoipa::openapi::Deprecated::True));
                    }

                    ident => return Err(Error::new(ident.span(), "expected 'required' only, received {ident}")),
                }
            }

            let Expr::Assign(ExprAssign { left, right, .. }) = expr else {
                return Err(Error::new(
                    expr.span(),
                    "expected expr assignment, i.e 'description = \"hi\"'",
                ));
            };

            let left = *left.clone();
            let Expr::Path(ExprPath { path, .. }) = &left else {
                return Err(Error::new(
                    left.span(),
                    "expected left hand assignment to be a path with a single identifier",
                ));
            };

            let Some(ident) = path.get_ident() else {
                return Err(Error::new(
                    path.span(),
                    "expected left hand assignment to be a path with a single identifier",
                ));
            };

            let ident = ident.to_string();
            match ident.as_str() {
                "description" => {
                    let right = *right.clone();
                    let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &right else {
                        return Err(Error::new(
                            right.span(),
                            "expected right hand assignment to be a literal string",
                        ));
                    };

                    let desc = s.value();
                    builder = builder.description(Some(desc));
                }

                ident => {
                    return Err(Error::new(
                        ident.span(),
                        "expected 'description' only, received {ident}",
                    ))
                }
            }
        }

        if stream.peek(Token![,]) {
            stream.parse::<Token![,]>()?;
        }

        Ok(builder.build().into())
    }

    pub(crate) fn parse_response(input: ParseStream) -> Result<(u16, Response)> {
        input.parse::<kw::response>()?;

        let mut builder = ResponseBuilder::new();
        let buf;
        parenthesized!(buf in input);

        let params = buf.parse_terminated(Expr::parse, Token![,])?;
        let mut args = params.iter();

        let Some(first) = args.next() else {
            return Err(Error::new(params.span(), "missing status code argument"));
        };

        let Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) = first else {
            return Err(Error::new(first.span(), "expected literal int for status code"));
        };

        let status = i.base10_parse::<u16>()?;
        let Some(second) = args.next() else {
            return Err(Error::new(params.span(), "missing description argument"));
        };

        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = second else {
            return Err(Error::new(second.span(), "expected literal str for description"));
        };

        let desc = s.value();
        builder = builder.description(desc);

        let Some(third) = args.next() else {
            return Err(Error::new(params.span(), "missing contentType argument"));
        };

        let Expr::Tuple(ExprTuple { elems, .. }) = third else {
            return Err(Error::new(third.span(), "expected Paren expression"));
        };

        if elems.len() != 2 {
            return Err(Error::new(
                elems.span(),
                format!("expected 2 arguments, received {} arguments", elems.len()),
            ));
        }

        let mut items = elems.iter();
        let (ty, arg) = (items.next(), items.next());
        if ty.is_none() {
            return Err(Error::new(elems.span(), "missing content type to use"));
        }

        if arg.is_none() {
            return Err(Error::new(elems.span(), "missing response/schema type to use"));
        }

        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = ty.unwrap() else {
            return Err(Error::new(first.span(), "expected literal str for content type"));
        };

        let content_type = s.value();
        let mut content = ContentBuilder::new();
        match arg.unwrap() {
            Expr::Path(ExprPath { path, .. }) => {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(path.span(), "expected single identifier for Path"));
                };

                let Some(schema) = ident_to_type(ident) else {
                    return Err(Error::new(ident.span(), format!("expected 'string', 'datetime', 'int32', 'int64', 'binary', 'snowflake', or 'name' as the identifier, received {ident}")));
                };

                content = content.schema(RefOr::T(schema));
            }

            Expr::Macro(ExprMacro {
                mac: Macro { tokens, path, .. },
                ..
            }) => {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(
                        path.span(),
                        "expected single identifier in macro invocation",
                    ));
                };

                match ident.to_string().as_str() {
                    "response" => {
                        let tt = tokens.clone();
                        let s = match syn::parse::<Lit>(tt.into()) {
                            Ok(Lit::Str(s)) => s,
                            Ok(lit) => {
                                return Err(Error::new(
                                    lit.span(),
                                    "failed to parse literal string from macro invocation",
                                ))
                            }

                            Err(e) => {
                                return Err(Error::new(
                                    e.span(),
                                    "failed to parse literal string from macro invocation",
                                ))
                            }
                        };

                        content = content.schema(RefOr::Ref(Ref::from_response_name(s.value())));
                    }

                    "schema" => {
                        let tt = tokens.clone();
                        let s = match syn::parse::<Lit>(tt.into()) {
                            Ok(Lit::Str(s)) => s,
                            Ok(lit) => {
                                return Err(Error::new(
                                    lit.span(),
                                    "failed to parse literal string from macro invocation",
                                ))
                            }

                            Err(e) => {
                                return Err(Error::new(
                                    e.span(),
                                    "failed to parse literal string from macro invocation",
                                ))
                            }
                        };

                        content = content.schema(RefOr::Ref(Ref::from_schema_name(s.value())));
                    }

                    i => {
                        return Err(Error::new(
                            ident.span(),
                            format!("unexpected macro {i}, only wanted [response or schema]"),
                        ))
                    }
                }
            }

            expr => {
                return Err(Error::new(
                    expr.span(),
                    "expected Path or Macro expressions, received {expr}",
                ))
            }
        }

        builder = builder.content(content_type, content.build());
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok((status, builder.build().into()))
    }

    pub(crate) fn parse_description(&mut self, input: ParseStream) -> Result<()> {
        input.parse::<kw::description>()?;
        input.parse::<Token![=]>()?;

        let lit = input.parse::<Lit>()?;
        match lit {
            Lit::Str(s) => {
                self.id = s.value();
            }

            _ => return Err(Error::new(lit.span(), "expected literal string")),
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(())
    }

    pub(crate) fn parse_tags(&mut self, input: ParseStream) -> Result<()> {
        input.parse::<kw::tags>()?;

        let c;
        parenthesized!(c in input);

        let mut tags = vec![];

        // #puntporg real
        let punt = c.parse_terminated(Expr::parse, Token![,])?;
        for expr in punt.iter() {
            match expr {
                Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => tags.push(s.value()),
                _ => return Err(Error::new(expr.span(), "expected string literal")),
            }
        }

        self.tags = tags;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(())
    }

    pub(crate) fn parse_operation_id(&mut self, input: ParseStream) -> Result<()> {
        input.parse::<kw::id>()?;
        input.parse::<Token![=]>()?;

        let lit = input.parse::<Lit>()?;
        match lit {
            Lit::Str(s) => {
                self.id = s.value();
            }

            _ => return Err(Error::new(lit.span(), "expected literal string")),
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(())
    }

    pub(crate) fn parse_query_parameter(stream: ParseStream) -> Result<Parameter> {
        stream.parse::<kw::queryParameter>()?;

        let mut builder = ParameterBuilder::new()
            .parameter_in(ParameterIn::Query)
            .required(utoipa::openapi::Required::False);

        let c;
        parenthesized!(c in stream);

        let params = c.parse_terminated(Expr::parse, Token![,])?;
        let mut args = params.iter();

        // first arg is the name
        let Some(name) = args.next() else {
            return Err(Error::new(params.span(), "expected an argument"));
        };

        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = name else {
            return Err(Error::new(name.span(), "expected literal string for first argument"));
        };

        let name = s.value();
        builder = builder.name(name);

        let Some(schema) = args.next() else {
            return Err(Error::new(params.span(), "expected 2nd argument"));
        };

        match schema {
            Expr::Path(ExprPath { path, .. }) => {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(path.span(), "expected single identifier as the Path"));
                };

                let Some(schema) = ident_to_type(ident) else {
                    return Err(Error::new(ident.span(), "expected 'string', 'datetime', 'int32', 'int64', 'binary', 'snowflake', or 'name' as the identifier"));
                };

                builder = builder.schema(Some(RefOr::T(schema)));
            }

            Expr::Macro(ExprMacro {
                mac: Macro { tokens, path, .. },
                ..
            }) => {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(path.span(), "expected single identifier"));
                };

                let ident = ident.to_string();
                if ident.as_str() != "schema" {
                    return Err(Error::new(path.span(), "expected macro name to be 'schema'"));
                }

                let tt = tokens.clone();
                let s = match syn::parse::<Lit>(tt.into()) {
                    Ok(Lit::Str(s)) => s,
                    Ok(lit) => {
                        return Err(Error::new(
                            lit.span(),
                            "failed to parse literal string from macro invocation",
                        ))
                    }

                    Err(e) => {
                        return Err(Error::new(
                            e.span(),
                            "failed to parse literal string from macro invocation",
                        ))
                    }
                };

                let value = s.value();
                builder = builder.schema(Some(RefOr::Ref(Ref::from_schema_name(value))));
            }

            expr => return Err(Error::new(expr.span(), "expected macro invocation or path argument")),
        }

        // parse other arguments, which will be expression assignment,
        // i.e, "description = <...>"
        for expr in args {
            if let Expr::Path(ExprPath { path, .. }) = expr {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(path.span(), "expected path with a single identifier"));
                };

                let ident = ident.to_string();
                match ident.as_str() {
                    "deprecated" => {
                        builder = builder.deprecated(Some(utoipa::openapi::Deprecated::True));
                    }

                    "required" => {
                        builder = builder.required(utoipa::openapi::Required::True);
                    }

                    ident => {
                        return Err(Error::new(
                            ident.span(),
                            "expected [deprecated or required], received {ident}",
                        ))
                    }
                }
            }

            let Expr::Assign(ExprAssign { left, right, .. }) = expr else {
                return Err(Error::new(
                    expr.span(),
                    "expected expr assignment, i.e 'description = \"hi\"'",
                ));
            };

            let left = *left.clone();
            let Expr::Path(ExprPath { path, .. }) = &left else {
                return Err(Error::new(
                    left.span(),
                    "expected left hand assignment to be a path with a single identifier",
                ));
            };

            let Some(ident) = path.get_ident() else {
                return Err(Error::new(
                    path.span(),
                    "expected left hand assignment to be a path with a single identifier",
                ));
            };

            let ident = ident.to_string();
            match ident.as_str() {
                "description" => {
                    let right = *right.clone();
                    let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = &right else {
                        return Err(Error::new(
                            right.span(),
                            "expected right hand assignment to be a literal string",
                        ));
                    };

                    let desc = s.value();
                    builder = builder.description(Some(desc));
                }

                ident => {
                    return Err(Error::new(
                        ident.span(),
                        "expected 'description' only, received {ident}",
                    ))
                }
            }
        }

        if stream.peek(Token![,]) {
            stream.parse::<Token![,]>()?;
        }

        Ok(builder.build().into())
    }

    pub(crate) fn parse_request_body(input: ParseStream) -> Result<RequestBody> {
        input.parse::<kw::requestBody>()?;

        let mut builder = RequestBodyBuilder::new();
        let buf;
        parenthesized!(buf in input);

        let params = buf.parse_terminated(Expr::parse, Token![,])?;
        let mut args = params.iter();

        let (desc_expr, content_tuple_expr) = (args.next(), args.next());
        if desc_expr.is_none() {
            return Err(Error::new(params.span(), "missing description argument"));
        }

        let desc_expr = desc_expr.unwrap();
        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = desc_expr else {
            return Err(Error::new(desc_expr.span(), "expected literal string for description"));
        };

        if content_tuple_expr.is_none() {
            return Err(Error::new(params.span(), "missing content type tuple argument"));
        }

        builder = builder.description(Some(s.value()));

        let content_tuple_expr = content_tuple_expr.unwrap();
        let Expr::Tuple(ExprTuple { elems, .. }) = content_tuple_expr else {
            return Err(Error::new(content_tuple_expr.span(), "expected tuple argument"));
        };

        if elems.len() != 2 {
            return Err(Error::new(
                elems.span(),
                format!("expected 2 arguments, received {} arguments", elems.len()),
            ));
        }

        let mut items = elems.iter();
        let (ty, arg) = (items.next(), items.next());
        if ty.is_none() {
            return Err(Error::new(elems.span(), "missing content type to use"));
        }

        if arg.is_none() {
            return Err(Error::new(elems.span(), "missing response/schema type to use"));
        }

        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = ty.unwrap() else {
            return Err(Error::new(elems.span(), "expected literal str for content type"));
        };

        let content_type = s.value();
        let mut content = ContentBuilder::new();
        match arg.unwrap() {
            Expr::Path(ExprPath { path, .. }) => {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(path.span(), "expected single identifier for Path"));
                };

                let Some(schema) = ident_to_type(ident) else {
                    return Err(Error::new(ident.span(), format!("expected 'string', 'datetime', 'int32', 'int64', 'binary', 'snowflake', or 'name' as the identifier, received {ident}")));
                };

                content = content.schema(RefOr::T(schema));
            }

            Expr::Macro(ExprMacro {
                mac: Macro { tokens, path, .. },
                ..
            }) => {
                let Some(ident) = path.get_ident() else {
                    return Err(Error::new(
                        path.span(),
                        "expected single identifier in macro invocation",
                    ));
                };

                match ident.to_string().as_str() {
                    "response" => {
                        let tt = tokens.clone();
                        let s = match syn::parse::<Lit>(tt.into()) {
                            Ok(Lit::Str(s)) => s,
                            Ok(lit) => {
                                return Err(Error::new(
                                    lit.span(),
                                    "failed to parse literal string from macro invocation",
                                ))
                            }

                            Err(e) => {
                                return Err(Error::new(
                                    e.span(),
                                    "failed to parse literal string from macro invocation",
                                ))
                            }
                        };

                        content = content.schema(RefOr::Ref(Ref::from_response_name(s.value())));
                    }

                    "schema" => {
                        let tt = tokens.clone();
                        let s = match syn::parse::<Lit>(tt.into()) {
                            Ok(Lit::Str(s)) => s,
                            Ok(lit) => {
                                return Err(Error::new(
                                    lit.span(),
                                    "failed to parse literal string from macro invocation",
                                ))
                            }

                            Err(e) => {
                                return Err(Error::new(
                                    e.span(),
                                    "failed to parse literal string from macro invocation",
                                ))
                            }
                        };

                        content = content.schema(RefOr::Ref(Ref::from_schema_name(s.value())));
                    }

                    i => {
                        return Err(Error::new(
                            ident.span(),
                            format!("unexpected macro {i}, only wanted [response or schema]"),
                        ))
                    }
                }
            }

            expr => {
                return Err(Error::new(
                    expr.span(),
                    "expected Path or Macro expressions, received {expr}",
                ))
            }
        }

        builder = builder.content(content_type, content.build());
        if args.next().is_some() {
            return Err(Error::new(
                params.span(),
                "unexpected nth argument after parsing both description and content type tuple.",
            ));
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(builder.build().into())
    }
}

#[allow(dead_code)] // it is used, but clippy is being mean :Nod:
fn ident_to_type(ident: &Ident) -> Option<Schema> {
    match ident.to_string().as_str() {
        "string" => Some(Schema::Object(
            ObjectBuilder::new().schema_type(SchemaType::String).build(),
        )),

        "datetime" => Some(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
                .build(),
        )),

        "int32" => Some(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Integer)
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int32)))
                .build(),
        )),

        "int64" => Some(Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Integer)
                .format(Some(SchemaFormat::KnownFormat(KnownFormat::Int64)))
                .build(),
        )),

        "snowflake" => {
            let (_, r) = ID::schema();
            match r {
                RefOr::T(schema) => Some(schema),
                _ => unreachable!(),
            }
        }

        "name" => {
            let (_, r) = Name::schema();
            match r {
                RefOr::T(schema) => Some(schema),
                _ => unreachable!(),
            }
        }

        _ => None,
    }
}
