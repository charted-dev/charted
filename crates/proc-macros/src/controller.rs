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
use proc_macro2::Ident;
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
        ContentBuilder, KnownFormat, ObjectBuilder, Ref, RefOr, ResponseBuilder, Schema, SchemaFormat, SchemaType,
    },
    ToSchema,
};

#[derive(Clone, Default)]
pub struct Args {
    pub id: String,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub responses: HashMap<u16, Response>,
    pub request_body: Option<RequestBody>,
    pub parameters: HashMap<String, Parameter>,
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
    syn::custom_keyword!(tags);
    syn::custom_keyword!(id);
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = Args::default();

        // parse "id = <string literal>"
        if input.peek(kw::id) {
            input.parse::<kw::id>()?;
            input.parse::<Token![=]>()?;

            let lit = input.parse::<Lit>()?;
            match lit {
                Lit::Str(s) => {
                    args.id = s.token().to_string().replace('\"', "");
                }

                _ => return Err(Error::new(lit.span(), "expected literal string")),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        // parse "tags(..., ...)"
        if input.peek(kw::tags) {
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

            args.tags = tags;
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        // parse "description = <string literal>"
        if input.peek(kw::description) {
            input.parse::<kw::description>()?;
            input.parse::<Token![=]>()?;

            let lit = input.parse::<Lit>()?;
            match lit {
                Lit::Str(s) => {
                    args.id = s.token().to_string().replace('\"', "");
                }

                _ => return Err(Error::new(lit.span(), "expected literal string")),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        // parse "response(200, "description", ("content type", "what schema?!"))"
        if input.peek(kw::response) {
            input.parse::<kw::response>()?;
            let c;
            parenthesized!(c in input);

            let params = c.parse_terminated(Expr::parse, Token![,])?;
            let mut iter = params.iter();

            let Some(first) = iter.next() else {
                return Err(Error::new(params.span(), "missing status code"));
            };

            let Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) = first else {
                return Err(Error::new(first.span(), "expected literal int for status code"));
            };

            let status = i.base10_parse::<u16>()?;
            let Some(second) = iter.next() else {
                return Err(Error::new(params.span(), "missing description"));
            };

            let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = second else {
                return Err(Error::new(first.span(), "expected literal str for description"));
            };

            let desc = s.value();
            let Some(third) = iter.next() else {
                return Err(Error::new(params.span(), "missing paren tuple"));
            };

            let Expr::Tuple(ExprTuple { elems, .. }) = third else {
                return Err(Error::new(third.span(), "expected Paren expression"));
            };

            let mut elems = elems.iter();
            let Some(content_expr) = elems.next() else {
                return Err(Error::new(params.span(), "missing cntent type"));
            };

            let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = content_expr else {
                return Err(Error::new(first.span(), "expected literal str for content type"));
            };

            let content_type = s.value();
            let Some(response) = elems.next() else {
                return Err(Error::new(third.span(), "expected response"));
            };

            let mut res = ResponseBuilder::new().description(desc);
            match response {
                Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => {
                    let resp = Ref::from_response_name(s.value());
                    res = res.content(content_type, ContentBuilder::new().schema(RefOr::Ref(resp)).build());
                }

                Expr::Macro(ExprMacro {
                    mac: Macro { tokens, path, .. },
                    ..
                }) => {
                    let value = path.segments.first().unwrap();
                    let ident = value.ident.clone().to_string().replace('\"', "");
                    if ident.as_str() != "response" {
                        return Err(Error::new(value.ident.span(), "expected 'response' as macro name"));
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

                    let resp = Ref::from_response_name(s.value());
                    res = res.content(content_type, ContentBuilder::new().schema(RefOr::Ref(resp)).build());
                }

                expr => {
                    return Err(Error::new(expr.span(), "expected literal string or response!() macro"));
                }
            }

            let res = res.build();
            args.responses.insert(status, res.into());

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if input.peek(kw::deprecated) {
            input.parse::<kw::deprecated>()?;

            // parses "= <why>"
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;

                let expr = input.parse::<Expr>()?;
                let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = expr else {
                    return Err(Error::new(expr.span(), "expected literal string"));
                };

                let why = s.value();
                args.is_deprecated = Some(Some(why));
            } else {
                args.is_deprecated = Some(None);
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if input.peek(kw::pathParameter) {
            input.parse::<kw::pathParameter>()?;
            let param = Args::parse_path_parameter(input)?;
            let inner = param.0.clone(); // we only want the name that we populated here :Nod:

            args.parameters.insert(inner.name, param);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(args)
    }
}

impl Args {
    pub(crate) fn parse_path_parameter(stream: ParseStream) -> Result<Parameter> {
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
