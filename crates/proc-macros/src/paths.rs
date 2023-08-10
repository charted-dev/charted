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

use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Block, Error, Expr, ExprArray, ExprBlock, ExprLit, ExprPath, Lit, Result, Stmt,
};
use utoipa::openapi::{
    path::{Operation, OperationBuilder, Parameter, ParameterBuilder, ParameterIn, ParameterStyle},
    request_body::{RequestBody, RequestBodyBuilder},
    ContentBuilder, Deprecated, PathItemType, Ref, RefOr, Required, Response, ResponseBuilder,
};

macro_rules! validate_args {
    // shortcut for validate_args!(args, span: args.clone(), len: <int>);
    ($args:expr, len: $expected:expr) => {
        validate_args!($args, span: $args.span(), len: $expected);
    };

    ($args:expr, optional: true, max: $max:expr) => {
        validate_args!($args, span: $args.span(), optional: true, max: $max);
    };

    ($args:expr, span: $span:expr, optional: true, max: $max:expr) => {
        if $args.len() > $max {
            return Err(::syn::Error::new(
                $span,
                format!("expected {} argument(s), received {} arguments", $expected, $args.len()),
            ));
        }
    };

    ($args:expr, span: $span:expr, len: $expected:expr) => {
        if $args.is_empty() {
            return Err(::syn::Error::new(
                $span,
                format!("expected {} argument(s), received none", $expected),
            ));
        }

        if $args.len() < $expected {
            return Err(::syn::Error::new(
                $span,
                format!("required {} argument(s), received {} arguments", $expected, $args.len()),
            ));
        }

        if $args.len() > $expected {
            return Err(::syn::Error::new(
                $span,
                format!("expected {} argument(s), received {} arguments", $expected, $args.len()),
            ));
        }
    };
}

use validate_args;

pub(crate) struct Args {
    pub operations: HashMap<PathItemType, Operation>,
}

impl Args {
    /// Parses a [`Operation`] object out of a [`Block`]. This is mainly for
    /// the top-level `paths!` macro.
    pub(crate) fn parse_operation(block: Block) -> Result<Operation> {
        let mut builder = OperationBuilder::new();
        if block.stmts.is_empty() {
            return Err(Error::new(block.span(), "missing one or more statements"));
        }

        for stmt in block.stmts.clone().iter() {
            let Stmt::Expr(expr, Some(_)) = stmt else {
                return Err(Error::new(stmt.span(), "expected expression with semicolon at end"));
            };

            let Expr::Call(call) = expr else {
                return Err(Error::new(expr.span(), "expected a function call expression"));
            };

            let Expr::Path(ExprPath { path, .. }) = *call.func.clone() else {
                return Err(Error::new(call.span(), "expected path after ExprCall"));
            };

            for ident in path.segments.iter().map(|f| f.ident.to_string()) {
                match ident.as_str() {
                    "description" => {
                        let args = call.args.clone();
                        validate_args!(args, span: args.span(), len: 1);

                        let desc = args.first().unwrap();
                        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = desc else {
                            return Err(Error::new(
                                desc.span(),
                                "required literal string to be the first argument.",
                            ));
                        };

                        let text = s.token().to_string();
                        builder = builder.description(Some(text.replace('\"', "")));
                    }

                    "operationId" => {
                        let args = call.args.clone();
                        validate_args!(args, span: args.span(), len: 1);

                        let id = args.first().unwrap();
                        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = id else {
                            return Err(Error::new(
                                id.span(),
                                "required literal string to be the first argument.",
                            ));
                        };

                        let text = s.token().to_string();
                        builder = builder.operation_id(Some(text.replace('\"', "")))
                    }

                    "tags" => {
                        let args = call.args.clone();
                        validate_args!(args, span: args.span(), len: 1);

                        let arr = args.first().unwrap();
                        let Expr::Array(ExprArray { elems, .. }) = arr else {
                            return Err(Error::new(
                                arr.span(),
                                "required literal slice `[a, b, c]` to be the first argument.",
                            ));
                        };

                        let mut items: Vec<String> = vec![];
                        for item in elems.iter() {
                            let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = item else {
                                return Err(Error::new(item.span(), "expected literal string to be here"));
                            };

                            let text = s.token().to_string();
                            items.push(text.replace('\"', ""));
                        }

                        builder = builder.tags(Some(items));
                    }

                    "deprecated" => {
                        builder = builder.deprecated(Some(Deprecated::True));
                    }

                    "response" => {
                        let args = call.args.clone();
                        validate_args!(args, len: 3);

                        // turn it into an array so we can select the elements we
                        // want.
                        let iter = args.iter().collect::<Vec<_>>();
                        let first = iter.first().unwrap();

                        let Expr::Lit(ExprLit {
                            lit: Lit::Int(code), ..
                        }) = first
                        else {
                            return Err(Error::new(first.span(), "expected int literal"));
                        };

                        let second = iter.get(1).unwrap();
                        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = second else {
                            return Err(Error::new(second.span(), "expected literal string to be here"));
                        };

                        let third = iter.get(2).unwrap();
                        let Expr::Block(ExprBlock { block, .. }) = third else {
                            return Err(Error::new(third.span(), "missing third argument"));
                        };

                        let status_str = code.token().to_string();
                        let content_type_str = s.token().to_string().replace('\"', "");
                        let _ = status_str
                            .parse::<u16>()
                            .map_err(|_| Error::new(code.span(), "expected u16 variant, but couldn't parse it?!"))?;

                        builder =
                            builder.response(status_str, Args::process_response(block.clone(), content_type_str)?);
                    }

                    "requestBody" => {
                        let args = call.args.clone();
                        validate_args!(args, len: 2);

                        let Some(Expr::Lit(ExprLit { lit: Lit::Str(s), .. })) = args.first() else {
                            return Err(Error::new(args.span(), "expected literal string as first argument"));
                        };

                        let iter = args.iter().collect::<Vec<_>>();
                        let Some(Expr::Block(ExprBlock { block, .. })) = iter.get(1) else {
                            return Err(Error::new(args.span(), "expected block as second argument"));
                        };

                        let content_type = s.token().to_string().replace('\"', "");
                        let body = Args::process_request_body(block.clone(), content_type)?;
                        builder = builder.request_body(Some(body));
                    }

                    "queryParameter" => {
                        let args = call.args.clone();
                        validate_args!(args, len: 2);

                        let Some(Expr::Lit(ExprLit { lit: Lit::Str(s), .. })) = args.first() else {
                            return Err(Error::new(args.span(), "expected literal string as first argument"));
                        };

                        let iter = args.iter().collect::<Vec<_>>();
                        let Some(Expr::Block(ExprBlock { block, .. })) = iter.get(1) else {
                            return Err(Error::new(args.span(), "expected block as second argument"));
                        };

                        let name = s.token().to_string().replace('\"', "");
                        let param =
                            Args::process_parameter(block.clone(), name, ParameterIn::Query, ParameterStyle::Form)?;

                        builder = builder.parameter(param);
                    }

                    "pathParameter" => {
                        let args = call.args.clone();
                        validate_args!(args, len: 2);

                        let Some(Expr::Lit(ExprLit { lit: Lit::Str(s), .. })) = args.first() else {
                            return Err(Error::new(args.span(), "expected literal string as first argument"));
                        };

                        let iter = args.iter().collect::<Vec<_>>();
                        let Some(Expr::Block(ExprBlock { block, .. })) = iter.get(1) else {
                            return Err(Error::new(args.span(), "expected block as second argument"));
                        };

                        let name = s.token().to_string().replace('\"', "");
                        let param =
                            Args::process_parameter(block.clone(), name, ParameterIn::Path, ParameterStyle::Simple)?;

                        builder = builder.parameter(param);
                    }

                    ident => return Err(Error::new(ident.span(), format!("unknown function {ident}"))),
                }
            }
        }

        Ok(builder.build())
    }

    /// Processes a [`RequestBody`] object from a Rust [`Block`]. This is mainly for the
    /// `requestBody` function in a top-level `paths!` macro declaration.
    ///
    /// ```no_run
    /// # use charted_proc_macros::*;
    /// #
    /// paths! {
    ///     Get {
    ///         description("Some description");
    ///         operationId("id");
    ///         requestBody("application/json", {
    ///             description("A description");
    ///             schema(schema!("CreateUserPayload"));
    ///             required(); // marks this as required
    ///         });
    ///     }
    /// }
    /// ```
    pub(crate) fn process_request_body(block: Block, content_type: String) -> Result<RequestBody> {
        let mut body = RequestBodyBuilder::new().required(Some(Required::False));
        if block.stmts.is_empty() {
            return Err(Error::new(block.span(), "missing one or more statements"));
        }

        for stmt in block.stmts.clone().iter() {
            let Stmt::Expr(expr, Some(_)) = stmt else {
                return Err(Error::new(stmt.span(), "expected expression with semicolon at end"));
            };

            let Expr::Call(call) = expr else {
                return Err(Error::new(expr.span(), "expected a function call expression"));
            };

            let Expr::Path(ExprPath { path, .. }) = *call.func.clone() else {
                return Err(Error::new(call.span(), "expected path after ExprCall"));
            };

            while let Some(ident) = path.segments.iter().map(|f| f.ident.to_string()).next() {
                match ident.as_str() {
                    "description" => {
                        let args = call.args.clone();
                        validate_args!(args, len: 1);

                        let desc = args.first().unwrap();
                        let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = desc else {
                            return Err(Error::new(
                                desc.span(),
                                "required literal string to be the first argument.",
                            ));
                        };

                        let text = s.token().to_string();
                        body = body.description(Some(text.replace('\"', "")));
                    }

                    "required" => {
                        body = body.required(Some(Required::True));
                    }

                    "schema" => {
                        let args = call.args.clone();
                        validate_args!(args, len: 1);

                        let expr = args.first().unwrap();
                        let mut content = ContentBuilder::new();

                        match expr.clone() {
                            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => {
                                let schema = s.token().to_string();
                                content = content.schema(RefOr::Ref(Ref::from_response_name(schema.replace('\"', ""))));
                            }

                            Expr::Path(ExprPath { path, .. }) => {
                                dbg!(path);
                            }

                            _ => {
                                return Err(Error::new(
                                    expr.span(),
                                    "unexpected expression, wanted Path or string literal",
                                ))
                            }
                        }

                        body = body.content(content_type.clone(), content.build());
                    }

                    ident => {
                        return Err(Error::new(
                            ident.span(),
                            format!("unknown ident in response block {ident}"),
                        ));
                    }
                }
            }
        }

        Ok(body.build())
    }

    /// Processes a [`Parameter`] object from a Rust [`Block`] declaration. This is mainly
    /// for the `pathParameter` and `queryParameter` function in the `paths!` macro.
    ///
    /// ```no_run
    /// # use charted_proc_macros::*;
    /// #
    /// paths! {
    ///     Get {
    ///         description("Some description");
    ///         operationId("id");
    ///         pathParameter("idOrName", {
    ///             description("Parameter description");
    ///             deprecated(); // marks this as deprecated
    ///         });
    ///     }
    /// }
    /// ```
    pub(crate) fn process_parameter(
        block: Block,
        name: String,
        param_in: ParameterIn,
        style: ParameterStyle,
    ) -> Result<Parameter> {
        let param = ParameterBuilder::new()
            .name(name)
            .parameter_in(param_in)
            .style(Some(style));

        if block.stmts.is_empty() {
            return Err(Error::new(block.span(), "missing one or more statements"));
        }

        for stmt in block.stmts.clone().iter() {
            let Stmt::Expr(expr, Some(_)) = stmt else {
                return Err(Error::new(stmt.span(), "expected expression with semicolon at end"));
            };

            let Expr::Call(call) = expr else {
                return Err(Error::new(expr.span(), "expected a function call expression"));
            };

            let Expr::Path(ExprPath { path, .. }) = *call.func.clone() else {
                return Err(Error::new(call.span(), "expected path after ExprCall"));
            };

            while let Some(ident) = path.segments.iter().map(|f| f.ident.to_string()).next() {
                match ident.as_str() {
                    "description" => {}
                    "deprecated" => {}
                    "schema" => {}

                    ident => {
                        return Err(Error::new(
                            ident.span(),
                            format!("unknown ident in response block {ident}"),
                        ));
                    }
                }
            }
        }

        Ok(param.build())
    }

    pub(crate) fn process_response(block: Block, _content_type: String) -> Result<Response> {
        let res = ResponseBuilder::new();
        if block.stmts.is_empty() {
            return Err(Error::new(block.span(), "missing one or more statements"));
        }

        for stmt in block.stmts.clone().iter() {
            let Stmt::Expr(expr, Some(_)) = stmt else {
                return Err(Error::new(stmt.span(), "expected expression with semicolon at end"));
            };

            let Expr::Call(call) = expr else {
                return Err(Error::new(expr.span(), "expected a function call expression"));
            };

            let Expr::Path(ExprPath { path, .. }) = *call.func.clone() else {
                return Err(Error::new(call.span(), "expected path after ExprCall"));
            };

            while let Some(ident) = path.segments.iter().map(|f| f.ident.to_string()).next() {
                match ident.as_str() {
                    "description" => {}
                    "schema" => {}

                    ident => {
                        return Err(Error::new(
                            ident.span(),
                            format!("unknown ident in response block {ident}"),
                        ));
                    }
                }
            }
        }

        Ok(res.build())
    }
}

mod kw {
    pub mod fns {
        syn::custom_keyword!(securityScheme);
        syn::custom_keyword!(operationId);
        syn::custom_keyword!(description);
        syn::custom_keyword!(requestBody);
        syn::custom_keyword!(deprecated);
        syn::custom_keyword!(parameter);
        syn::custom_keyword!(response);
        syn::custom_keyword!(schema);
    }

    pub mod verbs {
        syn::custom_keyword!(Delete);
        syn::custom_keyword!(Patch);
        syn::custom_keyword!(Post);
        syn::custom_keyword!(Put);
        syn::custom_keyword!(Get);
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut operations = HashMap::new();
        if input.peek(kw::verbs::Get) {
            input.parse::<kw::verbs::Get>()?;

            let operation = Args::parse_operation(input.parse()?)?;
            operations.insert(PathItemType::Get, operation);
        }

        Ok(Args { operations })
    }
}
